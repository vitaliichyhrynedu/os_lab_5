use crate::kernel::{
    Kernel,
    file::{FileDescriptor, FileStats},
    fs::transaction,
};

impl Kernel {
    /// Creates a file with a given name, if it doesn't exist.
    pub fn create(&mut self, name: &str) -> Result<()> {
        todo!()
    }

    /// Opens the file specified with `name`, returning a corresponding file descriptor.
    pub fn open(&mut self, name: &str) -> Result<FileDescriptor> {
        todo!()
    }

    /// Close the file descriptor referenced by `fd`.
    pub fn close(&mut self, fd: FileDescriptor) -> Result<()> {
        todo!()
    }

    /// Reposition the offset of the file descriptor referenced by `fd`.
    pub fn seek(&mut self, fd: FileDescriptor, offset: usize) -> Result<()> {
        todo!()
    }

    /// Reads up to `buf.len()` bytes into `buf` from the file referenced by `fd`.
    /// Returns the number of bytes read.
    pub fn read(&mut self, fd: FileDescriptor, buf: &mut [u8]) -> Result<usize> {
        todo!()
    }

    /// Writes up to `buf.len()` bytes from `buf` to the file referenced by `fd`.
    /// Returns the number of bytes written.
    pub fn write(&mut self, fd: FileDescriptor, buf: &[u8]) -> Result<usize> {
        todo!()
    }

    /// Creates a hard link `new_name` to an existing file referenced by `old_name`.
    pub fn link(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        todo!()
    }

    /// Removes a hard link referenced by `name` from the filesystem.
    /// If `name` was the last link to reference the file, it is deleted.
    /// If `name` is currently opened, it is deleted after it's closed.
    pub fn unlink(&mut self, name: &str) -> Result<()> {
        todo!()
    }

    /// Truncates the file referenced by `name` to be truncated to a size of `length` bytes.
    pub fn truncate(&mut self, name: &str, length: usize) -> Result<()> {
        todo!()
    }

    /// Returns statistics about a file referenced by `name`.
    pub fn stat(&mut self, name: &str) -> Result<FileStats> {
        todo!()
    }

    /// Returns the list of hard links and corresponding node indeces in the root directory.
    pub fn ls(&mut self) -> Result<Vec<(String, usize)>> {
        todo!()
    }

    /// Formats the storage device with a filesystem capable of handling `count` nodes.
    pub fn mkfs(&mut self, count: usize) -> Result<()> {
        todo!()
    }

    /// Mounts the filesystem.
    pub fn mount(&mut self) -> Result<()> {
        todo!()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Filesystem(transaction::Error),
    InvalidFileDescriptor,
    FilesystemNotMounted,
}
