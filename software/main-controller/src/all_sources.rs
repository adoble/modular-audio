extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use crate::sources::Source;

//const NUMBER_SOURCES_SUPPORTED: u8 = 6;

pub struct AllSources {
    selected: Option<usize>,
    //sources: [Option<dyn Source>; 6],
    sources: Vec<Box<dyn Source>>,
}

impl AllSources {
    /// Constructs a new `AllSources` object
    /// No source is selected
    pub fn new() -> Self {
        //TODO this should take the size of the array as parameter
        AllSources {
            sources: Vec::new(),
            selected: None,
        }
    }

    /// Returns a optional box reference to the next `Source`. The method uses wraps around to the first
    /// source if the end has been reached.
    ///
    /// If no entries are present then returns None.
    /// If no source has been selected then returns the first.
    pub fn next(&mut self) -> Option<&Box<dyn Source>> {
        if self.sources.is_empty() {
            return None;
        }

        match self.selected {
            None => {
                self.selected = Some(0);
                self.sources.get(0)
                //&Some(self.sources[0])
            }
            Some(selected_index) => {
                let next_index = (selected_index + 1) % self.sources.len();
                self.selected = Some(next_index);
                self.sources.get(next_index)
                //&Some(self.sources[next_index])
            }
        }
    }

    pub fn len(&self) -> u8 {
        self.sources.len() as u8
    }

    pub fn selected(&self) -> Option<&Box<dyn Source>> {
        match self.selected {
            None => None,
            Some(selected) => self.sources.get(selected),
        }
    }

    /// Returns and optional index (as `u8`) of the selected source.
    pub fn selected_index(&self) -> Option<u8> {
        match self.selected {
            None => None,
            Some(selected_index) => {
                let selected_index_as_u8 = selected_index as u8;
                Some(selected_index_as_u8)
            }
        }
    }
}

impl Index<usize> for AllSources {
    type Output = Box<dyn Source>; // The type of the array value

    fn index(&self, index: usize) -> &Self::Output {
        &self.sources[index]
    }
}

impl IndexMut<usize> for AllSources {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sources[index]
    }
}

// Required as possible shared between threads.
// See https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Send for AllSources {}
unsafe impl Sync for AllSources {}
