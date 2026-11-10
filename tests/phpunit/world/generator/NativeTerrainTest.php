<?php

/*
 *
 *  ____            _        _   __  __ _                  __  __ ____
 * |  _ \ ___   ___| | _____| |_|  \/  (_)_ __   ___      |  \/  |  _ \
 * | |_) / _ \ / __| |/ / _ \ __| |\/| | | '_ \ / _ \_____| |\/| | |_) |
 * |  __/ (_) | (__|   <  __/ |_| |  | | | | | |  __/_____| |  | |  __/
 * |_|   \___/ \___|_|\_\___|\__|_|  |_|_|_| |_|\___|     |_|  |_|_|
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * @author PocketMine Team
 * @link http://www.pocketmine.net/
 *
 *
 */

declare(strict_types=1);

namespace pocketmine\world\generator;

use PHPUnit\Framework\TestCase;
use pocketmine\utils\Random;
use pocketmine\world\format\Chunk;
use pocketmine\world\format\SubChunk;
use pocketmine\world\generator\hell\Nether;
use pocketmine\world\generator\noise\Simplex;
use pocketmine\world\generator\normal\Normal;
use pocketmine\world\SimpleChunkManager;
use pocketmine\world\World;
use function array_fill;
use function array_keys;
use function count;
use function extension_loaded;

/**
 * Verifies that the pmmpnative extension produces exactly the same results as the PHP code it replaces.
 */
class NativeTerrainTest extends TestCase{

	protected function setUp() : void{
		if(!extension_loaded('pmmpnative')){
			self::markTestSkipped("pmmpnative extension is not loaded");
		}
	}

	/**
	 * Returns a copy of the given noise which always uses the PHP implementation, since native code is never used
	 * for subclasses of Simplex.
	 */
	private static function phpOnly(Simplex $noise) : Simplex{
		$copy = (new \ReflectionClass(PhpOnlySimplex::class))->newInstanceWithoutConstructor();
		foreach(["perm", "offsetX", "offsetY", "offsetZ", "octaves", "persistence", "expansion"] as $propertyName){
			$property = new \ReflectionProperty(Simplex::class, $propertyName);
			$property->setValue($copy, $property->getValue($noise));
		}
		self::assertNull($copy->getNativeParameters());
		return $copy;
	}

	/**
	 * @phpstan-return \Generator<string, array{int, float, float, int, int, int, int, int, int, int, int, int}, void, void>
	 */
	public static function fastNoise3DProvider() : \Generator{
		//octaves, persistence, expansion, xSize, ySize, zSize, xSamplingRate, ySamplingRate, zSamplingRate, x, y, z
		yield "overworld" => [4, 1 / 4, 1 / 32, 16, 64, 16, 4, 8, 4, 160, 40, -320];
		yield "nether" => [4, 1 / 4, 1 / 64, 16, 128, 16, 4, 8, 4, -16, 0, 0];
		yield "negative coordinates" => [3, 1 / 2, 1 / 8, 8, 16, 12, 2, 4, 3, -5000, -64, -70000];
		yield "far from origin" => [4, 1 / 4, 1 / 32, 16, 32, 16, 4, 8, 4, 29999984, 0, -29999984];
		yield "no interpolation" => [2, 1 / 16, 1 / 512, 4, 4, 4, 1, 1, 1, 7, 9, 11];
		yield "zero height" => [4, 1 / 4, 1 / 32, 16, 0, 16, 4, 8, 4, 0, 64, 0];
		yield "unsupported sizes fall back to PHP" => [4, 1 / 4, 1 / 32, 16, 16, 16, 4, 8, 4, 0, 0, 2 ** 60];
	}

	/**
	 * @dataProvider fastNoise3DProvider
	 */
	public function testFastNoise3DMatchesPhp(int $octaves, float $persistence, float $expansion, int $xSize, int $ySize, int $zSize, int $xSamplingRate, int $ySamplingRate, int $zSamplingRate, int $x, int $y, int $z) : void{
		$native = new Simplex(new Random(1337), $octaves, $persistence, $expansion);
		self::assertNotNull($native->getNativeParameters());
		$php = self::phpOnly($native);

		//floats must be bit-for-bit identical, otherwise terrain would differ depending on the extension being loaded
		self::assertSame(
			$php->getFastNoise3D($xSize, $ySize, $zSize, $xSamplingRate, $ySamplingRate, $zSamplingRate, $x, $y, $z),
			$native->getFastNoise3D($xSize, $ySize, $zSize, $xSamplingRate, $ySamplingRate, $zSamplingRate, $x, $y, $z)
		);
	}

