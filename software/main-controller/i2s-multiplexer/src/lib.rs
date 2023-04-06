#![no_std]
//#![allow(dead_code, non_camel_case_types)]
#![allow(dead_code)]

use embedded_hal as hal;

use hal::digital::v2::OutputPin;

const NUMBER_CHANNELS_SUPPORTED: u8 = 6;

pub enum Error {
    InvalidSourceChannel,
    EnableError,
    InvalidChannel,
    AddressSetError,
}

pub struct I2SMultiplexer<ADDR0, ADDR1, ADDR2, ENABLE> {
    a0_pin: ADDR0,
    a1_pin: ADDR1,
    a2_pin: ADDR2,
    enable_pin: ENABLE,
}

impl<ADDR0, ADDR1, ADDR2, ENABLE> I2SMultiplexer<ADDR0, ADDR1, ADDR2, ENABLE>
where
    ADDR0: OutputPin,
    ADDR1: OutputPin,
    ADDR2: OutputPin,
    ENABLE: OutputPin,
{
    pub fn new(
        a0_pin: ADDR0,
        a1_pin: ADDR1,
        a2_pin: ADDR2,
        enable_pin: ENABLE,
    ) -> Result<Self, Error> {
        let i2s_multiplexer = I2SMultiplexer {
            a0_pin,
            a1_pin,
            a2_pin,
            enable_pin,
        };

        Ok(i2s_multiplexer)
    }

    pub fn set_source(&mut self, source_channel: u8) -> Result<(), Error> {
        if source_channel >= NUMBER_CHANNELS_SUPPORTED {
            return Err(Error::InvalidChannel);
        }

        self.enable_pin.set_low().map_err(|_| Error::EnableError)?;

        self.a0_pin.set_low().map_err(|_| Error::AddressSetError)?;
        self.a1_pin.set_low().map_err(|_| Error::AddressSetError)?;
        self.a2_pin.set_low().map_err(|_| Error::AddressSetError)?;

        if source_channel & (1) != 0 {
            self.a0_pin.set_high().map_err(|_| Error::AddressSetError)?;
        } else {
            self.a0_pin.set_low().map_err(|_| Error::AddressSetError)?;
        };

        if source_channel & (1 << 1) != 0 {
            self.a1_pin.set_high().map_err(|_| Error::AddressSetError)?;
        } else {
            self.a1_pin.set_low().map_err(|_| Error::AddressSetError)?;
        };

        if source_channel & (1 << 2) != 0 {
            self.a2_pin.set_high().map_err(|_| Error::AddressSetError)?;
        } else {
            self.a2_pin.set_low().map_err(|_| Error::AddressSetError)?;
        };

        self.enable_pin.set_high().map_err(|_| Error::EnableError)?;

        Ok(())
    }
}
