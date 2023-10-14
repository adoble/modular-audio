use crate::channel::Channel;
use crate::source::{DisplayPosition, Source, SourceError}; // TODO this will be somewhere else (i2smultiplexer?) during integration

#[derive(Debug)]
pub struct SourceBluetooth {
    channel: Channel,
    display_position: DisplayPosition,
}

impl SourceBluetooth {
    pub fn new(channel: Channel, display_position: DisplayPosition) -> Result<Self, SourceError> {
        if channel.verify() {
            let source = SourceBluetooth {
                channel,
                display_position,
            };
            Ok(source)
        } else {
            Err(SourceError::IncorrectChannel)
        }
    }
}

impl Source for SourceBluetooth {
    fn activate(&self) -> Result<(), SourceError> {
        defmt::info!("Activating Source: Bluettooth");
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
