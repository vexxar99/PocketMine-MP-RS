//! Encoding of 16x16x16 block storages into the paletted format accepted by `PalettedBlockArray::fromData()`.

pub const SUBCHUNK_VOLUME: usize = 4096;

/// Sizes supported by chunkutils2's PalettedBlockArray.
const BITS_PER_BLOCK: [u8; 8] = [1, 2, 3, 4, 5, 6, 8, 16];

#[derive(Debug, PartialEq, Eq)]
pub enum Layer {
    /// Every block has the same state ID.
    Uniform(u32),
    Paletted { bits_per_block: u8, words: Vec<u8>, palette: Vec<u32> },
}

/// `blocks` is indexed `(x << 8) | (z << 4) | y`, the same as PalettedBlockArray.
pub fn encode(blocks: &[u32]) -> Layer {
    assert_eq!(blocks.len(), SUBCHUNK_VOLUME);

    let mut palette: Vec<u32> = Vec::new();
    let mut indexes = [0u16; SUBCHUNK_VOLUME];
    let mut last: Option<(u32, u16)> = None;
    for (slot, &block) in indexes.iter_mut().zip(blocks) {
        *slot = match last {
            Some((id, index)) if id == block => index,
            _ => {
                let index = match palette.iter().position(|&id| id == block) {
                    Some(index) => index,
                    None => {
                        palette.push(block);
                        palette.len() - 1
                    }
                } as u16;
                last = Some((block, index));
                index
            }
        };
    }

    if palette.len() == 1 {
        return Layer::Uniform(palette[0]);
    }

    let bits_per_block = BITS_PER_BLOCK
        .into_iter()
        .find(|&bits| palette.len() <= 1usize << bits)
        .expect("a subchunk cannot have more than 4096 distinct blocks");
    let bits = bits_per_block as usize;
    let blocks_per_word = 32 / bits;

    let mut words = Vec::with_capacity(SUBCHUNK_VOLUME.div_ceil(blocks_per_word) * 4);
    for chunk in indexes.chunks(blocks_per_word) {
        let mut word = 0u32;
        for (position, &index) in chunk.iter().enumerate() {
            word |= (index as u32) << (position * bits);
        }
        words.extend_from_slice(&word.to_le_bytes());
    }

    Layer::Paletted { bits_per_block, words, palette }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(bits_per_block: u8, words: &[u8], palette: &[u32]) -> Vec<u32> {
        let bits = bits_per_block as usize;
        let blocks_per_word = 32 / bits;
        let mask = (1u32 << bits) - 1;
        (0..SUBCHUNK_VOLUME)
            .map(|i| {
                let offset = i / blocks_per_word * 4;
                let word = u32::from_le_bytes(words[offset..offset + 4].try_into().unwrap());
                palette[((word >> ((i % blocks_per_word) * bits)) & mask) as usize]
            })
            .collect()
    }

    #[test]
    fn uniform_storage_has_no_word_array() {
        assert_eq!(encode(&[7; SUBCHUNK_VOLUME]), Layer::Uniform(7));
    }

    #[test]
    fn round_trips_at_every_palette_size() {
        for distinct in [2usize, 3, 4, 5, 9, 17, 33, 65, 257, 4096] {
            let blocks: Vec<u32> = (0..SUBCHUNK_VOLUME).map(|i| ((i * 31) % distinct) as u32 + 1000).collect();
            let Layer::Paletted { bits_per_block, words, palette } = encode(&blocks) else {
                panic!("expected paletted layer for {distinct} blocks");
            };
            assert_eq!(palette.len(), distinct);
            assert!(palette.len() <= 1 << bits_per_block);
            assert_eq!(words.len(), SUBCHUNK_VOLUME.div_ceil(32 / bits_per_block as usize) * 4);
            assert_eq!(decode(bits_per_block, &words, &palette), blocks);
        }
    }

    #[test]
    fn uses_smallest_supported_size() {
        let size_for = |distinct: usize| {
            let blocks: Vec<u32> = (0..SUBCHUNK_VOLUME).map(|i| (i % distinct) as u32).collect();
            match encode(&blocks) {
                Layer::Paletted { bits_per_block, .. } => bits_per_block,
                Layer::Uniform(_) => 0,
            }
        };
        assert_eq!(size_for(2), 1);
        assert_eq!(size_for(4), 2);
        assert_eq!(size_for(64), 6);
        assert_eq!(size_for(65), 8);
        assert_eq!(size_for(257), 16);
    }
}
