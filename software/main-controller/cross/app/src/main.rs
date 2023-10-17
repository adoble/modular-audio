//! Selects a source based on the interruts delivered by the source_select_processor and changes
//! the LEDS on the source_select_processor accordingly.
//!
//! Circuit:
//!     Source select button (pulled up, active low) -> MCP23017::GPB0
//!     MCP23017::GPA[0..5]  -> LEDS for each channel
//!     MCP23017::INTB (change, active low) -> MCU::GPIO1
//!     MCP23017::[SDA, SCL] -> MCU::I2C1[SDA, SCL]
//!     Pulldown Button -> MCU::GPIO1
//!  - Process the interrupt over GPIO 1 (button has been pressed).
//!

// TODOs
// - Maybe need to use shared-bus, see https://docs.rs/shared-bus/latest/shared_bus/ and the referenced blog post.
//
// DEVELOPMENT NOTE:
// - Using https://github.com/lulf/watchful/blob/main/firmware/app/src/main.rs as an example do two things:
//

#![no_std]
#![no_main]
// #![feature(default_alloc_error_handler)] // Stable since 1.68.0 and no longer requires an attribute to enable
#![feature(type_alias_impl_trait)]

// Included modules
mod source_select_driver;

// Imports
use defmt as _;
use defmt_rtt as _;
//#[cfg(feature = "panic-probe")]
use panic_probe as _;

// Required as Sources uses Vec.
extern crate alloc;

use embedded_alloc::Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();

// Emabssy imports.
use embassy_executor::Spawner;
use embassy_rp::gpio; // gpio::peripherals::PIN_1,
use embassy_rp::peripherals::{PIN_1, PIN_10, PIN_11, PIN_12, PIN_13};
use embassy_time::{Duration, Timer};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex; // Is this the right one? TODO - Using an async mutex

use embassy_rp::i2c::{self, Blocking, Config, I2c};
use embassy_rp::peripherals::I2C0;
use gpio::{Input, Level, Output, Pull};

// Controller specific crates
use crate::source_select_driver::SourceSelectDriver;
use i2s_multiplexer::I2SMultiplexer;

// Data structures used
use channel::Channel;
use sources::{DisplayPosition, Source, SourceConfig, Sources};

// The driver types are omplicated and need to be explictly set for the shared variable.
// To keep this more more manageble a number of types are defined here.

// Pin Types
type MuxAddr0 = Output<'static, PIN_10>;
type MuxAddr1 = Output<'static, PIN_11>;
type MuxAddr2 = Output<'static, PIN_12>;
type MuxEnable = Output<'static, PIN_13>;

// Driver types
type MultiplexerDriver = I2SMultiplexer<MuxAddr0, MuxAddr1, MuxAddr2, MuxEnable>;

// I2C type
type I2CBus = I2c<'static, I2C0, Blocking>;

// Shared resources
// Using the async mutex type as the structures are shared across aynch tasks.
static SHARED_SOURCES: Mutex<CriticalSectionRawMutex, Option<Sources>> = Mutex::new(None);
static SHARED_I2S_MULTIPLEXER: Mutex<CriticalSectionRawMutex, Option<MultiplexerDriver>> =
    Mutex::new(None);
static SHARED_SOURCE_SELECTION_DRIVER: Mutex<
    CriticalSectionRawMutex,
    Option<SourceSelectDriver<I2CBus>>,
> = Mutex::new(None);

