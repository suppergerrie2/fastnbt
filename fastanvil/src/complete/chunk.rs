use std::ops::Range;

use crate::biome::Biome;
use crate::complete::section_tower::SectionTower;
// Chunk as DimensionChunk need because complete::Chunk and dimension::Chunk are both called Chunk maybe rename one
use crate::{
    dimension, Block, Chunk as DimensionChunk, CurrentJavaChunk, HeightMode, Heightmaps, JavaChunk,
};

pub struct Chunk {
    pub status: String,

    pub sections: SectionTower,

    //todo more | different Heightmaps
    pub heightmap: [i16; 256],
}

impl Chunk {
    pub fn from_bytes(data: &[u8]) -> fastnbt::error::Result<Self> {
        return match JavaChunk::from_bytes(data)? {
            JavaChunk::Post18(chunk) => Ok(chunk.into()),
            JavaChunk::Pre18(_) => {
                todo!()
            }
            JavaChunk::Pre13(_) => {
                todo!()
            }
        };
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = &Block> {
        self.sections.iter_blocks()
    }

    pub fn is_valid_block_cord(&self, x: usize, y: isize, z: usize) -> bool {
        self.y_range().contains(&y) && (0..16).contains(&x) && (0..16).contains(&z)
    }
}

impl dimension::Chunk for Chunk {
    fn status(&self) -> String {
        self.status.clone()
    }

    fn surface_height(&self, x: usize, z: usize, mode: HeightMode) -> isize {
        match mode {
            HeightMode::Trust => self.heightmap[z * 16 + x] as isize,
            HeightMode::Calculate => {
                todo!()
            }
        }
    }

    fn biome(&self, x: usize, y: isize, z: usize) -> Option<Biome> {
        if !self.is_valid_block_cord(x, y, z) {
            return None;
        }

        self.sections.biome(x, y, z)
    }

    fn block(&self, x: usize, y: isize, z: usize) -> Option<&Block> {
        if !self.is_valid_block_cord(x, y, z) {
            return None;
        }

        self.sections.block(x, y, z)
    }

    fn y_range(&self) -> Range<isize> {
        self.sections.y_range()
    }
}

impl From<CurrentJavaChunk> for Chunk {
    fn from(current_java_chunk: CurrentJavaChunk) -> Self {
        //probably find better way. maybe always recalculate if need and then cache
        current_java_chunk.recalculate_heightmap(HeightMode::Trust);
        let heightmap = current_java_chunk
            .lazy_heightmap
            .read()
            .unwrap()
            .unwrap();

        Chunk {
            status: current_java_chunk.status.clone(),
            sections: current_java_chunk.sections.unwrap().into(),
            heightmap,
        }
    }
}
