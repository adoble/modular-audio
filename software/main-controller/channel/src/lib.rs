#![cfg_attr(not(test), no_std)]

pub const NUMBER_SUPPORTED_CHANNELS: u8 = 6; // TODO should this be in the i2smultiplexer driver?

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Channel(pub u8); //TODO Why do I need the u8 to be public?

impl Channel {
    pub fn new(channel_number: u8) -> Result<Self, ChannelError> {
        if channel_number < NUMBER_SUPPORTED_CHANNELS {
            Ok(Channel(channel_number))
        } else {
            Err(ChannelError::Invalid)
        }
    }

    // pub fn verify(&self) -> bool {
    //     self.0 < NUMBER_SUPPORTED_CHANNELS
    // }

    pub fn channel_number(&self) -> u8 {
        self.0
    }
}

// impl TryFrom<u8> for Channel {
//     type Error = ChannelError;

//     fn try_from(channel_value: u8) -> Result<Self, Self::Error> {
//         Self::new(channel_value)
//     }
// }

pub enum ChannelError {
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try() {
        assert!(true);
    }
}