// TODO change this into a proper Duration
const DEBOUNCE_DURATION: u64 = 100; // Milliseconds

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Starting main task");

    // Initialise the allocator. Required for data structures.
    // Assuming that 2048 bytes is enough
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 2048;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let p = embassy_rp::init(Default::default());

    // Setup an interrupt on pin 1 to register a button press
    // This is configured as an input pin so that the value can be read.
    // TODO change the name of this pin as it is not an interrupt!
    let source_change_pin = Input::new(p.PIN_1, Pull::Up);

    // Setup the pins used for i2c
    let sda_pin = p.PIN_4;
    let scl_pin = p.PIN_5;

    // The MCP23017 mulitplexer chip used to driver the source lights can use a 400 KHz clock for I2C.
    // However, embassy (currently) only allows the default to 100 KHz (as seen from code).
    // Initail test show that the  slower value still works.
    let i2c_config = Config::default(); // Defaults to 100_000 Hz (as seen from code). Hoping that a slower value will still work

    // Setup the I2C peripheral
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl_pin, sda_pin, i2c_config);

    // Set up the source select hardware module as a shared resource
    let address_offset: u8 = 0x01;
    let select_source_driver = SourceSelectDriver::new(i2c, address_offset)
        .unwrap_or_else(|_| defmt::panic!("Cannot initialise select source driver"));

    // Lock the driver mutex and update it with initialised driver by inserting (using insert()) into the Option
    let mut driver = SHARED_SOURCE_SELECTION_DRIVER.lock().await;
    let _ = driver.insert(select_source_driver);

    // Set up the I2S multiplexer driver
    let mux_addr0_pin = Output::new(p.PIN_10, Level::Low);
    let mux_addr1_pin = Output::new(p.PIN_11, Level::Low);
    let mux_addr2_pin = Output::new(p.PIN_12, Level::Low);
    let mux_en_pin = Output::new(p.PIN_13, Level::Low);

    let i2s_multiplexer =
        I2SMultiplexer::new(mux_addr0_pin, mux_addr1_pin, mux_addr2_pin, mux_en_pin)
            .unwrap_or_else(|_| defmt::panic!("Cannot initialise i2s-multiplexer driver"));

    // As the I2S Multiplexer driver is used by other tasks, set it up as a shared resource
    let mut driver = SHARED_I2S_MULTIPLEXER.lock().await;
    let _ = driver.insert(i2s_multiplexer);

    // Set up the source channel mapping
    let source_bluetooth = Source::Bluetooth(SourceConfig {
        channel: Channel(2),
        display_position: DisplayPosition(0),
    });

    let source_wlan = Source::WirelessLan(SourceConfig {
        channel: Channel(2),
        display_position: DisplayPosition(1),
    });

    let source_cd = Source::Cd(SourceConfig {
        channel: Channel(4),
        display_position: DisplayPosition(2),
    });

    // Note that sources are ordered depending on the display_position
    let sources = sources::Sources::from_array(&[source_bluetooth, source_wlan, source_cd]);

    // Setup the sources as a shared resource so that thay can be used by the sources iterator over many tasks
    // Code is based on this https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives
    let mut shared_sources = SHARED_SOURCES.lock().await;
    // Insert into the option
    let _ = shared_sources.insert(sources);

    // Run a task to set up the intial source
    spawner.spawn(activate_initial_source()).unwrap();

    // Now monitor for source changes triggered by the user pressing the
    //source selection buttons
    spawner.spawn(source_change(source_change_pin)).unwrap();
}

#[embassy_executor::task]
// Altough only called once this needs to be an async function so that we can use the
// await keyword on the locks
// TODO need a more general way to handle source selection
async fn activate_initial_source() {
    defmt::info!("Task activate_initial_source");

    //let mut sources_iterator = SHARED_SOURCES_ITERATOR.lock().await;
    let mut sources = SHARED_SOURCES.lock().await;

    //if let Some(initial_source) = sources_iterator.unwrap().peek() {
    if let Some(initial_source) = sources.as_mut().unwrap().current_source() {
        // Get the new source channel
        let initial_channel = initial_source.channel();
        // Switch the i2s multiplexer to the correct channel
        // Assuming that the mutliplexer driver is initialized
        // Cannot chain the following two statements together due to lifetime issues
        let mut guard = SHARED_I2S_MULTIPLEXER.lock().await;
        let multiplexer = guard.as_mut().unwrap();
        //let mut multiplexer = SHARED_I2S_MULTIPLEXER.lock().await.unwrap();

        let channel_number: u8 = initial_channel.channel_number();

        defmt::info!("Setting channel {}", channel_number);

        multiplexer
            .set_channel(channel_number)
            .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
    } else {
        defmt::panic!("No initial channel set");
    }
}

/// Monitor the source_changed pin is pulsed low from the
/// select source board.
#[embassy_executor::task]
async fn source_change(mut source_change_pin: Input<'static, PIN_1>) {
    defmt::info!("Task: monitor_source_change");

    loop {
        source_change_pin.wait_for_falling_edge().await;

        // Debounce
        Timer::after(Duration::from_millis(DEBOUNCE_DURATION)).await;

        if source_change_pin.is_low() {
            // Source change pin is still low so change source
            defmt::info!("Task activate_source");

            // Lock all resources - assuming that they have been initialised. Note: cannot chain the lock await and the
            // get value together due to lifcycle issues.
            let mut guard = SHARED_SOURCE_SELECTION_DRIVER.lock().await;
            let select_source_driver = guard.as_mut().unwrap();

            let mut guard = SHARED_I2S_MULTIPLEXER.lock().await;
            let i2s_multiplexer_driver = guard.as_mut().unwrap();

            let mut guard = SHARED_SOURCES.lock().await;
            let sources = guard.as_mut().unwrap();

            match select_source_driver.changed_source(sources) {
                Ok(()) => {
                    let new_channel = sources.current_source().unwrap().channel();
                    // Switch the i2s multiplexer to the correct channel
                    let channel_number = new_channel.channel_number();
                    defmt::info!("Setting channel {}", channel_number);

                    i2s_multiplexer_driver
                        .set_channel(channel_number as u8)
                        .unwrap_or_else(|_| defmt::panic!("Cannot set channel"));
                }
                Err(_) => defmt::panic!("Unable to determine changed source"),
            }
        }
    }
}

// End of file
