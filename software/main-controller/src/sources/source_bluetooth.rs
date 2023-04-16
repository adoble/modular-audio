use crate::sources::source::{Source, SourceError};

pub struct SourceBluetooth {
    channel: u8,
}

impl SourceBluetooth {
    pub fn new(channel: u8) -> Result<Self, SourceError> {
        let source = SourceBluetooth { channel };

        if source.channel_validated(channel) {
            return Ok(source);
        } else {
            return Err(SourceError::IncorrectChannel);
        }
    }
}

impl Source for SourceBluetooth {
    fn activate(&self) -> Result<(), SourceError> {
        defmt::info!("Activating source: bluetooth");
        Err(SourceError::NotImplemented)
    }

    fn channel(&self) -> u8 {
        self.channel
    }
}
