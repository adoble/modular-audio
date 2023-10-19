#![allow(dead_code)]

use embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

extern crate alloc;

// The driver for the MCP23017 chip used on the board
use mcp23017::MCP23017;

use defmt as _;
use panic_probe as _;

use sources::Sources;

// Defines errors being issued by the MCP23017 chip on the source select board
#[derive(Debug, Copy, Clone)]
pub enum MCP23017Errors {
    // TODO can we use the errors from the mcp23017 driver itself?
    Initialization,
    PinModeInput(u8),
    PinModeOutput(u8),
    InterruptPin,
    DigitalRead(u8), // Contains the pin number
}
/// Defines errors for the driver
#[derive(Debug, Copy, Clone)]
pub enum Error {
    MCP23017(MCP23017Errors),
    ClearLED(u8),
    SetLED,
    /// Interrupt pin not found
    InterruptPin,
}

const BASE_ADDRESS: u8 = 0x20;
const NUMBER_CHANNELS_SUPPORTED: u8 = 6;

#[derive(Clone, Copy, Debug)]
// TODO Change name to SourceSelector. Its not really a driver as domain info such as Source is used.
pub struct SourceSelectDriver<I2C: Write + WriteRead>
where
    I2C: WriteRead + Write,
{
    address_offset: u8,
    mcp23017_driver: MCP23017<I2C>,
}

impl<I2C, E> SourceSelectDriver<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C, address_offset: u8) -> Result<SourceSelectDriver<I2C>, Error> {
        let mut mcp23017_driver = mcp23017::MCP23017::new(i2c, BASE_ADDRESS + address_offset)
            .map_err(|_| Error::MCP23017(MCP23017Errors::Initialization))?;

        // The pins driving the leds are set to output mode and all but the
        // first one cleared.
        for i in 0..NUMBER_CHANNELS_SUPPORTED {
            mcp23017_driver
                .pin_mode(i, mcp23017::PinMode::OUTPUT)
                .map_err(|_| Error::MCP23017(MCP23017Errors::PinModeOutput(i)))?;

            if i == 0 {
                mcp23017_driver
                    .digital_write(i, true)
                    .map_err(|_| Error::ClearLED(i))?;
            } else {
                mcp23017_driver
                    .digital_write(i, false)
                    .map_err(|_| Error::ClearLED(i))?;
            }
        }

        Ok(SourceSelectDriver {
            address_offset,
            mcp23017_driver,
        })
    }

    /// TODO
    ///
    pub fn change_source(
        &mut self,
        //sources_iter: &'a mut SourceIterator,
        sources: &mut Sources,
    ) -> Result<(), Error> {
        // TODO try an remove the nested structure here
        defmt::info!("Changing source");
        if let Some(current_source) = sources.current_source() {
            let led_pin_number: u8 = current_source.display_position().into();
            // Clear the current source led
            self.mcp23017_driver
                .digital_write(led_pin_number, false)
                .map_err(|_| Error::ClearLED(led_pin_number))?;

            // Update the source
            if let Some(new_source) = sources.next() {
                let led_pin_number: u8 = new_source.display_position().into();
                // Now set the LED associated with the source
                self.mcp23017_driver
                    .digital_write(led_pin_number, true)
                    .map_err(|_| Error::SetLED)?;
                Ok(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
