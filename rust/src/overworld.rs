//! Complete `Normal::generateChunk()`: biome selection, elevation smoothing, terrain shaping and ground cover.
//!
//! As with the rest of this crate, `None` means "let the PHP implementation handle it".

use crate::palette::{self, Layer, SUBCHUNK_VOLUME};
use crate::simplex::Simplex;
use crate::terrain::{shape_normal_terrain, ChunkBlocks, Layers, NormalTerrain, CHUNK_EDGE, COLUMNS, MAX_ELEVATION};

/// Size of each axis of the biome lookup table built by `BiomeSelector::recalculate()`.
const BIOME_MAP_EDGE: i64 = 64;
const MAX_SMOOTH_SIZE: usize = 16;
const NOISE_SAMPLING_RATE_Y: i64 = 8;
/// `GroundCover` never places blocks above this.
const MAX_COVER_Y: i64 = 127;

pub struct CoverBlock {
    pub id: u32,
    pub solid: bool,
    pub can_be_flowed_into: bool,
}

pub struct Biome {
    pub min_elevation: i64,
    pub max_elevation: i64,
    pub ground_cover: Vec<CoverBlock>,
}

/// Properties of a block that terrain shaping can place, as needed by ground cover.
#[derive(Clone, Copy)]
pub struct TerrainBlock {
    pub id: u32,
    pub transparent: bool,
    pub air: bool,
    pub liquid: bool,
}

pub struct NormalChunk<'a> {
    pub noise: &'a Simplex,
    pub temperature: &'a Simplex,
    pub rainfall: &'a Simplex,
    /// Biome ID by `temperature + (rainfall << 6)`.
    pub biome_map: &'a [i64],
    /// Indexed by biome ID.
    pub biomes: &'a [Option<Biome>],
    pub seed: i64,
    pub base_x: i64,
    pub base_z: i64,
    pub water_height: i64,
    /// `Gaussian::$kernel1D` and `Gaussian::$weightSum1D`. Taken from PHP so the weights are bit-identical.
    pub kernel: &'a [f64],
    pub weight_sum: f64,
    pub empty: TerrainBlock,
    pub bedrock: TerrainBlock,
    pub stone: TerrainBlock,
    pub water: TerrainBlock,
}

pub struct GeneratedChunk {
    /// Biomes for every subchunk; they are the same at every height.
    pub biomes: Layer,
    pub blocks: Layers,
}

/// Equivalent of `Normal::pickBiome()` followed by `BiomeSelector::pickBiome()`.
fn pick_biome(params: &NormalChunk, x: i64, z: i64) -> Option<i64> {
    let hash = x.checked_mul(2345803)? ^ z.checked_mul(9236449)? ^ params.seed;
    // PHP integers become floats when they overflow, which this nearly always does
    let hash = match hash.checked_add(223) {
        Some(sum) => match hash.checked_mul(sum) {
            Some(product) => product as f64,
            None => hash as f64 * sum as f64,
        },
        None => hash as f64 * (hash as f64 + 223.0),
    };
    let hash = (hash % 9_223_372_036_854_775_808.0) as i64;
    let mut x_noise = (hash >> 20) & 3;
    let mut z_noise = (hash >> 22) & 3;
    if x_noise == 3 {
        x_noise = 1;
    }
    if z_noise == 3 {
        z_noise = 1;
    }

    let x = (x + x_noise - 1) as f64;
    let z = (z + z_noise - 1) as f64;
    let temperature = ((params.temperature.noise_2d_normalized(x, z) + 1.0) / 2.0 * 63.0) as i64;
    let rainfall = ((params.rainfall.noise_2d_normalized(x, z) + 1.0) / 2.0 * 63.0) as i64;

    let index = usize::try_from(temperature + (rainfall << 6)).ok()?;
    params.biome_map.get(index).copied()
}

