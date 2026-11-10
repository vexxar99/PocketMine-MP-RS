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
use pocketmine\block\BlockTypeIds;
use pocketmine\block\Liquid;
use pocketmine\block\RuntimeBlockStateRegistry;
use pocketmine\world\biome\BiomeRegistry;
use pocketmine\world\format\PalettedBlockArray;
use pocketmine\world\generator\biome\BiomeSelector;
use pocketmine\world\generator\noise\Simplex;
use function array_values;
use function is_int;

/**
 * Chunk generation implemented by the optional pmmpnative extension (see the rust directory).
 *
 * Every method returns null if native code can't be used for the given input. Callers must then run their PHP
 * implementation, which is the reference for what the native code produces.
 *
 * @internal
 * @phpstan-type NativeLayer int|array{int, string, list<int>}
 */
final class NativeTerrain{

	private function __construct(){
		//NOOP
	}

	/**
	 * Does everything Normal::generateChunk() does: biome selection, terrain shaping and ground cover.
	 *
	 * @return mixed[]|null biomes used by every subchunk, and block layers indexed by subchunk Y (subchunks with no
	 * entry are empty)
	 * @phpstan-return array{PalettedBlockArray, array<int, PalettedBlockArray>}|null
	 */
	public static function normal(
		Simplex $noise,
		BiomeSelector $selector,
		Gaussian $gaussian,
		int $seed,
		int $baseX,
		int $baseZ,
		int $waterHeight,
		Block $bedrock,
		Block $stone,
		Block $water
	) : ?array{
		$noiseParameters = $noise->getNativeParameters();
		$selectorParameters = $selector->getNativeParameters();
		if($noiseParameters === null || $selectorParameters === null){
			return null;
		}
		[$temperature, $rainfall, $biomeMap, $biomes] = $selectorParameters;

		//these can be changed by plugins at any time, so they have to be read for every chunk
		$biomeRegistry = BiomeRegistry::getInstance();
		$biomeProperties = [];
		foreach($biomes as $biomeId => $biome){
			if($biomeRegistry->getBiome($biomeId) !== $biome){
				//ground cover uses the registry while elevation uses the selector, and here they don't agree
				return null;
			}
			$groundCover = [];
			foreach($biome->getGroundCover() as $block){
				$groundCover[] = $block->getStateId();
				$groundCover[] = ($block->isSolid() ? 1 : 0) | ($block->canBeFlowedInto() ? 2 : 0);
			}
			$biomeProperties[$biomeId] = [$biome->getMinElevation(), $biome->getMaxElevation(), $groundCover];
		}

		$terrainBlocks = [];
		foreach([RuntimeBlockStateRegistry::getInstance()->fromStateId(Block::EMPTY_STATE_ID), $bedrock, $stone, $water] as $block){
			$terrainBlocks[] = $block->getStateId();
			$terrainBlocks[] = ($block->isTransparent() ? 1 : 0) | ($block->getTypeId() === BlockTypeIds::AIR ? 2 : 0) | ($block instanceof Liquid ? 4 : 0);
		}

		$result = \pmmp_native_normal_chunk(
			$noiseParameters,
			$temperature,
			$rainfall,
			$biomeMap,
			$biomeProperties,
			$seed,
			$baseX,
			$baseZ,
			$waterHeight,
			array_values($gaussian->kernel1D),
			$gaussian->weightSum1D,
			$terrainBlocks
		);
		if($result === null){
			return null;
		}
		$blockLayers = [];
		foreach($result[1] as $subChunkY => $layer){
			$blockLayers[$subChunkY] = self::decodeLayer($layer);
		}
		return [self::decodeLayer($result[0]), $blockLayers];
	}

	/**
	 * Does the terrain shaping of Nether::generateChunk().
	 *
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
		$layers = \pmmp_native_nether_terrain($noiseParameters, $baseX, $baseZ, $waterHeight, $emptyHeight, $emptyAmplitude, $density, Block::EMPTY_STATE_ID, $bedrock, $netherrack, $lava);
		if($layers === null){
			return null;
		}
		$result = [];
		foreach($layers as $subChunkY => $layer){
			$result[$subChunkY] = self::decodeLayer($layer);
		}
		return $result;
	}

	/**
	 * @param int|mixed[] $layer
	 * @phpstan-param NativeLayer $layer
	 */
	private static function decodeLayer(int|array $layer) : PalettedBlockArray{
		return is_int($layer) ? new PalettedBlockArray($layer) : PalettedBlockArray::fromData($layer[0], $layer[1], $layer[2]);
	}
}
