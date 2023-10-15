#![cfg_attr(not(test), no_std)]

// TODO:
// - remove the seperate sources iterator. This is giving problems with the static scope of the sources.

extern crate alloc;

//use core::iter::Cycle;

use alloc::vec::Vec;

use channel::Channel;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

/// A list of sources that can be cyclically iterated through.
pub struct Sources {
    /// The list of sources.
    sources: Vec<Source>,

    /// An index into the current source
    current_source_index: Option<usize>,
}

impl Sources {
    /// Initializes a new `Sources` instance from the given array.
    pub fn from_array(array: &[Source]) -> Self {
        let mut sources = array.to_vec();

        // Sort the entries by display position
        sources.sort_by(|a, b| {
            let a_pos = a.display_position().0;
            let b_pos = b.display_position().0;
            a_pos.cmp(&b_pos)
        });

        Sources {
            sources,
            current_source_index: None,
        }
    }

    // pub fn insert(&mut self, source: Source) {
    //     self.sources.push(source);

    //     // Sort the entries by display position
    //     self.sources.sort_by(|a, b| {
    //         let a_pos = a.display_position().0;
    //         let b_pos = b.display_position().0;
    //         a_pos.cmp(&b_pos)
    //     });
    // }

    /// Advances the iterator to the next source and returns it.
    pub fn next(&mut self) -> Option<Source> {
        // Instead of implementing the Iteration trait, this -  primitive -
        // implementation of iteration is done so that an extra iterator is
        // not needed with the attendent difficultes of guarding it in an
        // asynchrounous environment.
        // TODO is there a async iterator??
        if self.sources.is_empty() {
            return None;
        }

        let index = match self.current_source_index {
            // Not initialised
            None => 0,
            Some(index) => (index + 1) % self.sources.len(),
        };
        let next_source = self.sources[index];
        self.current_source_index = Some(index);
        Some(next_source)
    }

    /// Returns the current source selected over the iterator.
    pub fn current_source(&mut self) -> Option<Source> {
        if self.sources.is_empty() {
            return None;
        }

        let index = match self.current_source_index {
            None => 0,
            Some(index) => index,
        };
        self.current_source_index = Some(index);
        Some(self.sources[index])
    }
}

// pub struct SourceIterator<'a> {
//     index: usize,
//     sources: &'a Sources,
// }

// impl<'a> SourceIterator<'a> {
//     pub fn peek(&self) -> Option<&Source> {
//         if self.sources.0.len() == 0 {
//             return None;
//         }

//         Some(&self.sources.0[self.index as usize])
//     }
// }

// impl<'a> Iterator for SourceIterator<'a> {
//     type Item = &'a Source;

//     /// Cyclically interate over the sources that have been inserted. The order is determined
//     /// by the display position.
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.sources.0.is_empty() {
//             return None;
//         };

//         self.index = (self.index + 1) % self.sources.0.len();

//         Some(&self.sources.0[self.index])
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn create_empty() {
    //     let mut sources = Sources::new();

    //     assert_eq!(sources.next(), None);
    // }

    #[test]
    fn create_from_array() {
        let source_bluetooth = Source::Bluetooth(SourceConfig {
            channel: Channel(1),
            display_position: DisplayPosition(0),
        });

        let source_wlan = Source::WirelessLan(SourceConfig {
            channel: Channel(2),
            display_position: DisplayPosition(1),
        });

        let source_cd = Source::Cd(SourceConfig {
            channel: Channel(3),
            display_position: DisplayPosition(2),
        });

        let mut sources = Sources::from_array(&[source_bluetooth, source_wlan, source_cd]);

        assert_eq!(sources.next().unwrap(), source_bluetooth);
        assert_eq!(sources.next().unwrap(), source_wlan);
        assert_eq!(sources.next().unwrap(), source_cd);
    }

    #[test]
    fn order() {
        let source_bluetooth = Source::Bluetooth(SourceConfig {
            channel: Channel(1),
            display_position: DisplayPosition(0),
        });

        let source_wlan = Source::WirelessLan(SourceConfig {
            channel: Channel(2),
            display_position: DisplayPosition(3),
        });

        let source_cd = Source::Cd(SourceConfig {
            channel: Channel(3),
            display_position: DisplayPosition(1),
        });

        let mut sources = Sources::from_array(&[source_bluetooth, source_wlan, source_cd]);

        // Different order
        assert_eq!(sources.next().unwrap(), source_bluetooth);
        assert_eq!(sources.next().unwrap(), source_cd);
        assert_eq!(sources.next().unwrap(), source_wlan);
    }

    #[test]
    fn wraparound() {
        let source_bluetooth = Source::Bluetooth(SourceConfig {
            channel: Channel(1),
            display_position: DisplayPosition(0),
        });

        let source_wlan = Source::WirelessLan(SourceConfig {
            channel: Channel(2),
            display_position: DisplayPosition(2),
        });

        let source_cd = Source::Cd(SourceConfig {
            channel: Channel(3),
            display_position: DisplayPosition(5),
        });

        let mut sources = Sources::from_array(&[source_bluetooth, source_wlan, source_cd]);

        // Wrap around
        assert_eq!(sources.next().unwrap(), source_bluetooth);
        assert_eq!(sources.next().unwrap(), source_wlan);
        assert_eq!(sources.next().unwrap(), source_cd);
        assert_eq!(sources.next().unwrap(), source_bluetooth);
        assert_eq!(sources.next().unwrap(), source_wlan);
        assert_eq!(sources.next().unwrap(), source_cd);
        assert_eq!(sources.next().unwrap(), source_bluetooth);
        assert_eq!(sources.next().unwrap(), source_wlan);
        assert_eq!(sources.next().unwrap(), source_cd);
    }

    #[test]
    fn current_source() {
        let source_bluetooth = Source::Bluetooth(SourceConfig {
            channel: Channel(1),
            display_position: DisplayPosition(0),
        });

        let source_wlan = Source::WirelessLan(SourceConfig {
            channel: Channel(2),
            display_position: DisplayPosition(1),
        });

        let source_cd = Source::Cd(SourceConfig {
            channel: Channel(3),
            display_position: DisplayPosition(2),
        });

        let mut sources = Sources::from_array(&[source_bluetooth, source_wlan, source_cd]);

        assert_eq!(sources.current_source().unwrap(), source_bluetooth);
        sources.next();
        assert_eq!(sources.current_source().unwrap(), source_wlan);
        sources.next();
        assert_eq!(sources.current_source().unwrap(), source_cd);
        sources.next();
        assert_eq!(sources.current_source().unwrap(), source_bluetooth);
    }

    #[test]
    fn empty_init() {
        let mut sources = Sources::from_array(&[]);

        assert_eq!(sources.current_source(), None);

        assert_eq!(sources.next(), None);
    }
}
