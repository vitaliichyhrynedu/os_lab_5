use crate::{
    hardware::storage::Storage,
    kernel::{file::OpenFileTable, fs::Filesystem},
};

pub mod file;
pub mod fs;
pub mod syscall;

/// A model for the kernel.
pub struct Kernel {
    storage: Storage,
    filesystem: Option<Filesystem>,
    open_files: OpenFileTable,
}

impl Kernel {
    /// Constructs a [Kernel].
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            filesystem: None,
            open_files: OpenFileTable::new(),
        }
    }
}
