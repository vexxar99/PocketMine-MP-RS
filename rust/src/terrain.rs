//! Terrain shaping for the built-in Normal and Nether generators.
//!
//! These replace the per-block PHP loops in `Normal::generateChunk()` and `Nether::generateChunk()`. They return
//! `None` whenever the PHP implementation would throw or hit an edge case that isn't worth reproducing here; the
//! caller then runs the PHP implementation instead, so behaviour in those cases is unchanged.

use crate::palette::{self, Layer, SUBCHUNK_VOLUME};
use crate::simplex::Simplex;

pub const MIN_SUBCHUNK_INDEX: i64 = -4;
pub const MAX_SUBCHUNK_INDEX: i64 = 19;
const SUBCHUNK_COUNT: usize = (MAX_SUBCHUNK_INDEX - MIN_SUBCHUNK_INDEX + 1) as usize;
const CHUNK_EDGE: usize = 16;
const COLUMNS: usize = CHUNK_EDGE * CHUNK_EDGE;

/// Elevations beyond this can't produce a valid chunk anyway.
const MAX_ELEVATION: f64 = 1_000_000.0;

/// Block layers by subchunk Y. Subchunks without an entry have no block layers.
pub type Layers = Vec<(i64, Layer)>;

struct ChunkBlocks {
    empty_id: u32,
    blocks: Vec<u32>,
    has_layer: [bool; SUBCHUNK_COUNT],
}

impl ChunkBlocks {
    fn new(empty_id: u32) -> Self {
        Self { empty_id, blocks: vec![empty_id; SUBCHUNK_COUNT * SUBCHUNK_VOLUME], has_layer: [false; SUBCHUNK_COUNT] }
    }

    fn fill_subchunk(&mut self, subchunk_y: i64, id: u32) {
        let index = (subchunk_y - MIN_SUBCHUNK_INDEX) as usize;
        self.blocks[index * SUBCHUNK_VOLUME..(index + 1) * SUBCHUNK_VOLUME].fill(id);
        self.has_layer[index] = true;
    }

    /// Returns `None` if the Y coordinate is outside the world, where `Chunk::setBlockStateId()` would throw.
    fn set(&mut self, x: usize, y: i64, z: usize, id: u32) -> Option<()> {
        let subchunk_y = y >> 4;
        if !(MIN_SUBCHUNK_INDEX..=MAX_SUBCHUNK_INDEX).contains(&subchunk_y) {
            return None;
        }
        let index = (subchunk_y - MIN_SUBCHUNK_INDEX) as usize;
        self.blocks[index * SUBCHUNK_VOLUME + ((x << 8) | (z << 4) | (y & 0xf) as usize)] = id;
        self.has_layer[index] = true;
        Some(())
    }

    fn into_layers(self) -> Layers {
        let mut layers = Vec::new();
        for (index, blocks) in self.blocks.chunks(SUBCHUNK_VOLUME).enumerate() {
            if !self.has_layer[index] {
                continue;
            }
            let layer = palette::encode(blocks);
            if layer != Layer::Uniform(self.empty_id) {
                layers.push((index as i64 + MIN_SUBCHUNK_INDEX, layer));
            }
        }
        layers
    }
}

pub struct NormalTerrain<'a> {
    pub base_x: i64,
    pub base_z: i64,
    /// Vertical bounds of the noise field, as calculated by `Normal::generateChunk()`.
    pub noise_min: i64,
    pub noise_max: i64,
    pub water_height: i64,
    /// Smoothed biome elevations per column, indexed `x * 16 + z`.
    pub min_heights: &'a [f64],
    pub max_heights: &'a [f64],
    pub empty_id: u32,
    pub bedrock_id: u32,
    pub stone_id: u32,
    pub water_id: u32,
}

