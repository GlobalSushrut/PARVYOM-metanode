//! Central directory and heap arena for ZIPLOCK-JSON

use std::collections::BTreeMap;
use std::io::{Seek, SeekFrom, Write, Read};
use zerocopy::{AsBytes, FromBytes};
use crate::{ZjlResult, ZjlError};
use crate::blocks::{CentralDirEntry, BlockType, Block};
use crate::header::FixedHeader;

/// Heap arena for managing variable-length data
pub struct HeapArena {
    /// Current write position
    position: u64,
    /// Allocated blocks
    blocks: BTreeMap<u64, HeapBlock>,
    /// Free space tracking
    free_list: Vec<FreeBlock>,
}

#[derive(Debug, Clone)]
struct HeapBlock {
    offset: u64,
    size: u64,
    block_type: BlockType,
    path_id: u64,
}

#[derive(Debug, Clone)]
struct FreeBlock {
    offset: u64,
    size: u64,
}

impl HeapArena {
    pub fn new(start_offset: u64) -> Self {
        Self {
            position: start_offset,
            blocks: BTreeMap::new(),
            free_list: Vec::new(),
        }
    }

    /// Allocate space for a block
    pub fn allocate(&mut self, size: u64, block_type: BlockType, path_id: u64) -> u64 {
        // Try to find a suitable free block first
        if let Some(free_idx) = self.find_free_block(size) {
            let free_block = self.free_list.remove(free_idx);
            let offset = free_block.offset;
            
            // If the free block is larger than needed, split it
            if free_block.size > size {
                let remaining = FreeBlock {
                    offset: offset + size,
                    size: free_block.size - size,
                };
                self.free_list.push(remaining);
            }
            
            self.blocks.insert(offset, HeapBlock {
                offset,
                size,
                block_type,
                path_id,
            });
            
            return offset;
        }

        // No suitable free block found, allocate at the end
        let offset = self.position;
        self.blocks.insert(offset, HeapBlock {
            offset,
            size,
            block_type,
            path_id,
        });
        self.position += size;
        offset
    }

    /// Free a block
    pub fn free(&mut self, offset: u64) -> ZjlResult<()> {
        let block = self.blocks.remove(&offset)
            .ok_or(ZjlError::InvalidOffset(offset))?;

        // Add to free list
        let free_block = FreeBlock {
            offset: block.offset,
            size: block.size,
        };
        
        self.free_list.push(free_block);
        self.coalesce_free_blocks();
        
        Ok(())
    }

    /// Get current heap size
    pub fn size(&self) -> u64 {
        self.position
    }

    /// Get block info
    pub fn get_block(&self, offset: u64) -> Option<&HeapBlock> {
        self.blocks.get(&offset)
    }

    /// List all allocated blocks
    pub fn allocated_blocks(&self) -> impl Iterator<Item = &HeapBlock> {
        self.blocks.values()
    }

    fn find_free_block(&self, size: u64) -> Option<usize> {
        self.free_list.iter()
            .position(|block| block.size >= size)
    }

    fn coalesce_free_blocks(&mut self) {
        // Sort free blocks by offset
        self.free_list.sort_by_key(|block| block.offset);
        
        let mut coalesced = Vec::new();
        let mut current: Option<FreeBlock> = None;
        
        for block in &self.free_list {
            match current {
                None => current = Some(block.clone()),
                Some(ref mut curr) => {
                    // Check if blocks are adjacent
                    if curr.offset + curr.size == block.offset {
                        // Coalesce
                        curr.size += block.size;
                    } else {
                        // Not adjacent, save current and start new
                        coalesced.push(curr.clone());
                        *curr = block.clone();
                    }
                }
            }
        }
        
        if let Some(curr) = current {
            coalesced.push(curr);
        }
        
        self.free_list = coalesced;
    }
}

/// Central directory for ZIPLOCK-JSON files
pub struct CentralDirectory {
    /// Directory entries
    entries: Vec<CentralDirEntry>,
    /// Path ID to entry mapping
    path_index: BTreeMap<u64, Vec<usize>>,
    /// Block type index
    type_index: BTreeMap<u8, Vec<usize>>,
}

