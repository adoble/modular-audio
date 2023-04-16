use core::ops::{Index, IndexMut};

use crate::sources::Source;

//const NUMBER_SOURCES_SUPPORTED: u8 = 6;

pub struct AllSources {
    sources: [Option<dyn Source>; 6],
    selected: usize,
}

impl AllSources {
    /// Constructs a new `AllSources` object with all source entries initialized to `None` values.
    ///  The first entry is selected.
    pub fn new() -> Self {
        AllSources {
            sources: [None; 6],
            selected: 0, // Default to the first item
        }
    }

    /// Returns an Optional reference to the next `Source`. The method uses wraps around to the first
    /// source if the end has been reached.
    /// If an empty (None) entry is found then `next()` returns the next existing entry.
    /// If no entries are present then returns None.
    pub fn next(&self) -> Option<&dyn Source> {
        if self.is_empty() {
            return None;
        }

        match self.sources[(self.selected + 1) % 6] {
            Some(source) => Some(*source),
            None => self.next(),
        }
    }

    /// Returns `true` if no sources have been added.   
    pub fn is_empty(&self) -> bool {
        for entry in self.sources {
            if let Some(e) = entry {
                return false;
            }
        }
        true
    }
}

impl Index<usize> for AllSources {
    type Output = Option<dyn Source>; // The type of the array value

    fn index(&self, index: usize) -> &Self::Output {
        &self.sources[index]
    }
}

impl IndexMut<usize> for AllSources {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sources[index]
    }
}
