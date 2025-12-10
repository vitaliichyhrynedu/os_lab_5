use zerocopy::{IntoBytes, TryFromBytes};

/// Tracks allocation state of objects.
pub struct AllocMap {
    flags: Box<[AllocFlag]>,
}

impl AllocMap {
    /// Constructs a zero-initialized [AllocMap] that represents a list of objects of given count.
    pub fn new(count: usize) -> Self {
        AllocMap {
            flags: vec![AllocFlag::default(); count].into_boxed_slice(),
        }
    }

    /// Tries to find a contigous span of free objects of `count` length, using the first-fit algorithm.
    /// On success, returns a (start, end) tuple, representing an exclusive range of indices.
    fn find_free(&self, count: usize) -> Option<(usize, usize)> {
        if count == 0 {
            return None;
        }
        let mut start = 0;
        for (i, flag) in self.flags.iter().enumerate() {
            if *flag == AllocFlag::Used {
                start = i + 1;
                continue;
            }
            if (i + 1) - start == count {
                return Some((start, i + 1));
            }
        }
        None
    }

    /// Tries to allocate a contigious span of objects of `count` length.
    /// On success, returns a (start, end) tuple, representing an exclusive range of indices.
    pub fn allocate(&mut self, count: usize) -> Result<(usize, usize), Error> {
        let span = self.find_free(count).ok_or(Error::OutOfSpace)?;
        for flag in &mut self.flags[span.0..span.1] {
            *flag = AllocFlag::Used;
        }
        Ok(span)
    }

    /// Marks the specified span of objects as free.
    ///
    /// # Panics
    /// Panics if:
    /// - `span` is not a valid span
    /// - `span` is out of bounds
    pub fn free(&mut self, span: (usize, usize)) {
        assert!(span.0 < span.1);
        assert!(
            span.1 <= self.flags.len(),
            "Span end {} exceeds map length {}",
            span.1,
            self.flags.len()
        );
        for flag in &mut self.flags[span.0..span.1] {
            *flag = AllocFlag::Free;
        }
    }

    /// Returns a view of the allocation map as a slice of [AllocFlag].
    pub fn as_slice(&self) -> &[AllocFlag] {
        &self.flags
    }

    /// Constructs [AllocMap] from a slice of [AllocFlag].
    pub fn from_slice(flags: &[AllocFlag]) -> Self {
        Self {
            flags: Box::from(flags),
        }
    }
}

/// Represents allocation state of an object.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[derive(TryFromBytes, IntoBytes)]
#[repr(u8)]
pub enum AllocFlag {
    #[default]
    Free,
    Used,
}

/// [AllocMap]-related errors.
pub enum Error {
    OutOfSpace,
}
