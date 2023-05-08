#![allow(dead_code)]

use embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

extern crate alloc;

// The driver for the MCP23017 chip used on the board
use mcp23017::{Polarity, MCP23017};

use defmt as _;
use panic_probe as _;

use crate::sources::{Source, SourceIterator};

// Defines errors being issued by te MCP23017 chip on the source select board
#[derive(Debug, Copy, Clone)]
pub enum MCP23017Errors {
    // TODO can we use the errors from the mcp23017 driver itself?
    InitializationError,
    PinModeInputError(u8),
    InterruptSetupError,
    InterruptPinSetupError(u8),
    PinModeOutputError(u8),
    InterruptPinError,
    DigitalReadError(u8), // Contains the pin number
}
/// Defines errors for the driver
#[derive(Debug, Copy, Clone)]
pub enum Error {
    MCP23017Error(MCP23017Errors),
    ClearLEDError(u8),
    SetLEDError,
    /// Interrupt pin not found
    InterruptPinError,
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
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::InitializationError))?;

        // Set up the interupt logic on the MCP23017 used on the source display processor.
        // The pin connected to the button GPB0 = 8 in the driver logic. This is set to respond to
        // a value other than HIGH (i.e. the button has pulled the signal LOW).
        mcp23017_driver
            .pin_mode(8, mcp23017::PinMode::INPUT)
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::PinModeInputError(8)))?;
        let mirroring = false;
        let open_drain = false;
        mcp23017_driver
            .setup_interrupts(mirroring, open_drain, Polarity::LOW) // Active low interrupt
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::InterruptSetupError))?;
        mcp23017_driver
            .setup_interrupt_pin(8, mcp23017::InterruptMode::CHANGE) // Using CHANGE as this gives a pulse when the pin changes state
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::InterruptPinSetupError(8)))?;

        // The pins driving the leds are set to output mode and all but the
        // first one cleared.
        for i in 0..NUMBER_CHANNELS_SUPPORTED {
            mcp23017_driver
                .pin_mode(i, mcp23017::PinMode::OUTPUT)
                .map_err(|_| Error::MCP23017Error(MCP23017Errors::PinModeOutputError(i)))?;

            if i == 0 {
                mcp23017_driver
                    .digital_write(i, true)
                    .map_err(|_| Error::ClearLEDError(i))?;
            } else {
                mcp23017_driver
                    .digital_write(i, false)
                    .map_err(|_| Error::ClearLEDError(i))?;
            }
        }

        Ok(SourceSelectDriver {
            address_offset,
            mcp23017_driver,
        })
    }

    /// This should always be called when the source selector has delivered a interupt SUGGESTING that
    /// the source has been changed by the used. If the button has been pressed then this actually means
    /// that the user has changed the source. In this case thre function returns with the new source.
    ///
    /// If the button is released then an interrupt is also issued, but no source changehas happeded. In
    /// which case None is returned.  
    /// the newly selected source.
    ///  
    /// Example   TODO update this comment
    /// ```
    ///   match select_source_driver.changed_source(&sources_iterator).unwrap() {
    ///     Some(new_source) => // ... Activate the new source source...
    ///     None => ();   
    ///   }
    /// ```
    /// Alternatively:
    /// ```
    ///   if let Some(new_source) = select_source_driver.changed_source(&sources_iterator).unwrap() {
    ///     // ... Activate the new source ...
    ///   }
    /// ```
    ///
    // pub fn changed_source<A: SourceActivation>(
    //     &mut self,
    //     current_source: Source<A>,
    // ) -> Result<Option<Source<A>>, Error> {
    pub fn changed_source<'a>(
        &mut self,
        sources_iter: &'a mut SourceIterator,
    ) -> Result<Option<&'a Source>, Error> {
        defmt::debug!("Entering changed_source");

        // This code works. Maybe this shows that we can have an external driver encapsulated with another, higher level,
        // function driver and only the high level driver needs to be locked!!! This is a TODO
        // self.mcp23017_driver
        //     .digital_write(2, true)
        //     .unwrap_or_else(|_| defmt::panic!("Error Here"));

        // Now check the state of the pin causing the interrupt. If the button
        // is being pressed then this will be False. If the button is being
        // released then this will be True. This is debounces the source select
        // button press.
        // IMPORTANT: This will also clear the interrrupt. This is essential
        // for the operation.
        let intr_pin = self
            .mcp23017_driver
            .get_last_interrupt_pin()
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::InterruptPinError))?;

        let state = self
            .mcp23017_driver
            .digital_read(intr_pin)
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::DigitalReadError(intr_pin)))?;

        // Is the button causing the interrupt being pressed or released.
        // if the button is being released then do nothing
        let pressed = !state;

        if pressed {
            // Get the pin number of the current source. The circuit is such that
            // the pin number corresponds to display position of the source in
            // sources.
            defmt::debug!("Button pressed");
            if let Some(current_source) = sources_iter.peek() {
                let led_pin_number: u8 = current_source.display_position().into();
                // Clear the current source led
                self.mcp23017_driver
                    .digital_write(led_pin_number, false)
                    .map_err(|_| Error::ClearLEDError(led_pin_number))?;

                // Update the source
                if let Some(new_source) = sources_iter.next() {
                    let led_pin_number: u8 = new_source.display_position().into();
                    // Now set the LED associated with the source
                    self.mcp23017_driver
                        .digital_write(led_pin_number, true)
                        .map_err(|_| Error::SetLEDError)?;
                    Ok(Some(new_source))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