/// Equivalent of `Normal::generateBiomes()`. Returns the biome ID and the smoothed minimum and maximum elevation of
/// every column, all indexed `x * 16 + z`.
fn generate_biomes(params: &NormalChunk) -> Option<(Vec<i64>, Vec<f64>, Vec<f64>)> {
    if params.kernel.len() % 2 == 0 || params.kernel.len() > MAX_SMOOTH_SIZE * 2 + 1 {
        return None;
    }
    if params.weight_sum == 0.0 || !params.weight_sum.is_finite() || !params.kernel.iter().all(|w| w.is_finite()) {
        return None;
    }
    if params.biome_map.len() != (BIOME_MAP_EDGE * BIOME_MAP_EDGE) as usize {
        return None;
    }
    let padding = params.kernel.len() / 2;
    let edge = CHUNK_EDGE + padding * 2;

    let magnitude = params.base_x.unsigned_abs().max(params.base_z.unsigned_abs()) as f64 + (edge + 2) as f64;
    if !params.temperature.supports_coordinate(magnitude) || !params.rainfall.supports_coordinate(magnitude) {
        return None;
    }

    // biomes of the chunk and the surrounding padding, indexed by offset from the corner of the padded area
    let mut biome_cache: Vec<&Biome> = Vec::with_capacity(edge * edge);
    let mut biome_ids = vec![0i64; COLUMNS];
    let mut uniform = true;
    let mut first_id = None;
    for px in 0..edge {
        let x = px as i64 - padding as i64;
        for pz in 0..edge {
            let z = pz as i64 - padding as i64;

            let biome_id = pick_biome(params, params.base_x.checked_add(x)?, params.base_z.checked_add(z)?)?;
            let biome = params.biomes.get(usize::try_from(biome_id).ok()?)?.as_ref()?;
            if biome.min_elevation.abs() as f64 > MAX_ELEVATION || biome.max_elevation.abs() as f64 > MAX_ELEVATION {
                return None;
            }
            biome_cache.push(biome);
            uniform &= *first_id.get_or_insert(biome_id) == biome_id;

            if (0..CHUNK_EDGE as i64).contains(&x) && (0..CHUNK_EDGE as i64).contains(&z) {
                biome_ids[x as usize * CHUNK_EDGE + z as usize] = biome_id;
            }
        }
    }

    if uniform {
        // nothing to blur if the whole area has the same elevation
        let biome = biome_cache[0];
        let min_heights = vec![(biome.min_elevation - 1) as f64; COLUMNS];
        let max_heights = vec![biome.max_elevation as f64; COLUMNS];
        return Some((biome_ids, min_heights, max_heights));
    }

    // blur along the X axis first, including the Z padding which the second pass needs
    let mut min_heights_x = vec![0.0; CHUNK_EDGE * edge];
    let mut max_heights_x = vec![0.0; CHUNK_EDGE * edge];
    for x in 0..CHUNK_EDGE {
        for pz in 0..edge {
            let mut min_sum = 0.0;
            let mut max_sum = 0.0;
            for (offset, &weight) in params.kernel.iter().enumerate() {
                let adjacent = biome_cache[(x + offset) * edge + pz];
                min_sum += (adjacent.min_elevation - 1) as f64 * weight;
                max_sum += adjacent.max_elevation as f64 * weight;
            }
            min_heights_x[x * edge + pz] = min_sum / params.weight_sum;
            max_heights_x[x * edge + pz] = max_sum / params.weight_sum;
        }
    }

    let mut min_heights = vec![0.0; COLUMNS];
    let mut max_heights = vec![0.0; COLUMNS];
    for x in 0..CHUNK_EDGE {
        for z in 0..CHUNK_EDGE {
            let mut min_sum = 0.0;
            let mut max_sum = 0.0;
            for (offset, &weight) in params.kernel.iter().enumerate() {
                min_sum += min_heights_x[x * edge + z + offset] * weight;
                max_sum += max_heights_x[x * edge + z + offset] * weight;
            }
            min_heights[x * CHUNK_EDGE + z] = min_sum / params.weight_sum;
            max_heights[x * CHUNK_EDGE + z] = max_sum / params.weight_sum;
        }
    }

    Some((biome_ids, min_heights, max_heights))
}

/// Equivalent of `GroundCover::populate()` for a chunk that only contains blocks placed by terrain shaping.
fn apply_ground_cover(params: &NormalChunk, chunk: &mut ChunkBlocks, biome_ids: &[i64]) -> Option<()> {
    let terrain_blocks = [params.empty, params.bedrock, params.stone, params.water];
    let block_info = |id: u32| terrain_blocks.iter().find(|block| block.id == id);

    for x in 0..CHUNK_EDGE {
        for z in 0..CHUNK_EDGE {
            let biome = params.biomes[biome_ids[x * CHUNK_EDGE + z] as usize].as_ref()?;
            let cover = &biome.ground_cover;
            if cover.is_empty() {
                continue;
            }
            let diff_y = if cover[0].solid { 0 } else { 1 };

            let Some(mut start_y) = chunk.highest_block_at(x, z) else {
                continue;
            };
            while start_y > 0 {
                if !block_info(chunk.get(x, start_y, z)?)?.transparent {
                    break;
                }
                start_y -= 1;
            }
            let start_y = MAX_COVER_Y.min(start_y + diff_y);
            let end_y = start_y - cover.len() as i64;

            let mut y = start_y;
            while y > end_y && y >= 0 {
                let cover_block = &cover[(start_y - y) as usize];
                // blocks below the top of the column haven't been covered yet, so they are still terrain blocks
                let existing = block_info(chunk.get(x, y, z)?)?;
                if existing.air && cover_block.solid {
                    break;
                }
                if !(cover_block.can_be_flowed_into && existing.liquid) {
                    chunk.set(x, y, z, cover_block.id)?;
                }
                y -= 1;
            }
        }
    }
    Some(())
}

