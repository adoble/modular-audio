use crate::channel::Channel;
use crate::source::{DisplayPosition, Source, SourceError}; // TODO this will be somewhere else (i2smultiplexer?) during integration

#[derive(Debug)]
pub struct SourceCd {
    channel: Channel,
    display_position: DisplayPosition,
}

impl SourceCd {
    pub fn new(channel: Channel, display_position: DisplayPosition) -> Result<Self, SourceError> {
        if channel.verify() {
            let source = SourceCd {
                channel,
                display_position,
            };
            Ok(source)
        } else {
            Err(SourceError::IncorrectChannel)
        }
    }
}

impl Source for SourceCd {
    fn activate(&self) -> Result<(), SourceError> {
        //TODO
        defmt::info!("Activating source: CD");
        Ok(())
    }

    fn channel(&self) -> Channel {
        self.channel
    }

    fn display_position(&self) -> DisplayPosition {
        self.display_position
    }
}
