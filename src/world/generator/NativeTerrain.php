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

use pocketmine\block\Block;
use pocketmine\world\format\PalettedBlockArray;
use pocketmine\world\generator\noise\Simplex;
use function is_int;

/**
 * Terrain shaping implemented by the optional pmmpnative extension (see the rust directory).
 *
 * Every method returns null if native code can't be used for the given input. Callers must then run their PHP
 * implementation, which is the reference for what the native code produces.
 *
 * @internal
 */
final class NativeTerrain{

	private function __construct(){
		//NOOP
	}

	/**
	 * @param float[] $minHeights indexed by x * 16 + z
	 * @param float[] $maxHeights indexed by x * 16 + z
	 * @phpstan-param list<float> $minHeights
	 * @phpstan-param list<float> $maxHeights
	 *
	 * @return PalettedBlockArray[]|null block layers indexed by subchunk Y, subchunks with no entry are empty
	 * @phpstan-return array<int, PalettedBlockArray>|null
	 */
	public static function normal(
		Simplex $noise,
		int $baseX,
		int $baseZ,
		int $noiseMin,
		int $noiseMax,
		int $waterHeight,
		array $minHeights,
		array $maxHeights,
		int $bedrock,
		int $stone,
		int $water
	) : ?array{
		$noiseParameters = $noise->getNativeParameters();
		if($noiseParameters === null){
			return null;
		}
		return self::decodeLayers(\pmmp_native_normal_terrain(
			...$noiseParameters,
			...[$baseX, $baseZ, $noiseMin, $noiseMax, $waterHeight, $minHeights, $maxHeights, Block::EMPTY_STATE_ID, $bedrock, $stone, $water]
		));
	}

	/**
	 * @return PalettedBlockArray[]|null block layers indexed by subchunk Y, subchunks with no entry are empty
	 * @phpstan-return array<int, PalettedBlockArray>|null
	 */
	public static function nether(
		Simplex $noise,
		int $baseX,
		int $baseZ,
		int $waterHeight,
		int $emptyHeight,
		int $emptyAmplitude,
		float $density,
		int $bedrock,
		int $netherrack,
		int $lava
	) : ?array{
		$noiseParameters = $noise->getNativeParameters();
		if($noiseParameters === null){
			return null;
		}
		return self::decodeLayers(\pmmp_native_nether_terrain(
			...$noiseParameters,
			...[$baseX, $baseZ, $waterHeight, $emptyHeight, $emptyAmplitude, $density, Block::EMPTY_STATE_ID, $bedrock, $netherrack, $lava]
		));
	}

	/**
	 * @param mixed[]|null $layers
	 * @phpstan-param array<int, int|array{int, string, list<int>}>|null $layers
	 *
	 * @return PalettedBlockArray[]|null
	 * @phpstan-return array<int, PalettedBlockArray>|null
	 */
	private static function decodeLayers(?array $layers) : ?array{
		if($layers === null){
			return null;
		}
		$result = [];
		foreach($layers as $subChunkY => $layer){
			$result[$subChunkY] = is_int($layer) ? new PalettedBlockArray($layer) : PalettedBlockArray::fromData($layer[0], $layer[1], $layer[2]);
		}
		return $result;
	}
}
