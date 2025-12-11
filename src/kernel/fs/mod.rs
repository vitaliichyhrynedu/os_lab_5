use crate::{
    hardware::storage::Storage,
    kernel::fs::{
        alloc_map::AllocMap, directory::Directory, superblock::Superblock, transaction::Transaction,
    },
};

pub mod alloc_map;
pub mod directory;
pub mod node;
pub mod superblock;
pub mod transaction;

/// Root directory's node index.
pub const ROOT_INDEX: usize = 1;

/// An in-memory view of the filesystem.
pub struct FileSystem {
    superblock: Superblock,
    block_map: AllocMap,
    node_map: AllocMap,
}

impl FileSystem {
    /// Formats the persistent storage with a filesystem.
    ///
    /// # Panics
    /// ...
    pub fn format(storage: &mut Storage, block_count: usize, node_count: usize) -> Self {
        // Superblock
        let superblock = Superblock::new(block_count, node_count);

        // Allocation maps
        let mut block_map = AllocMap::new(block_count);
        let mut node_map = AllocMap::new(node_count);

        // Allocate metadata regions
        block_map
            .allocate_span((0, superblock.data_offset))
            .expect("'0'..'superblock.data_offset' blocks must not be allocated");

        // Allocate the null node
        node_map
            .allocate_at(0)
            .expect("'0'th node must not be allocated");

        // Create filesystem
        let mut fs = FileSystem {
            superblock,
            block_map,
            node_map,
        };

        // Initialize the root directory
        {
            let mut tx = Transaction::new(&mut fs, storage);
            let root_index = tx
                .create_node()
                .expect("Must be able to create the root node");
            assert!(root_index == ROOT_INDEX);
            let root = Directory::new(root_index, root_index);
            tx.write_directory(root_index, &root)
                .expect("Must be able to write the root directory");
            tx.commit();
        }

        fs
    }
}
