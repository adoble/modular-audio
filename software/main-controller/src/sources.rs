extern crate alloc;

//use alloc::boxed::Box;
use alloc::vec::Vec;
//use core::ops::{Index, IndexMut};

use crate::channel::Channel;

pub enum Source {
    Bluetooth(SourceConfig),
    WirelessLan(SourceConfig),
    Cd(SourceConfig),
    InternetRadio(SourceConfig),
    DabRadio(SourceConfig),
    Aux(SourceConfig),
}

impl Source {
    pub fn display_position(&self) -> DisplayPosition {
        match self {
            Source::Bluetooth(config) => config.display_position,
            Source::WirelessLan(config) => config.display_position,
            Source::Cd(config) => config.display_position,
            Source::InternetRadio(config) => config.display_position,
            Source::DabRadio(config) => config.display_position,
            Source::Aux(config) => config.display_position,
        }
    }

    pub fn channel(&self) -> Channel {
        match self {
            Source::Bluetooth(config) => config.channel,
            Source::WirelessLan(config) => config.channel,
            Source::Cd(config) => config.channel,
            Source::InternetRadio(config) => config.channel,
            Source::DabRadio(config) => config.channel,
            Source::Aux(config) => config.channel,
        }
    }
}

pub struct SourceConfig {
    pub channel: Channel,
    pub display_position: DisplayPosition,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayPosition(pub u8);

impl From<u8> for DisplayPosition {
    fn from(index: u8) -> Self {
        DisplayPosition(index)
    }
}

impl From<DisplayPosition> for u8 {
    fn from(dp: DisplayPosition) -> Self {
        dp.0
    }
}

pub struct Sources(Vec<Source>);

impl Sources {
    pub fn new() -> Self {
        Sources(Vec::new())
    }

    pub fn insert(&mut self, source: Source) {
        self.0.push(source);

        // Sort the entries by display position
        self.0.sort_by(|a, b| {
            let a_pos = a.display_position().0;
            let b_pos = b.display_position().0;
            a_pos.cmp(&b_pos)
        });
    }

    pub fn iter(&self) -> SourceIterator<'_> {
        SourceIterator {
            index: 0,
            sources: self,
        }
    }
}

pub struct SourceIterator<'a> {
    index: usize,
    sources: &'a Sources,
}

impl<'a> SourceIterator<'a> {
    pub fn peek(&self) -> Option<&Source> {
        if self.sources.0.len() == 0 {
            return None;
        }

        Some(&self.sources.0[self.index as usize])
    }
}

impl<'a> Iterator for SourceIterator<'a> {
    type Item = &'a Source;

    /// Cyclically interate over the sources that have been inserted. The order is determined
    /// by the display position.
    fn next(&mut self) -> Option<Self::Item> {
        if self.sources.0.is_empty() {
            return None;
        };

        self.index = (self.index + 1) % self.sources.0.len();

        Some(&self.sources.0[self.index])
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_test() {
        assert!(true);
    }
}
