const NUMBER_SUPPORTED_CHANNELS: u8 = 6;
pub trait Source {
    fn activate(&self) -> Result<(), SourceError>;

    // fn validate_channel(&self, channel: u8) -> Result<(), SourceError> {
    //     if channel >= 6 {
    //         return Err(SourceError::IncorrectChannel);
    //     }
    //     Ok(())
    // }
    fn channel_validated(&self, channel: u8) -> bool {
        channel < 6
    }

    /// Get the channel associated with this source
    fn channel(&self) -> u8;
}

#[non_exhaustive]
pub enum SourceError {
    IncorrectChannel,
    ActivationFailed,
    NotImplemented,
}