pub fn normal_chunk(params: &NormalChunk) -> Option<GeneratedChunk> {
    let (biome_ids, min_heights, max_heights) = generate_biomes(params)?;

    let lowest_noise_block = min_heights.iter().copied().fold(f64::INFINITY, f64::min).floor() as i64;
    let highest_noise_block = max_heights.iter().copied().fold(f64::NEG_INFINITY, f64::max).ceil() as i64;
    // aligned with the sampling rate: floor for the minimum, ceil for the maximum
    let noise_min = lowest_noise_block.div_euclid(NOISE_SAMPLING_RATE_Y) * NOISE_SAMPLING_RATE_Y;
    let noise_max = -(-highest_noise_block).div_euclid(NOISE_SAMPLING_RATE_Y) * NOISE_SAMPLING_RATE_Y;

    let mut chunk = shape_normal_terrain(
        params.noise,
        &NormalTerrain {
            base_x: params.base_x,
            base_z: params.base_z,
            noise_min,
            noise_max,
            water_height: params.water_height,
            min_heights: &min_heights,
            max_heights: &max_heights,
            empty_id: params.empty.id,
            bedrock_id: params.bedrock.id,
            stone_id: params.stone.id,
            water_id: params.water.id,
        },
    )?;
    apply_ground_cover(params, &mut chunk, &biome_ids)?;

    let mut biomes = [0u32; SUBCHUNK_VOLUME];
    for (column, &biome_id) in biome_ids.iter().enumerate() {
        biomes[column * 16..(column + 1) * 16].fill(u32::try_from(biome_id).ok()?);
    }

    Some(GeneratedChunk { biomes: palette::encode(&biomes), blocks: chunk.into_layers() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplex::tests::test_simplex;

    const EMPTY: u32 = 0;
    const BEDROCK: u32 = 1;
    const STONE: u32 = 2;
    const WATER: u32 = 3;
    const GRASS: u32 = 10;
    const DIRT: u32 = 11;
    const SNOW_LAYER: u32 = 12;

    fn block(id: u32, transparent: bool, air: bool, liquid: bool) -> TerrainBlock {
        TerrainBlock { id, transparent, air, liquid }
    }

    fn cover(blocks: &[(u32, bool)]) -> Vec<CoverBlock> {
        blocks.iter().map(|&(id, solid)| CoverBlock { id, solid, can_be_flowed_into: false }).collect()
    }

    struct Fixture {
        noise: Simplex,
        temperature: Simplex,
        rainfall: Simplex,
        biome_map: Vec<i64>,
        biomes: Vec<Option<Biome>>,
        kernel: Vec<f64>,
    }

    impl Fixture {
        /// `biome_for` maps a (temperature, rainfall) table cell to a biome ID.
        fn new(biome_for: impl Fn(i64, i64) -> i64, biomes: Vec<Option<Biome>>) -> Self {
            Self {
                noise: test_simplex(4, 0.25, 1.0 / 32.0),
                temperature: test_simplex(2, 1.0 / 16.0, 1.0 / 512.0),
                rainfall: test_simplex(2, 1.0 / 16.0, 1.0 / 64.0),
                biome_map: (0..4096).map(|i| biome_for(i & 63, i >> 6)).collect(),
                biomes,
                kernel: vec![0.5, 1.2, 2.0, 1.2, 0.5],
            }
        }

        fn params(&self, base_x: i64) -> NormalChunk<'_> {
            NormalChunk {
                noise: &self.noise,
                temperature: &self.temperature,
                rainfall: &self.rainfall,
                biome_map: &self.biome_map,
                biomes: &self.biomes,
                seed: 1337,
                base_x,
                base_z: -32,
                water_height: 62,
                kernel: &self.kernel,
                weight_sum: self.kernel.iter().sum(),
                empty: block(EMPTY, true, true, false),
                bedrock: block(BEDROCK, false, false, false),
                stone: block(STONE, false, false, false),
                water: block(WATER, true, false, true),
            }
        }
    }

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

    #[test]
    fn single_biome_chunk_is_uniform_and_covered() {
        let grassy = Biome {
            min_elevation: 66,
            max_elevation: 80,
            ground_cover: cover(&[(GRASS, true), (DIRT, true), (DIRT, true)]),
        };
        let fixture = Fixture::new(|_, _| 1, vec![None, Some(grassy)]);
        let chunk = normal_chunk(&fixture.params(160)).unwrap();

        assert_eq!(chunk.biomes, Layer::Uniform(1));
        for x in 0..16 {
            for z in 0..16 {
                // elevation is entirely above sea level, so every column is land topped with the ground cover
                let top = (0..128).rev().find(|&y| block_at(&chunk.blocks, x, y, z) != EMPTY).unwrap();
                assert!(top >= 65, "column {x},{z} tops out at {top}");
                assert_eq!(block_at(&chunk.blocks, x, top, z), GRASS);
                assert_eq!(block_at(&chunk.blocks, x, top - 1, z), DIRT);
                assert_eq!(block_at(&chunk.blocks, x, top - 2, z), DIRT);
                assert_eq!(block_at(&chunk.blocks, x, top - 3, z), STONE);
                assert_eq!(block_at(&chunk.blocks, x, 0, z), BEDROCK);
            }
        }
    }

    #[test]
    fn non_solid_cover_sits_on_top_of_the_terrain() {
        let snowy = Biome {
            min_elevation: 66,
            max_elevation: 80,
            ground_cover: cover(&[(SNOW_LAYER, false), (GRASS, true), (DIRT, true)]),
        };
        let fixture = Fixture::new(|_, _| 0, vec![Some(snowy)]);
        let chunk = normal_chunk(&fixture.params(0)).unwrap();

        let top = (0..128).rev().find(|&y| block_at(&chunk.blocks, 5, y, 9) != EMPTY).unwrap();
        assert_eq!(block_at(&chunk.blocks, 5, top, 9), SNOW_LAYER);
        assert_eq!(block_at(&chunk.blocks, 5, top - 1, 9), GRASS);
        assert_eq!(block_at(&chunk.blocks, 5, top - 2, 9), DIRT);
        assert_eq!(block_at(&chunk.blocks, 5, top - 3, 9), STONE);
    }

    #[test]
    fn elevation_is_blended_between_biomes() {
        let biome = |min, max| Some(Biome { min_elevation: min, max_elevation: max, ground_cover: Vec::new() });
        // rainfall varies quickly in the fixture, so splitting on it gives several biome borders
        let fixture = Fixture::new(|_, rainfall| i64::from(rainfall >= 32), vec![biome(40, 50), biome(90, 100)]);

        let mixed = (0..64)
            .map(|i| generate_biomes(&fixture.params(i * 16)).unwrap())
            .find(|(ids, _, _)| ids.iter().any(|&id| id == 0) && ids.iter().any(|&id| id == 1))
            .expect("no chunk with a biome border found");
        let (_, min_heights, max_heights) = mixed;

        assert!(min_heights.iter().all(|h| (39.0..=89.0).contains(h)));
        assert!(max_heights.iter().all(|h| (50.0..=100.0).contains(h)));
        // blurring produces elevations which neither biome has on its own
        assert!(min_heights.iter().any(|&h| h > 39.5 && h < 88.5));
    }

    #[test]
    fn defers_to_php_for_input_it_cannot_reproduce() {
        let biome = || Some(Biome { min_elevation: 66, max_elevation: 80, ground_cover: Vec::new() });
        let fixture = Fixture::new(|_, _| 0, vec![biome()]);
        assert!(normal_chunk(&fixture.params(0)).is_some());

        let unknown_biome = Fixture::new(|_, _| 7, vec![biome()]);
        assert!(normal_chunk(&unknown_biome.params(0)).is_none());

        let mut even_kernel = Fixture::new(|_, _| 0, vec![biome()]);
        even_kernel.kernel.pop();
        assert!(normal_chunk(&even_kernel.params(0)).is_none());

        assert!(normal_chunk(&NormalChunk { weight_sum: 0.0, ..fixture.params(0) }).is_none());
        assert!(normal_chunk(&NormalChunk { base_x: i64::MAX - 8, ..fixture.params(0) }).is_none());
        assert!(normal_chunk(&NormalChunk { biome_map: &fixture.biome_map[..100], ..fixture.params(0) }).is_none());
    }
}