pub fn normal_terrain(noise: &Simplex, params: &NormalTerrain) -> Option<Layers> {
    const SAMPLING_RATE_Y: i64 = 8;

    let NormalTerrain { noise_min, noise_max, water_height, .. } = *params;
    if params.min_heights.len() != COLUMNS || params.max_heights.len() != COLUMNS {
        return None;
    }
    if !params.min_heights.iter().chain(params.max_heights).all(|h| h.abs() <= MAX_ELEVATION) {
        return None;
    }
    if noise_min > noise_max || noise_min % SAMPLING_RATE_Y != 0 || noise_max % SAMPLING_RATE_Y != 0 {
        return None;
    }
    if (noise_min.abs().max(noise_max.abs()) as f64) > MAX_ELEVATION {
        return None;
    }

    let grid = noise.fast_noise_3d(
        CHUNK_EDGE as i64,
        noise_max - noise_min,
        CHUNK_EDGE as i64,
        4,
        SAMPLING_RATE_Y,
        4,
        params.base_x,
        noise_min,
        params.base_z,
    )?;

    let mut chunk = ChunkBlocks::new(params.empty_id);

    // Everything above 0 and below the noise field is always solid stone
    let min_noise_subchunk = noise_min.div_euclid(16);
    for subchunk_y in 0..min_noise_subchunk.min(MAX_SUBCHUNK_INDEX + 1) {
        chunk.fill_subchunk(subchunk_y, params.stone_id);
    }

    for x in 0..CHUNK_EDGE {
        for z in 0..CHUNK_EDGE {
            chunk.set(x, 0, z, params.bedrock_id)?;

            let min_sum = params.min_heights[x * CHUNK_EDGE + z];
            let max_sum = params.max_heights[x * CHUNK_EDGE + z];
            let max_block_y = max_sum.max(water_height as f64);
            let smooth_height = (max_sum - min_sum) / 2.0;
            if smooth_height == 0.0 {
                return None;
            }

            let mut y = min_noise_subchunk * 16;
            while (y as f64) < min_sum {
                chunk.set(x, y, z, params.stone_id)?;
                y += 1;
            }

            let mut y = min_sum.floor() as i64;
            while (y as f64) <= max_block_y {
                let noise_value = if y > noise_max {
                    -1.0
                } else {
                    let noise_y = usize::try_from(y - noise_min).ok()?;
                    grid.get(x, z, noise_y) - 1.0 / smooth_height * (y as f64 - smooth_height - min_sum)
                };

                if noise_value > 0.0 {
                    chunk.set(x, y, z, params.stone_id)?;
                } else if y <= water_height {
                    chunk.set(x, y, z, params.water_id)?;
                }
                y += 1;
            }
        }
    }

    Some(chunk.into_layers())
}

pub struct NetherTerrain {
    pub base_x: i64,
    pub base_z: i64,
    pub water_height: i64,
    pub empty_height: i64,
    pub empty_amplitude: i64,
    pub density: f64,
    pub empty_id: u32,
    pub bedrock_id: u32,
    pub netherrack_id: u32,
    pub lava_id: u32,
}

