/// Used to index the open file table.
pub type FileDescriptorNumber = usize;

/// A unique handle to a file.
pub struct FileDescriptor {
    node_index: usize,
    pub offset: usize,
}

impl FileDescriptor {
    /// Creates a new [FileDescriptor] for the file.
    pub fn new(node_index: usize) -> Self {
        Self {
            node_index,
            offset: 0,
        }
    }

    pub fn node_index(&self) -> usize {
        self.node_index
    }
}
