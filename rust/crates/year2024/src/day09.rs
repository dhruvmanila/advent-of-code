use std::fmt::{self, Write};

use anyhow::Result;

/// A block of memory that represents a contiguous block of files or free space.
#[derive(Debug, Clone, Copy)]
enum WholeBlock {
    File { id: u32, size: u8 },
    Free(u8),
}

impl WholeBlock {
    /// Return `true` if this is a free block.
    const fn is_free(self) -> bool {
        matches!(self, WholeBlock::Free(_))
    }

    /// Return `true` if this is a file block.
    const fn is_file(self) -> bool {
        matches!(self, WholeBlock::File { .. })
    }

    /// Return the size of the block.
    const fn size(self) -> u8 {
        match self {
            WholeBlock::Free(size) | WholeBlock::File { size, .. } => size,
        }
    }

    /// Reduce the size of the block by `size`.
    const fn reduce_size(&mut self, size: u8) {
        match self {
            WholeBlock::Free(current_size)
            | WholeBlock::File {
                size: current_size, ..
            } => *current_size -= size,
        }
    }

    /// Returns an iterator over the individual blocks contained in this whole block.
    fn individual_blocks(self) -> impl Iterator<Item = Block> {
        match self {
            WholeBlock::File { id, size } => std::iter::repeat(Block::File(id)).take(size as usize),
            WholeBlock::Free(size) => std::iter::repeat(Block::Free).take(size as usize),
        }
    }
}

impl fmt::Display for WholeBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in self.individual_blocks() {
            write!(f, "{block}")?;
        }
        Ok(())
    }
}

/// An individual block in the memory.
#[derive(Debug, Clone, Copy)]
enum Block {
    File(u32),
    Free,
}

impl Block {
    /// Returns the file ID if the block is a file, otherwise `None`.
    const fn as_file(self) -> Option<u32> {
        match self {
            Block::File(id) => Some(id),
            Block::Free => None,
        }
    }

    /// Returns `true` if this is a file block.
    const fn is_file(self) -> bool {
        matches!(self, Block::File(_))
    }

    /// Returns `true` if this is a free block.
    const fn is_free(self) -> bool {
        matches!(self, Block::Free)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::File(id) => write!(f, "{id}"),
            Block::Free => f.write_char('.'),
        }
    }
}

/// A disk map that represents the memory layout of files and free space on the disk.
#[derive(Debug)]
struct DiskMap<'a>(&'a str);

impl DiskMap<'_> {
    /// Decode the disk map into individual and whole blocks.
    fn decode(self) -> (Blocks, WholeBlocks) {
        enum BlockKind {
            File,
            Free,
        }

        let mut individual_blocks = Vec::new();
        let mut whole_blocks = Vec::with_capacity(self.0.len());

        let mut file_id = 0;

        for (byte, kind) in self
            .0
            .bytes()
            .zip([BlockKind::File, BlockKind::Free].iter().cycle())
        {
            if byte == b'0' {
                continue;
            }
            let whole_block = match kind {
                BlockKind::File => {
                    let block = WholeBlock::File {
                        id: file_id,
                        size: byte - b'0',
                    };
                    file_id += 1;
                    block
                }
                BlockKind::Free => WholeBlock::Free(byte - b'0'),
            };
            individual_blocks.extend(whole_block.individual_blocks());
            whole_blocks.push(whole_block);
        }

        (Blocks(individual_blocks), WholeBlocks(whole_blocks))
    }
}

/// A collection of individual blocks in the memory that represents a file system.
#[derive(Debug)]
struct Blocks(Vec<Block>);

impl Blocks {
    /// Compact the file system by moving all the file blocks to the front of the memory.
    ///
    /// This operation is done in-place by swapping the blocks.
    fn compact(&mut self) {
        let mut free_pos = 0;
        let mut file_pos = self.0.len();

        loop {
            let Some(next_free_pos) = self
                .0
                .get(free_pos..)
                .and_then(|blocks| blocks.iter().position(|block| block.is_free()))
                .map(|offset| free_pos + offset)
            else {
                break;
            };
            let Some(next_file_pos) = self
                .0
                .get(0..file_pos)
                .and_then(|blocks| blocks.iter().rposition(|block| block.is_file()))
            else {
                break;
            };
            free_pos = next_free_pos;
            file_pos = next_file_pos;
            if free_pos >= file_pos {
                break;
            }
            self.0.swap(free_pos, file_pos);
        }
    }

    /// Calculate the checksum of the file system.
    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, block)| block.as_file().map(|file_id| file_id as usize * index))
            .sum()
    }
}

/// A collection of whole blocks in the file system.
///
/// Each element represents a contiguous block of files or free space.
#[derive(Debug)]
struct WholeBlocks(Vec<WholeBlock>);

impl WholeBlocks {
    /// Compact the file system by moving all the file blocks to the front of the memory.
    ///
    /// This operation is done in-place.
    fn compact(&mut self) {
        let mut file_pos = self.0.len();

        loop {
            let Some(next_file_pos) = self
                .0
                .get(0..file_pos)
                .and_then(|blocks| blocks.iter().rposition(|block| block.is_file()))
            else {
                break;
            };
            file_pos = next_file_pos;

            let file_block = self.0[file_pos];

            // Find the first free space block that can accommodate the entire file block.
            let Some(free_space_pos) = self.0.get(0..file_pos).and_then(|blocks| {
                blocks
                    .iter()
                    .position(|block| block.is_free() && block.size() >= file_block.size())
            }) else {
                continue;
            };

            let free_space_block = &mut self.0[free_space_pos];

            if free_space_block.size() == file_block.size() {
                // Optimization: If the size is the same, then we can just swap the blocks.
                self.0.swap(free_space_pos, file_pos);
            } else {
                // Otherwise, (1) reduce the size of the free space block to accommodate the file
                // block
                free_space_block.reduce_size(file_block.size());
                // then, (2) replace the file block with a free space block of the same size
                let _ =
                    std::mem::replace(&mut self.0[file_pos], WholeBlock::Free(file_block.size()));
                // and finally, (3) insert the file block at the free space position
                self.0.insert(free_space_pos, file_block);
                // Increment the file block position to account for the newly inserted file
                // block.
                file_pos += 1;
            }
        }
    }

    /// Calculate the checksum of the file system.
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut index = 0;

        for block in &self.0 {
            match block {
                WholeBlock::File { id, size } => {
                    for _ in 0..*size {
                        checksum += *id as usize * index;
                        index += 1;
                    }
                }
                WholeBlock::Free(size) => index += *size as usize,
            }
        }

        checksum
    }
}

pub fn solve(input: &str) -> Result<()> {
    let (mut individual_blocks, mut whole_blocks) = DiskMap(input.trim()).decode();

    individual_blocks.compact();
    println!("Part 1: {:?}", individual_blocks.checksum());

    whole_blocks.compact();
    println!("Part 2: {:?}", whole_blocks.checksum());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn sample() {
        let (mut individual_blocks, mut whole_blocks) = DiskMap(SAMPLE_INPUT).decode();

        individual_blocks.compact();
        assert_eq!(individual_blocks.checksum(), 1928);

        whole_blocks.compact();
        assert_eq!(whole_blocks.checksum(), 2858);
    }
}
