extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use crate::source::Source;

//TODO  => new type pattern
pub struct Sources {
    sources: Vec<Box<dyn Source>>,
}

impl Sources {
    /// Constructs a new `AllSources` object

    pub fn new() -> Self {
        //TODO this should take the size of the array as parameter
        Sources {
            sources: Vec::new(),
        }
    }

    /// Adds a new source
    pub fn add(&mut self, source: Box<dyn Source>) {
        self.sources.push(source);

        self.sources.sort_by(|a, b| {
            let a_pos = a.display_position().0;
            let b_pos = b.display_position().0;
            a_pos.cmp(&b_pos)
        });
    }

    pub fn len(&self) -> u8 {
        self.sources.len() as u8
    }
}

impl Default for Sources {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Sources {
    type Output = Box<dyn Source>; // The type of the array value

    fn index(&self, index: usize) -> &Self::Output {
        &self.sources[index]
    }
}

impl IndexMut<usize> for Sources {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sources[index]
    }
}

// See https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
impl<'a> IntoIterator for &'a Sources {
    type Item = &'a Box<dyn Source>;
    type IntoIter = SourceInterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SourceInterator {
            index: self.len() - 1, // Set to the last position so that next() will return the first element
            sources: self,
        }
    }
}

//  See https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
pub struct SourceInterator<'a> {
    index: u8,
    sources: &'a Sources,
}

impl<'a> SourceInterator<'a> {
    pub fn peek(&self) -> Option<&'a Box<dyn Source>> {
        if self.sources.len() == 0 {
            return None;
        }

        Some(&self.sources[self.index as usize])
    }
}

impl<'a> Iterator for SourceInterator<'a> {
    type Item = &'a Box<dyn Source>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sources.len() == 0 {
            return None;
        }

        self.index = (self.index + 1) % self.sources.len();
        Some(&self.sources[self.index as usize])
    }
}
// Required as possibly shared between threads.
// See https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for SourceInterator<'_> {}
unsafe impl Sync for SourceInterator<'_> {}

// Required as possibly shared between threads.
// See https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for Sources {}
unsafe impl Sync for Sources {}
