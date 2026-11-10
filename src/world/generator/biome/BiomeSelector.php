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

namespace pocketmine\world\generator\biome;

use pocketmine\utils\Random;
use pocketmine\world\biome\Biome;
use pocketmine\world\biome\BiomeRegistry;
use pocketmine\world\biome\UnknownBiome;
use pocketmine\world\generator\noise\Simplex;
use function count;

/**
 * @phpstan-import-type NativeSimplexParameters from Simplex
 */
abstract class BiomeSelector{
	private Simplex $temperature;
	private Simplex $rainfall;

	/**
	 * @var Biome[]|\SplFixedArray
	 * @phpstan-var \SplFixedArray<Biome>
	 */
	private \SplFixedArray $map;

	/**
	 * Same as the map, but biome IDs instead of biomes. Empty if native code can't be used with this selector.
	 * @var int[]
	 * @phpstan-var list<int>
	 */
	private array $nativeBiomeIds = [];
	/**
	 * @var Biome[] biome ID => biome, for every biome in the map
	 * @phpstan-var array<int, Biome>
	 */
	private array $nativeBiomes = [];

	public function __construct(Random $random){
		$this->temperature = new Simplex($random, 2, 1 / 16, 1 / 512);
		$this->rainfall = new Simplex($random, 2, 1 / 16, 1 / 512);
	}

	/**
	 * Lookup function called by recalculate() to determine the biome to use for this temperature and rainfall.
	 *
	 * @return int biome ID 0-255
	 */
	abstract protected function lookup(float $temperature, float $rainfall) : int;

	public function recalculate() : void{
		$this->map = new \SplFixedArray(64 * 64);

		$biomeRegistry = BiomeRegistry::getInstance();
		for($i = 0; $i < 64; ++$i){
			for($j = 0; $j < 64; ++$j){
				$biome = $biomeRegistry->getBiome($this->lookup($i / 63, $j / 63));
				if($biome instanceof UnknownBiome){
					throw new \RuntimeException("Unknown biome returned by selector with ID " . $biome->getId());
				}
				$this->map[$i + ($j << 6)] = $biome;
			}
		}

		$this->nativeBiomeIds = [];
		$this->nativeBiomes = [];
		foreach(["pickBiome", "getTemperature", "getRainfall"] as $method){
			if((new \ReflectionMethod($this, $method))->getDeclaringClass()->getName() !== self::class){
				//native code can't know what an overridden method does
				return;
			}
		}
		foreach($this->map as $biome){
			$this->nativeBiomeIds[] = $biome->getId();
			$this->nativeBiomes[$biome->getId()] = $biome;
		}
	}

	/**
	 * Returns the description of this selector accepted by functions of the optional pmmpnative extension, or null
	 * if native code can't be used.
	 *
	 * @internal
	 * @return mixed[]|null
	 * @phpstan-return array{NativeSimplexParameters, NativeSimplexParameters, list<int>, array<int, Biome>}|null
	 */
	public function getNativeParameters() : ?array{
		$temperature = $this->temperature->getNativeParameters();
		$rainfall = $this->rainfall->getNativeParameters();
		if($temperature === null || $rainfall === null || count($this->nativeBiomeIds) === 0){
			return null;
		}
		return [$temperature, $rainfall, $this->nativeBiomeIds, $this->nativeBiomes];
	}

	public function getTemperature(float $x, float $z) : float{
		return ($this->temperature->noise2D($x, $z, true) + 1) / 2;
	}

	public function getRainfall(float $x, float $z) : float{
		return ($this->rainfall->noise2D($x, $z, true) + 1) / 2;
	}

	public function pickBiome(float $x, float $z) : Biome{
		$temperature = (int) ($this->getTemperature($x, $z) * 63);
		$rainfall = (int) ($this->getRainfall($x, $z) * 63);

		return $this->map[$temperature + ($rainfall << 6)];
	}
}
