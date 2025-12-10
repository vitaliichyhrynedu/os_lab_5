use crate::hardware::storage::Storage;

pub mod fs;

/// A model for the kernel.
pub struct Kernel {
    storage: Storage,
}

impl Kernel {
    /// Constructs a [Kernel].
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