	/**
	 * The comparisons below would pass vacuously if the native code always deferred to PHP.
	 */
	public function testNativeCodeHandlesOrdinaryInput() : void{
		$heights = static fn(float $height) : array => array_fill(0, Chunk::EDGE_LENGTH ** 2, $height);

		$overworld = NativeTerrain::normal(new Simplex(new Random(1337), 4, 1 / 4, 1 / 32), 0, 0, 40, 80, 62, $heights(45.0), $heights(74.0), 1, 2, 3);
		self::assertNotNull($overworld);
		self::assertSame([0, 1, 2, 3, 4], array_keys($overworld));

		$nether = NativeTerrain::nether(new Simplex(new Random(1337), 4, 1 / 4, 1 / 64), 0, 0, 32, 64, 1, 0.5, 1, 2, 3);
		self::assertNotNull($nether);
		self::assertContains(0, array_keys($nether));
		self::assertContains(7, array_keys($nether));
	}

	/**
	 * @phpstan-return \Generator<string, array{\Closure(int) : Generator, int, int, int}, void, void>
	 */
	public static function generatorProvider() : \Generator{
		$overworld = static fn(int $seed) : Generator => new Normal($seed, "");
		$nether = static fn(int $seed) : Generator => new Nether($seed, "");

		yield "overworld" => [$overworld, 1337, 0, 0];
		yield "overworld, other seed, negative coordinates" => [$overworld, -987654321, -40, -7];
		yield "overworld, far from origin" => [$overworld, 42, 1875000, -1875000];
		yield "nether" => [$nether, 1337, 0, 0];
		yield "nether, other seed, negative coordinates" => [$nether, -987654321, -40, -7];
	}

	/**
	 * @dataProvider generatorProvider
	 * @phpstan-param \Closure(int) : Generator $createGenerator
	 */
	public function testGeneratedChunksMatchPhp(\Closure $createGenerator, int $seed, int $startX, int $startZ) : void{
		$size = 3;

		$worlds = [];
		foreach([false, true] as $usePhp){
			$generator = $createGenerator($seed);
			if($usePhp){
				$noiseBase = new \ReflectionProperty($generator, "noiseBase");
				$noise = $noiseBase->getValue($generator);
				self::assertInstanceOf(Simplex::class, $noise);
				$noiseBase->setValue($generator, self::phpOnly($noise));
			}

			$world = new SimpleChunkManager(World::Y_MIN, World::Y_MAX);
			//populating a chunk requires its neighbours to be generated
			for($chunkX = $startX - 1; $chunkX <= $startX + $size; $chunkX++){
				for($chunkZ = $startZ - 1; $chunkZ <= $startZ + $size; $chunkZ++){
					$world->setChunk($chunkX, $chunkZ, new Chunk([], false));
					$generator->generateChunk($world, $chunkX, $chunkZ);
				}
			}
			for($chunkX = $startX; $chunkX < $startX + $size; $chunkX++){
				for($chunkZ = $startZ; $chunkZ < $startZ + $size; $chunkZ++){
					$generator->populateChunk($world, $chunkX, $chunkZ);
				}
			}
			$worlds[] = $world;
		}
		[$nativeWorld, $phpWorld] = $worlds;

		for($chunkX = $startX - 1; $chunkX <= $startX + $size; $chunkX++){
			for($chunkZ = $startZ - 1; $chunkZ <= $startZ + $size; $chunkZ++){
				$nativeChunk = $nativeWorld->getChunk($chunkX, $chunkZ);
				$phpChunk = $phpWorld->getChunk($chunkX, $chunkZ);
				self::assertNotNull($nativeChunk);
				self::assertNotNull($phpChunk);
				self::assertSame($phpChunk->getTerrainDirtyFlags(), $nativeChunk->getTerrainDirtyFlags());

				$phpSubChunks = $phpChunk->getSubChunks();
				foreach($nativeChunk->getSubChunks() as $subChunkY => $nativeSubChunk){
					self::assertSubChunksEqual($phpSubChunks[$subChunkY], $nativeSubChunk, "chunk $chunkX $chunkZ subchunk $subChunkY");
				}
			}
		}
	}

	private static function assertSubChunksEqual(SubChunk $expected, SubChunk $actual, string $where) : void{
		self::assertSame($expected->getEmptyBlockId(), $actual->getEmptyBlockId(), $where);
		self::assertLessThanOrEqual(1, count($expected->getBlockLayers()), $where);
		self::assertLessThanOrEqual(1, count($actual->getBlockLayers()), $where);

		$expectedBiomes = $expected->getBiomeArray();
		$actualBiomes = $actual->getBiomeArray();
		$mismatches = 0;
		for($x = 0; $x < SubChunk::EDGE_LENGTH; $x++){
			for($z = 0; $z < SubChunk::EDGE_LENGTH; $z++){
				for($y = 0; $y < SubChunk::EDGE_LENGTH; $y++){
					//comparing contents rather than serialized data, since the same blocks can be stored in different ways
					if(
						$expected->getBlockStateId($x, $y, $z) !== $actual->getBlockStateId($x, $y, $z) ||
						$expectedBiomes->get($x, $y, $z) !== $actualBiomes->get($x, $y, $z)
					){
						$mismatches++;
					}
				}
			}
		}
		self::assertSame(0, $mismatches, "$where has different blocks or biomes");
	}
}

class PhpOnlySimplex extends Simplex{

}
