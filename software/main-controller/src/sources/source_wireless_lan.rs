use crate::sources::source::{Source, SourceError};

pub struct SourceWirelessLan {
    channel: u8,
}

impl SourceWirelessLan {
    pub fn new(channel: u8) -> Result<Self, SourceError> {
        let source = SourceWirelessLan { channel };
        if source.channel_validated(channel) {
            return Ok(source);
        } else {
            return Err(SourceError::IncorrectChannel);
        }
    }
}

impl Source for SourceWirelessLan {
    fn activate(&self) -> Result<(), SourceError> {
        defmt::info!("Activating source: Wireless LAN");
        Err(SourceError::NotImplemented)
    }

    fn channel(&self) -> u8 {
        self.channel
    }
}