pub fn nether_terrain(noise: &Simplex, params: &NetherTerrain) -> Option<Layers> {
    const HEIGHT: i64 = 128;

    let NetherTerrain { empty_height, empty_amplitude, water_height, density, .. } = *params;
    if empty_height == 0 || empty_height.abs() > i32::MAX as i64 || empty_amplitude.abs() > i32::MAX as i64 {
        return None;
    }
    if !density.is_finite() {
        return None;
    }

    let grid = noise.fast_noise_3d(CHUNK_EDGE as i64, HEIGHT, CHUNK_EDGE as i64, 4, 8, 4, params.base_x, 0, params.base_z)?;

    let mut chunk = ChunkBlocks::new(params.empty_id);
    for x in 0..CHUNK_EDGE {
        for z in 0..CHUNK_EDGE {
            for y in 0..HEIGHT {
                if y == 0 || y == HEIGHT - 1 {
                    chunk.set(x, y, z, params.bedrock_id)?;
                    continue;
                }
                let mut noise_value = ((empty_height - y).abs() as f64 / empty_height as f64) * empty_amplitude as f64
                    - grid.get(x, z, y as usize);
                noise_value -= 1.0 - density;

                if noise_value > 0.0 {
                    chunk.set(x, y, z, params.netherrack_id)?;
                } else if y <= water_height {
                    chunk.set(x, y, z, params.lava_id)?;
                }
            }
        }
    }

    Some(chunk.into_layers())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplex::tests::test_simplex;

    const EMPTY: u32 = 0;
    const BEDROCK: u32 = 1;
    const STONE: u32 = 2;
    const LIQUID: u32 = 3;

    fn block_at(layers: &Layers, x: usize, y: i64, z: usize) -> u32 {
        let Some((_, layer)) = layers.iter().find(|(subchunk_y, _)| *subchunk_y == y >> 4) else {
            return EMPTY;
        };
        match layer {
            Layer::Uniform(id) => *id,
            Layer::Paletted { bits_per_block, words, palette } => {
                let bits = *bits_per_block as usize;
                let per_word = 32 / bits;
                let i = (x << 8) | (z << 4) | (y & 0xf) as usize;
                let word = u32::from_le_bytes(words[i / per_word * 4..i / per_word * 4 + 4].try_into().unwrap());
                palette[((word >> ((i % per_word) * bits)) & ((1 << bits) - 1)) as usize]
            }
        }
    }

    fn normal_params<'a>(min: &'a [f64], max: &'a [f64], noise_min: i64, noise_max: i64) -> NormalTerrain<'a> {
        NormalTerrain {
            base_x: 32,
            base_z: -48,
            noise_min,
            noise_max,
            water_height: 62,
            min_heights: min,
            max_heights: max,
            empty_id: EMPTY,
            bedrock_id: BEDROCK,
            stone_id: STONE,
            water_id: LIQUID,
        }
    }

    #[test]
    fn normal_terrain_has_expected_vertical_structure() {
        let noise = test_simplex(4, 0.25, 1.0 / 32.0);
        let (min, max) = ([45.0; COLUMNS], [74.0; COLUMNS]);
        let layers = normal_terrain(&noise, &normal_params(&min, &max, 40, 80)).unwrap();

        // subchunks 0 and 1 are below the noise field: flood filled, bedrock only in the bottom one
        assert_eq!(layers[0].0, 0);
        assert_eq!(layers[1], (1, Layer::Uniform(STONE)));
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(block_at(&layers, x, 0, z), BEDROCK);
                for y in 1..45 {
                    assert_eq!(block_at(&layers, x, y, z), STONE, "y={y}");
                }
                for y in 45..=74 {
                    let block = block_at(&layers, x, y, z);
                    assert!(block == STONE || block == if y <= 62 { LIQUID } else { EMPTY }, "y={y} block={block}");
                }
                for y in 75..96 {
                    assert_eq!(block_at(&layers, x, y, z), EMPTY, "y={y}");
                }
            }
        }
        assert!(layers.iter().all(|(y, _)| (0..=4).contains(y)));
    }

    #[test]
    fn normal_terrain_defers_to_php_for_degenerate_input() {
        let noise = test_simplex(4, 0.25, 1.0 / 32.0);
        let (min, max) = ([45.0; COLUMNS], [74.0; COLUMNS]);
        assert!(normal_terrain(&noise, &normal_params(&min[..255], &max, 40, 80)).is_none());
        assert!(normal_terrain(&noise, &normal_params(&min, &max, 41, 80)).is_none(), "unaligned noise bounds");
        assert!(normal_terrain(&noise, &normal_params(&min, &min, 40, 48)).is_none(), "division by zero in PHP");
        let too_high = [400.0; COLUMNS];
        assert!(normal_terrain(&noise, &normal_params(&min, &too_high, 40, 400)).is_none(), "outside world height");
        let nan = [f64::NAN; COLUMNS];
        assert!(normal_terrain(&noise, &normal_params(&nan, &max, 40, 80)).is_none());
    }

    #[test]
    fn nether_terrain_is_capped_with_bedrock_and_floods_low_caves() {
        let noise = test_simplex(4, 0.25, 1.0 / 64.0);
        let params = NetherTerrain {
            base_x: 160,
            base_z: 16,
            water_height: 32,
            empty_height: 64,
            empty_amplitude: 1,
            density: 0.5,
            empty_id: EMPTY,
            bedrock_id: BEDROCK,
            netherrack_id: STONE,
            lava_id: LIQUID,
        };
        let layers = nether_terrain(&noise, &params).unwrap();
        assert!(layers.iter().all(|(y, _)| (0..8).contains(y)));

        let mut seen = [0usize; 4];
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(block_at(&layers, x, 0, z), BEDROCK);
                assert_eq!(block_at(&layers, x, 127, z), BEDROCK);
                for y in 1..127 {
                    let block = block_at(&layers, x, y, z);
                    assert_ne!(block, BEDROCK);
                    assert_eq!(block == LIQUID, block != STONE && y <= 32, "y={y}");
                    seen[block as usize] += 1;
                }
            }
        }
        assert!(seen[STONE as usize] > 0 && seen[EMPTY as usize] > 0);

        assert!(nether_terrain(&noise, &NetherTerrain { empty_height: 0, ..params }).is_none());
    }
}
