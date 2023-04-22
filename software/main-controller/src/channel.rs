// TODO this should be in the i2smultiplexer driver
pub const NUMBER_SUPPORTED_CHANNELS: u8 = 6;

#[derive(Copy, Clone, Debug)]
pub struct Channel(pub u8);

impl Channel {
    pub fn verify(&self) -> bool {
        self.0 < NUMBER_SUPPORTED_CHANNELS
    }
}
