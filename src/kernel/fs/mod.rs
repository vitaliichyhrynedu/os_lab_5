use crate::{
    kernel::fs::{alloc_map::AllocMap, superblock::Superblock},
};

pub mod alloc_map;
pub mod directory;
pub mod node;
pub mod superblock;
pub mod transaction;

pub struct FileSystem {
    superblock: Superblock,
    block_map: AllocMap,
    node_map: AllocMap,
}

impl FileSystem {}
