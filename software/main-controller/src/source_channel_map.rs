use crate::source::Source;

/// A mapping of sources to channels.
/// Channels are the channels on the i2smultiplexer.
/// Note that more then one source can be mapped to a channel.
///
/// This struct allows mapping sources to channels using the `add_mapping` method and retrieving
/// the channel for a source using the `get_channel` method:
///
/// Example usage:
/// ```
///   let mut map = SourceChannelMap::new();
///   map.add_mapping(Source::Bluetooth, 1);
///   map.add_mapping(Source::Aux, 2);
///   map.add_mapping(Source::CD, 3);
///  
///   assert_eq!(map.get_channel(Source::Bluettooth), Some(1));
///   assert_eq!(map.get_channel(Source::Aux), Some(2));
///   assert_eq!(map.get_channel(Source::CD), Some(3));
///   assert_eq!(map.get_channel(Source::InternetRadio), None);
/// ```
///
pub struct SourceChannelMap {
    mappings: [(Option<Source>, u8); 5],
}

impl SourceChannelMap {
    /// Creates a new `SourceChannelMap` with no mappings.
    pub fn new() -> Self {
        Self {
            mappings: [(None, 0); 5],
        }
    }

    /// Adds a mapping between a source and a channel.
    ///
    /// If a mapping for the source already exists, the previous mapping is overwritten.
    pub fn add_mapping(&mut self, source: Source, channel: u8) {
        for (src, chan) in self.mappings.iter_mut() {
            if let Some(src) = *src {
                if src == source {
                    //Overwrite
                    *chan = channel;
                    return;
                }
            }
        }

        // Here we could not find a match to overwrite, so add a mapping at the first
        // None entry
        for (src, chan) in self.mappings.iter_mut() {
            match *src {
                Some(_) => (),
                None => {
                    *src = Some(source);
                    *chan = channel;
                    return;
                }
            }
        }
    }

    /// Gets the channel for a source.
    ///
    /// Returns `Some(channel)` if a mapping for the source has been added, or `None` if no mapping
    /// has been added for the source.
    pub fn get_channel(&self, source: Source) -> Option<u8> {
        for (src, chan) in self.mappings.iter() {
            match src {
                Some(src) => {
                    if *src == source {
                        return Some(*chan);
                    }
                }
                None => (),
            }
        }
        None
    }
}
