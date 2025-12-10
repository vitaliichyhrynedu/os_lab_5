use std::collections::HashMap;

use zerocopy::{FromBytes, IntoBytes};

use crate::{
    hardware::storage::{
        Storage,
        block::{BLOCK_SIZE, Block},
    },
    kernel::fs::{
        node::{NODE_SIZE, NODES_PER_BLOCK, Node},
        superblock::Superblock,
    },
};

/// Represents a filesystem operation that buffers changes in memory before commiting them to
/// persistent storage.
pub struct Transaction<'a> {
    superblock: &'a Superblock,
    storage: &'a mut Storage,
    changes: HashMap<usize, Block>,
}

impl<'a> Transaction<'a> {
    /// Constructs a [Transaction] for the given superblock and storage.
    pub fn new(superblock: &'a Superblock, storage: &'a mut Storage) -> Self {
        Self {
            superblock,
            storage,
            changes: HashMap::new(),
        }
    }

    /// Commits the transaction to persistent storage, consuming the transaction.
    pub fn commit(self) {
        for (&index, block) in self.changes.iter() {
            self.storage
                .write_block(index, block)
                .expect("'index' must be a valid block index")
        }
    }

    /// Reads the node at a given index from the node table.
    pub fn read_node(&self, index: usize) -> Node {
        assert!(
            index < self.superblock.node_count,
            "Index {} exceeds node count {}",
            index,
            self.superblock.node_count
        );
        let block_index = self.get_node_block_index(index);
        let block = self.read_block(block_index);
        let byte_offset = Self::get_node_byte_offset(index);
        Node::read_from_bytes(&block.data[byte_offset..(byte_offset + NODE_SIZE)])
            .expect("'bytes' must have length 'NODE_SIZE'")
    }

    // Writes the node at a given index to the node table.
    pub fn write_node(&mut self, index: usize, node: Node) {
        assert!(
            index < self.superblock.node_count,
            "Index {} exceeds node count {}",
            index,
            self.superblock.node_count
        );
        let block_index = self.get_node_block_index(index);
        let mut block = self.read_block(block_index);
        let byte_offset = Self::get_node_byte_offset(index);
        block.data[byte_offset..(byte_offset + NODE_SIZE)].copy_from_slice(node.as_bytes());
        self.changes.insert(block_index, block);
    }

    /// Reads the block at a given index.
    fn read_block(&self, index: usize) -> Block {
        // Check cached changes
        match self.changes.get(&index) {
            Some(block) => *block,
            None => self
                .storage
                .read_block(index)
                .expect("'block_index' must be a valid block index"),
        }
    }

    /// Returns the index of the block in which the node resides.
    fn get_node_block_index(&self, index: usize) -> usize {
        self.superblock.node_table_offset + (index * NODE_SIZE / BLOCK_SIZE)
    }

    /// Returns the byte offset of the node within the block.
    fn get_node_byte_offset(index: usize) -> usize {
        index % NODES_PER_BLOCK * NODE_SIZE
    }
}