impl CentralDirectory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            path_index: BTreeMap::new(),
            type_index: BTreeMap::new(),
        }
    }

    /// Add an entry to the directory
    pub fn add_entry(&mut self, entry: CentralDirEntry) {
        let index = self.entries.len();
        
        // Update path index
        self.path_index.entry(entry.path_id)
            .or_insert_with(Vec::new)
            .push(index);
        
        // Update type index
        self.type_index.entry(entry.block_type)
            .or_insert_with(Vec::new)
            .push(index);
        
        self.entries.push(entry);
    }

    /// Add a block to the directory
    pub fn add_block(&mut self, block: &Block, offset: u64) {
        let entry = CentralDirEntry::new(
            offset,
            block.block_type().unwrap_or(BlockType::Pad),
            block.header.path_id,
            block.header.hash,
            block.header.compressed_len,
            block.header.uncompressed_len,
        );
        self.add_entry(entry);
    }

    /// Find entries by path ID
    pub fn find_by_path(&self, path_id: u64) -> Vec<&CentralDirEntry> {
        self.path_index.get(&path_id)
            .map(|indices| indices.iter().map(|&i| &self.entries[i]).collect())
            .unwrap_or_default()
    }

    /// Find entries by block type
    pub fn find_by_type(&self, block_type: BlockType) -> Vec<&CentralDirEntry> {
        self.type_index.get(&(block_type as u8))
            .map(|indices| indices.iter().map(|&i| &self.entries[i]).collect())
            .unwrap_or_default()
    }

    /// Get all entries
    pub fn entries(&self) -> &[CentralDirEntry] {
        &self.entries
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if directory is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Serialize directory to bytes
    pub fn to_bytes(&self) -> ZjlResult<Vec<u8>> {
        let mut bytes = Vec::new();
        
        // Write entry count
        bytes.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        
        // Write entries
        for entry in &self.entries {
            bytes.extend_from_slice(entry.as_bytes());
        }
        
        Ok(bytes)
    }

    /// Deserialize directory from bytes
    pub fn from_bytes(data: &[u8]) -> ZjlResult<Self> {
        if data.len() < 4 {
            return Err(ZjlError::InvalidData("Central directory too short".to_string()));
        }
        
        let entry_count = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let entry_size = CentralDirEntry::size();
        let expected_size = 4 + entry_count * entry_size;
        
        if data.len() != expected_size {
            return Err(ZjlError::InvalidData("Central directory size mismatch".to_string()));
        }
        
        let mut directory = Self::new();
        let mut offset = 4;
        
        for _ in 0..entry_count {
            let entry_bytes = &data[offset..offset + entry_size];
            let entry = CentralDirEntry::read_from(entry_bytes)
                .ok_or_else(|| ZjlError::InvalidData("Failed to read central directory entry".to_string()))?;
            directory.add_entry(entry);
            offset += entry_size;
        }
        
        Ok(directory)
    }

    /// Write directory to a writer
    pub fn write_to<W: Write>(&self, writer: &mut W) -> ZjlResult<u64> {
        let bytes = self.to_bytes()?;
        writer.write_all(&bytes)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        Ok(bytes.len() as u64)
    }

    /// Read directory from a reader
    pub fn read_from<R: Read>(reader: &mut R) -> ZjlResult<Self> {
        let mut count_bytes = [0u8; 4];
        reader.read_exact(&mut count_bytes)
            .map_err(|e| ZjlError::IoError(e.to_string()))?;
        
        let entry_count = u32::from_le_bytes(count_bytes) as usize;
        let entry_size = CentralDirEntry::size();
        
        let mut directory = Self::new();
        
        for _ in 0..entry_count {
            let mut entry_bytes = vec![0u8; entry_size];
            reader.read_exact(&mut entry_bytes)
                .map_err(|e| ZjlError::IoError(e.to_string()))?;
            
            let entry = CentralDirEntry::read_from(&entry_bytes)
                .ok_or_else(|| ZjlError::InvalidData("Failed to read central directory entry".to_string()))?;
            directory.add_entry(entry);
        }
        
        Ok(directory)
    }
}

/// B+ tree index for fast lookups
pub struct BPlusTreeIndex {
    /// Root node offset
    root_offset: u64,
    /// Node size
    node_size: usize,
    /// Maximum keys per node
    max_keys: usize,
}

impl BPlusTreeIndex {
    pub fn new(node_size: usize) -> Self {
        // Calculate max keys based on node size
        // Each key is 8 bytes (u64), each pointer is 8 bytes
        // Node header is ~32 bytes
        let available_space = node_size - 32;
        let max_keys = available_space / 16; // 8 bytes key + 8 bytes pointer
        
        Self {
            root_offset: 0,
            node_size,
            max_keys,
        }
    }

    /// Build index from central directory
    pub fn build_from_directory(&mut self, directory: &CentralDirectory) -> ZjlResult<()> {
        // For now, just store the root offset
        // Full B+ tree implementation would go here
        self.root_offset = 0;
        Ok(())
    }

    /// Find entries by key range
    pub fn find_range(&self, _start_key: u64, _end_key: u64) -> ZjlResult<Vec<u64>> {
        // B+ tree range query would go here
        Ok(Vec::new())
    }

