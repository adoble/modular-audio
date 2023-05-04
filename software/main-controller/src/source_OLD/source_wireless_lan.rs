use crate::channel::Channel;
use crate::source::{DisplayPosition, Source, SourceError}; // TODO this will be somewhere else (i2smultiplexer?) during integration

#[derive(Debug)]
pub struct SourceWirelessLan {
    channel: Channel,
    display_position: DisplayPosition,
}

impl SourceWirelessLan {
    pub fn new(channel: Channel, display_position: DisplayPosition) -> Result<Self, SourceError> {
        if channel.verify() {
            let source = SourceWirelessLan {
                channel,
                display_position,
            };
            Ok(source)
        } else {
            Err(SourceError::IncorrectChannel)
        }
    }
}

impl Source for SourceWirelessLan {
    fn activate(&self) -> Result<(), SourceError> {
        defmt::info!("Activating source: Wireless LAN");
        //Err(SourceError::NotImplemented)
        Ok(())
    }

    fn channel(&self) -> Channel {
        self.channel
    }

    fn display_position(&self) -> DisplayPosition {
        self.display_position
    }
}
