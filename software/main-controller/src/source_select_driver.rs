//#![no_std]
//#![allow(dead_code, non_camel_case_types)]
#![allow(dead_code)]

use embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

// The driver for the MCP23017 chip used on the board
use mcp23017::{Polarity, MCP23017};

use defmt as _;
use panic_probe as _;

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Bluetooth = 0,
    WirelessLAN = 1,
    CD = 2,
    InternetRadio = 3, //TODO could provide the url as well.
    DABRadio = 4,
    Aux = 5,
}

impl Source {
    pub fn init() -> Self {
        Source::Bluetooth
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Bluetooth => Self::WirelessLAN,
            Self::WirelessLAN => Self::CD,
            Self::CD => Self::InternetRadio,
            Self::InternetRadio => Self::DABRadio,
            Self::DABRadio => Self::Aux,
            Self::Aux => Self::Bluetooth,
        }
    }

    pub fn activate(&self) {
        // TODO
        defmt::info!("Activating source {}", *self as u8)
    }
}

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
    SetLEDError(u8),
    /// Interrupt pin not found
    InterruptPinError,
}

const BASE_ADDRESS: u8 = 0x20;
const NUMBER_CHANNELS_SUPPORTED: u8 = 6;

#[derive(Clone, Copy, Debug)]
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

    /// Example
    /// ```
    ///   match select_source_driver.changed_source(current_source).unwrap() {
    ///     Some(source) => source.activate(),
    ///     None => ();   
    ///   }
    /// ```
    /// Alternatively:
    /// ```
    ///   if let Some(new_source) = select_source_driver.changed_source(current_source).unwrap() {
    ///     new_source.activate()
    ///   }
    /// ```
    ///
    pub fn changed_source(&mut self, current_source: Source) -> Result<Option<Source>, Error> {
        // TODO

        // Clear the interrupt pin on the MCP23017
        let intr_pin = self
            .mcp23017_driver
            .get_last_interrupt_pin()
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::InterruptPinError))?;
        // Now check the state of the pin causing the interrupt. If the button
        // is being pressed then this will be False. If the button is being
        // released then this will be True. This is debounces the source select
        // button press.
        // IMPORTANT: This will also clear the interrrupt. This is essential
        // for the operation.
        let state = self
            .mcp23017_driver
            .digital_read(intr_pin)
            .map_err(|_| Error::MCP23017Error(MCP23017Errors::DigitalReadError(intr_pin)))?;

        // Is the button causing the interrupt being pressed or released.
        // if the button is being released then do nothing
        let pressed = !state;

        if pressed {
            // Clear the current source led
            self.mcp23017_driver
                .digital_write(current_source as u8, false)
                .map_err(|_| Error::ClearLEDError(current_source as u8))?;

            // Update the source
            let new_source = current_source.next();

            // Now set the LED associated with the source
            self.mcp23017_driver
                .digital_write(new_source as u8, true)
                .map_err(|_| Error::SetLEDError(new_source as u8))?;
            Ok(Some(new_source))
        } else {
            Ok(None)
        }
    }

    pub fn set_source(&self, source: Source) -> Result<Source, Error> {
        // TODO
        Ok(source)
    }
}