    /// Get root offset
    pub fn root_offset(&self) -> u64 {
        self.root_offset
    }
}

/// File layout manager
pub struct FileLayout {
    /// Fixed header size
    pub header_size: u64,
    /// Heap arena start
    pub heap_start: u64,
    /// Central directory offset
    pub central_dir_offset: u64,
    /// B+ tree index offset
    pub index_offset: u64,
    /// Signatures offset
    pub signatures_offset: u64,
    /// Total file size
    pub file_size: u64,
}

impl FileLayout {
    pub fn new() -> Self {
        Self {
            header_size: 160, // Fixed header size
            heap_start: 160,
            central_dir_offset: 0,
            index_offset: 0,
            signatures_offset: 0,
            file_size: 160,
        }
    }

    /// Update layout after writing heap data
    pub fn update_after_heap(&mut self, heap_size: u64) {
        self.central_dir_offset = self.heap_start + heap_size;
        self.file_size = self.central_dir_offset;
    }

    /// Update layout after writing central directory
    pub fn update_after_central_dir(&mut self, central_dir_size: u64) {
        self.index_offset = self.central_dir_offset + central_dir_size;
        self.file_size = self.index_offset;
    }

    /// Update layout after writing index
    pub fn update_after_index(&mut self, index_size: u64) {
        self.signatures_offset = self.index_offset + index_size;
        self.file_size = self.signatures_offset;
    }

    /// Update layout after writing signatures
    pub fn update_after_signatures(&mut self, signatures_size: u64) {
        self.file_size = self.signatures_offset + signatures_size;
    }

    /// Update fixed header with layout information
    pub fn update_header(&self, header: &mut FixedHeader) {
        header.central_dir_offset = self.central_dir_offset;
        header.root_index_offset = self.index_offset;
        header.signatures_offset = self.signatures_offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::BlockHeader;

    #[test]
    fn test_heap_arena() {
        let mut arena = HeapArena::new(1000);
        
        // Allocate some blocks
        let offset1 = arena.allocate(100, BlockType::JsonObject, 1);
        let offset2 = arena.allocate(200, BlockType::JsonArray, 2);
        
        assert_eq!(offset1, 1000);
        assert_eq!(offset2, 1100);
        assert_eq!(arena.size(), 1300);
        
        // Free first block
        arena.free(offset1).unwrap();
        
        // Allocate smaller block - should reuse freed space
        let offset3 = arena.allocate(50, BlockType::JsonString, 3);
        assert_eq!(offset3, 1000); // Reused freed space
    }

    #[test]
    fn test_central_directory() {
        let mut dir = CentralDirectory::new();
        
        let entry1 = CentralDirEntry::new(
            1000,
            BlockType::JsonObject,
            1,
            [1u8; 32],
            100,
            150,
        );
        
        let entry2 = CentralDirEntry::new(
            2000,
            BlockType::JsonArray,
            1,
            [2u8; 32],
            200,
            250,
        );
        
        dir.add_entry(entry1);
        dir.add_entry(entry2);
        
        assert_eq!(dir.len(), 2);
        
        // Find by path ID
        let path_entries = dir.find_by_path(1);
        assert_eq!(path_entries.len(), 2);
        
        // Find by type
        let obj_entries = dir.find_by_type(BlockType::JsonObject);
        assert_eq!(obj_entries.len(), 1);
    }

    #[test]
    fn test_central_directory_serialization() {
        let mut dir = CentralDirectory::new();
        
        let entry = CentralDirEntry::new(
            1000,
            BlockType::JsonObject,
            1,
            [1u8; 32],
            100,
            150,
        );
        dir.add_entry(entry);
        
        // Serialize
        let bytes = dir.to_bytes().unwrap();
        
        // Deserialize
        let dir2 = CentralDirectory::from_bytes(&bytes).unwrap();
        assert_eq!(dir2.len(), 1);
        let offset = dir2.entries()[0].offset;
        assert_eq!(offset, 1000);
    }

    #[test]
    fn test_file_layout() {
        let mut layout = FileLayout::new();
        assert_eq!(layout.header_size, 160);
        assert_eq!(layout.heap_start, 160);
        
        layout.update_after_heap(1000);
        assert_eq!(layout.central_dir_offset, 1160);
        
        layout.update_after_central_dir(200);
        assert_eq!(layout.index_offset, 1360);
        
        layout.update_after_index(100);
        assert_eq!(layout.signatures_offset, 1460);
        
        layout.update_after_signatures(300);
        assert_eq!(layout.file_size, 1760);
    }
}
