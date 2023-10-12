//! Selects a source based on the interruts delivered by the source_select_processor and changes
//! the LEDS on the source_select_processor accordingly.
//!
//! Circuit:
//!     Source select button (pulled up, active low) -> MCP23017::GPB0
//!     MCP23017::GPA[0..5]  -> LEDS for each channel
//!     MCP23017::INTB (change, active low) -> MCU::GPIO1
//!     MCP23017::[SDA, SCL] -> MCU::I2C1[SDA, SCL]
//!  - Process the interrupt over GPIO 1 (button has been pressed).  MCP23017 Pin used for the button is GPB0.
//!

// TODOs
// - The mcp23017 is not correctly initialised and can hang in that the "interrupt" stays stuck low.
// - Using https://github.com/lulf/watchful/blob/main/firmware/app/src/main.rs as an example do two things:
//        - Use StaticCell
//        - Only put I2C driver in as mutex. Other drivers, pass in as references to the corresponding functions.
//          However, my first approach would be to continue as is (mutexes on all the drivers) and then see if we can simplify
// - Maybe need to use shared-bus, see https://docs.rs/shared-bus/latest/shared_bus/ and the referenced blog post.
// - Restructure so that we can use std unit tests.
//       See https://ferrous-systems.com/blog/test-embedded-app/#accessing-std-when-testing-embedded-code-testing-host2target

#![no_std]
#![no_main]
// #![feature(default_alloc_error_handler)] // Stable since 1.68.0 and no longer requires an attribute to enable
#![feature(type_alias_impl_trait)]

//use defmt::*;

// Included modules
//mod channel;
mod error;
mod source_select_driver;
// mod sources;

//use channel::Channel;
//use sources::{Source, SourceIterator, Sources};

// These need to be explicity imported due to the #![cfg_attr(not(test), no_std)] above.
// use core::default::Default;
// use core::marker::Sized;
// use core::module_path;
// use core::option::Option::{self, None, Some};

use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

// use core::cell::RefCell;
// use core::mem::MaybeUninit;

use embassy_executor::Spawner;
use embassy_rp::gpio; // gpio::peripherals::PIN_1,
use embassy_rp::peripherals::{PIN_1, PIN_10, PIN_11, PIN_12, PIN_13};
use embassy_time::{Duration, Timer};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex; // Is this the right one? TODO - Using an async mutex

use embassy_rp::i2c::{self, Blocking, Config, I2c};
use embassy_rp::peripherals::I2C0;
use gpio::{Input, Level, Output, Pull};

// Controller specific crates
use crate::source_select_driver::SourceSelectDriver;
use i2s_multiplexer::I2SMultiplexer;

use sources::{DisplayPosition, Source, SourceConfig, SourceIterator, Sources};

use channel::Channel;

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
// See https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives#heading-readingwriting-across-async-tasks
// Is ThreadModeRawMutex the right model? or is NoopRawMutex better as  data is only shared between tasks running on the same executor?
// static SHARED_SOURCES: Mutex<ThreadModeRawMutex, RefCell<Option<Sources>>> =
//     Mutex::new(RefCell::new(None));
static SHARED_SOURCES: Mutex<ThreadModeRawMutex, Option<Sources>> = Mutex::new(None);

// static SHARED_SOURCES_ITERATOR: Mutex<
//     ThreadModeRawMutex,
//     RefCell<Option<SourceIterator<'static>>>,
// > = Mutex::new(RefCell::new(None));
static SHARED_SOURCES_ITERATOR: Mutex<ThreadModeRawMutex, Option<SourceIterator<'static>>> =
    Mutex::new(None);

static SHARED_I2S_MULTIPLEXER: Mutex<ThreadModeRawMutex, Option<MultiplexerDriver>> =
    Mutex::new(None);

static SHARED_SOURCE_SELECTION_DRIVER: Mutex<
    ThreadModeRawMutex,
    Option<SourceSelectDriver<I2CBus>>,
> = Mutex::new(None);

// TODO change this into a proper Duration
const DEBOUNCE_DURATION: u64 = 100; // Milliseconds

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Starting main task");

    let p = embassy_rp::init(Default::default());

    // Setup an interrupt on pin 1 to register a button press
    // This is configured as an input pin so that the value can be read.
    // TODO change the name of this pin as it is not an interrupt!
    let source_change_pin = Input::new(p.PIN_1, Pull::Up);

    // Setup the pins used for i2c
    let sda_pin = p.PIN_4;
    let scl_pin = p.PIN_5;
    // let i2c_config = Config {
    //     frequency: 400.khz(),   /// This is what we have been using before
    // };
    let i2c_config = Config::default(); // Defaults to 100_000 Hz (as seen from code). Hoping that a slower value will still work

    // Setup the I2C peripheral
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl_pin, sda_pin, i2c_config);

    // Set up the source select hardware module as a shared resource
    let address_offset: u8 = 0x01;
    let select_source_driver = SourceSelectDriver::new(i2c, address_offset)
        .unwrap_or_else(|_| defmt::panic!("Cannot initialise select source driver"));

    // Lock the driver mutex and update it with initialised driver
    let mut driver = SHARED_SOURCE_SELECTION_DRIVER.lock().await;
    driver.insert(select_source_driver); // inserting into the Option
                                         // SHARED_SOURCE_SELECTION_DRIVER = Mutex::new(Some(select_source_driver));

    // Set up the I2S multiplexer driver
    let mux_addr0_pin = Output::new(p.PIN_10, Level::Low);

    let mux_addr1_pin = Output::new(p.PIN_11, Level::Low);
    let mux_addr2_pin = Output::new(p.PIN_12, Level::Low);
    let mux_en_pin = Output::new(p.PIN_13, Level::Low);

    let i2s_multiplexer =
        I2SMultiplexer::new(mux_addr0_pin, mux_addr1_pin, mux_addr2_pin, mux_en_pin)
            .unwrap_or_else(|_| defmt::panic!("Cannot initialise i2s-multiplexer driver"));

    // As the I2S Multiplexer is used by other tasks, set it up as a shared resource
    //    SHARED_I2S_MULTIPLEXER = Mutex::new(Some(i2s_multiplexer));
    let mut driver = SHARED_I2S_MULTIPLEXER.lock().await;
    driver.insert(i2s_multiplexer);

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

    let mut sources = sources::Sources::new();

    sources.insert(source_bluetooth);
    sources.insert(source_wlan);
    sources.insert(source_cd);

    // Setup the sources as a shared resource so that thay can be used by the sources iterator over many tasks
    // Code is based on this https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives
    //let mut shared_sources = SHARED_SOURCES.lock().await;
    let mut shared_sources = SHARED_SOURCES.lock().await;
    // Insert into the option
    //let inserted_sources: &'static mut Sources = shared_sources.insert(sources);
    //let inserted_sources = shared_sources.insert(sources);
    shared_sources.insert(sources);

    let mut sources_iterator = shared_sources.as_mut().unwrap().iter();

    //let sources_iterator = inserted_sources.iter();

    //SHARED_SOURCES_ITERATOR = Mutex::new(Some(sources_iterator));
    let mut shared_sources_iterator = SHARED_SOURCES_ITERATOR.lock().await;
    shared_sources_iterator.insert(sources_iterator);

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

    let mut sources_iterator = SHARED_SOURCES_ITERATOR.lock().await;

    if let Some(initial_source) = sources_iterator.unwrap().peek() {
        // Get the new source channel
        let initial_channel = initial_source.channel();
        // Switch the i2s multiplexer to the correct channel
        // Assuming that the mutliplexer driver is initialized
        // Cannot chain the following two statements together due to lifetime issues
        let mut guard = SHARED_I2S_MULTIPLEXER.lock().await;
        let mut multiplexer = guard.as_mut().unwrap();
        //let mut multiplexer = SHARED_I2S_MULTIPLEXER.lock().await.unwrap();

        let channel_number: u8 = initial_channel.channel_number();

        defmt::info!("Setting channel {}", channel_number);

        multiplexer
            .set_channel(channel_number as u8)
            .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
    } else {
        defmt::panic!("No initial channel set");
    }

    //     SHARED_SOURCES_ITERATOR.lock(|sources_iter| {
    //         let x = sources_iter.borrow();
    // HERE
    // //Do you need to have a refcell in the Mutex.  In https://doc.rust-lang.org/std/cell/index.html
    // // is the following statement:
    // //  "If you need to do aliasing and mutation among multiple threads,
    // // Mutex<T>, RwLock<T>, OnceLock<T> or atomic types are the correct data structures to do so"

    // // Is having a critical section as below the way to go???

    // // Looking at https://apollolabsblog.hashnode.dev/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives#heading-readingwriting-across-async-tasks
    // // it seems that I should be using the async mutex type as I am reading and writing across async tasks.
    // // No longer need a refcell.
    //         if x.is_some() {}
    //     })(i2s_multiplexer_driver, sources_iterator)
    //     .lock(|multiplexer, sources_iter| {
    //         if let Some(initial_source) = sources_iter.peek() {
    //             // Get the new source channel
    //             let initial_channel = initial_source.channel();
    //             // Switch the i2s multiplexer to the correct channel
    //             let channel_number: u8 = initial_channel.channel_number();

    //             defmt::info!("Setting channel {}", channel_number);

    //             multiplexer
    //                 .set_channel(channel_number as u8)
    //                 .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
    //         } else {
    //             defmt::panic!("No initial channel set");
    //         }
    //     });
}

/// Monitor the source_changed pin is pulsed low from the
/// select source board.
///
/// This spawns a task to select the source.
//#[task(binds = IO_IRQ_BANK0, local = [source_change_interrupt_pin])]

//TODO this task needs to be spawned!!!!!!!
#[embassy_executor::task]
async fn source_change(mut source_change_pin: Input<'static, PIN_1>) {
    defmt::info!("Task: monitor_source_change");

    loop {
        source_change_pin.wait_for_falling_edge().await;
        defmt::info!("Falling edge detected");

        // Debounce
        Timer::after(Duration::from_millis(DEBOUNCE_DURATION)).await;

        if source_change_pin.is_low() {
            // Source change pin is still low so change source
            defmt::info!("Task activate_source");

            // Lock all resources - assuming that they have been initialised
            let select_source_driver = SHARED_SOURCE_SELECTION_DRIVER.lock().await.unwrap();
            //let i2s_multiplexer_driver = SHARED_I2S_MULTIPLEXER.lock().await.unwrap();
            let mut guard = SHARED_I2S_MULTIPLEXER.lock().await;
            let mut i2s_multiplexer_driver = guard.as_mut().unwrap();

            let sources_iterator = SHARED_SOURCES_ITERATOR.lock().await.unwrap();

            if let Some(new_source) = select_source_driver
                .changed_source(&mut sources_iterator)
                .unwrap_or_else(|_| {
                    defmt::panic!("Unable to determine changed source");
                })
            {
                // Get the new source channel
                let new_channel = new_source.channel();
                // Switch the i2s multiplexer to the correct channel
                let channel_number: u8 = new_channel.channel_number();

                defmt::info!("Setting channel {}", channel_number);

                i2s_multiplexer_driver
                    .set_channel(channel_number as u8)
                    .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
            };
        }
    }
}

/*
#[rtic::app(
    device = rp_pico::hal::pac, dispatchers = [TIMER_IRQ_1]
)]

mod app {

    //use embedded_hal::digital::v2::OutputPin;

    use i2s_multiplexer::I2SMultiplexer;
    use rp_pico::hal::gpio::PullUp;
    use rp_pico::hal::{
        clocks, gpio, gpio::pin::bank0::Gpio1, gpio::pin::bank0::Gpio10, gpio::pin::bank0::Gpio11,
        gpio::pin::bank0::Gpio12, gpio::pin::bank0::Gpio13, gpio::pin::bank0::Gpio4,
        gpio::pin::bank0::Gpio5, gpio::pin::Input, i2c::I2C, pac, sio::Sio, watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use crate::source_select_driver::SourceSelectDriver;

    // use crate::source::SourceError;

    // use crate::source::{DisplayPosition, SourceBluetooth, SourceCd, SourceWirelessLan};

    use crate::sources::{DisplayPosition, Source, SourceConfig, SourceIterator, Sources};

    use crate::channel::Channel;

    use rp2040_monotonic::{fugit::ExtU64, Rp2040Monotonic};

    use core::mem::MaybeUninit;

    // Time handling traits:
    use fugit::RateExtU32;

    //use enum_map::{enum_map, EnumMap};
    // Need to define an allocator to be ablke to use smart pointers such as Box
    extern crate alloc;

    use embedded_alloc::Heap;
    #[global_allocator]
    static HEAP: Heap = Heap::empty();

    //use alloc::boxed::Box;

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    type I2CBus = I2C<
        pac::I2C0,
        (
            gpio::Pin<Gpio4, gpio::FunctionI2C>,
            gpio::Pin<Gpio5, gpio::FunctionI2C>,
        ),
    >;

    // Pin types
    type MuxAddr0 = gpio::Pin<Gpio10, gpio::Output<gpio::PushPull>>;
    type MuxAddr1 = gpio::Pin<Gpio11, gpio::Output<gpio::PushPull>>;
    type MuxAddr2 = gpio::Pin<Gpio12, gpio::Output<gpio::PushPull>>;
    type MuxEnable = gpio::Pin<Gpio13, gpio::Output<gpio::PushPull>>;

    // Driver types
    type MultiplexerDriver = I2SMultiplexer<MuxAddr0, MuxAddr1, MuxAddr2, MuxEnable>;

    const DEBOUNCE_DURATION: u64 = 100; // Milliseconds

    // Shared resources
    #[shared]
    struct Shared {
        select_source_driver: SourceSelectDriver<I2CBus>,
        //source_selection_iterator: &'static mut SourceInterator<'static>,
        sources_iterator: SourceIterator<'static>,
        i2s_multiplexer: MultiplexerDriver,
    }

    // Local resources
    #[local]
    struct Local {
        // The MCU pin used to received the interrupt from the source select processor
        source_change_interrupt_pin: gpio::Pin<Gpio1, Input<PullUp>>,
    }

    /// RTIC Init task
    #[init(local=[
        // Task local initialized resources are static as per documentation.
        // Here we use MaybeUninit to allow for initialization in init()
        // This enables its usage in driver initialization
        // TODO do we need this? The new documentation is not clear
        //select_source_driver_ctx: MaybeUninit<SourceSelectDriver<I2CBus>> = MaybeUninit::uninit(),
        //i2s_multiplexer_ctx: MaybeUninit<Multiplexer> = MaybeUninit::unint(),
        sources_ctx: MaybeUninit<Sources> = MaybeUninit::uninit(),  // Need to keep the sources backing the iterator
        //source_selection_iterator_ctx : MaybeUninit<SourceInterator<'static>> = MaybeUninit::uninit(),

    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("Task init");

        // Setup the clock. This is required.
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // Get the pins
        let sio = Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        // TODOP new required
        // Initialise the allocator.
        // TODO guessing a healp size of 1024 as this used in examples. Need to check.
        {
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 1024;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }

        // Setup an interrupt on pin 1 to register a button press
        // This is configured as an input pin so that the value can be read.
        let source_change_interrupt_pin = pins.gpio1.into_pull_up_input();

        source_change_interrupt_pin.set_interrupt_enabled(gpio::Interrupt::EdgeLow, true); // ??? Does this work?

        // Setup the monotonic timer
        let mono = Rp2040Monotonic::new(ctx.device.TIMER);

        // Setup the pins used for i2c
        let sda_pin = pins.gpio4.into_mode::<rp_pico::hal::gpio::FunctionI2C>();
        let scl_pin = pins.gpio5.into_mode::<rp_pico::hal::gpio::FunctionI2C>();
        // Setup the I2C peripheral
        let i2c = I2C::i2c0(
            ctx.device.I2C0,
            sda_pin,
            scl_pin,
            400.kHz(),
            &mut ctx.device.RESETS,
            &clocks.peripheral_clock,
        );

        // Set up the source select hardware module
        let address_offset: u8 = 0x01;
        let select_source_driver = SourceSelectDriver::new(i2c, address_offset)
            .unwrap_or_else(|_| defmt::panic!("Cannot initialise select source driver"));
        // let select_source_driver_initialised: &'static mut _ = ctx
        //     .local
        //     .select_source_driver_ctx
        //     .write(select_source_driver);

        // Set up the I2S multiplexer driver
        let mux_addr0_pin: MuxAddr0 = pins.gpio10.into_push_pull_output();
        let mux_addr1_pin: MuxAddr1 = pins.gpio11.into_push_pull_output();
        let mux_addr2_pin: MuxAddr2 = pins.gpio12.into_push_pull_output();
        let mux_en_pin: MuxEnable = pins.gpio13.into_push_pull_output();

        let i2s_multiplexer =
            I2SMultiplexer::new(mux_addr0_pin, mux_addr1_pin, mux_addr2_pin, mux_en_pin)
                .unwrap_or_else(|_| defmt::panic!("Cannot initialise i2s-multiplexer driver"));

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

        let mut sources = crate::sources::Sources::new();

        sources.insert(source_bluetooth);
        sources.insert(source_wlan);
        sources.insert(source_cd);

        // Set up the sources interator. The sources are first stored in the static local
        //  context variable so that the iterator basis is also availble.
        let sources_initialised: &'static mut _ = ctx.local.sources_ctx.write(sources);
        let sources_iterator = sources_initialised.iter();

        // sources[3] = sources::SourceInternetRadio::new(2);
        // sources[4] = sources::SourceAux::new(0);
        // sources[5] = sources::SourceDabRadio::new(1);

        //TODO As sources does not go into Shared is this type of initialisation neccessary?
        //let sources_initialised: &'static mut _ = ctx.local.sources_ctx.write(sources);

        //let mut sources_selection_iterator = sources_initialised.into_iter();
        // let sources_selection_iterator = sources_initialised.into_iter();
        // let sources_selection_iterator_initialised: &'static mut _ = ctx
        //     .local
        //     .source_selection_iterator_ctx
        //     .write(sources_selection_iterator);

        // Activate the initial source
        activate_initial_source::spawn().unwrap();

        // let selected_source = sources_selection_iterator_initialised.next();
        // if let Err(_) = match selected_source {
        //     Some(source) => source.activate(),

        //     None => Err(SourceError::ActivationFailed),
        // } {
        //     defmt::error!("Cannot activate the initial source")
        // }

        (
            Shared {
                select_source_driver,
                sources_iterator,
                i2s_multiplexer,
                //source_selection_iterator: sources_selection_iterator_initialised,
            },
            Local {
                source_change_interrupt_pin,
            },
            init::Monotonics(mono),
        )
    }

    /// RTIC Idle task
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("Task idle - waiting for input");

        loop {
            continue;
        }
    }

    /// Service routine when the source_changed line is pulsed from the
    /// select source board. The source_changed line is set up as an
    /// interrupt.
    ///
    /// This spawns a task to select the source.
    #[task(binds = IO_IRQ_BANK0, local = [source_change_interrupt_pin])]
    fn source_change_irq(ctx: source_change_irq::Context) {
        defmt::info!("Task: source_change_irq");

        ctx.local
            .source_change_interrupt_pin
            .clear_interrupt(gpio::Interrupt::EdgeLow);

        // Spawn the task to select the source. Delayed for 100 ms to removed bounce
        // (the select_source task will check the button state)
        activate_source::spawn_after(DEBOUNCE_DURATION.millis())
            .unwrap_or_else(|_| defmt::panic!("Unable to spawn select_source"));
    }

    /// RTIC task to select a source AFTER an interrupt has been generated.
    /// The selected source is stored as a shared resource.
    #[task(shared = [select_source_driver, i2s_multiplexer, sources_iterator])]
    fn activate_source(ctx: activate_source::Context) {
        defmt::info!("Task activate_source");

        let select_source_driver = ctx.shared.select_source_driver;
        let i2s_multiplexer_driver = ctx.shared.i2s_multiplexer;
        let sources_iterator = ctx.shared.sources_iterator;

        (
            select_source_driver,
            i2s_multiplexer_driver,
            sources_iterator,
        )
            .lock(|selecter, multiplexer, sources_iter| {
                if let Some(new_source) =
                    selecter.changed_source(sources_iter).unwrap_or_else(|_| {
                        defmt::panic!("Unable to determine changed source");
                    })
                {
                    // Get the new source channel
                    let new_channel = new_source.channel();
                    // Switch the i2s multiplexer to the correct channel
                    let channel_number: u8 = new_channel.channel_number();

                    defmt::info!("Setting channel {}", channel_number);

                    multiplexer
                        .set_channel(channel_number as u8)
                        .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
                };
            });
    }

    #[task(shared = [select_source_driver, i2s_multiplexer, sources_iterator])]
    fn activate_initial_source(ctx: activate_initial_source::Context) {
        defmt::info!("Task activate_initial_source");

        let i2s_multiplexer_driver = ctx.shared.i2s_multiplexer;
        let sources_iterator = ctx.shared.sources_iterator;

        (i2s_multiplexer_driver, sources_iterator).lock(|multiplexer, sources_iter| {
            if let Some(initial_source) = sources_iter.peek() {
                // Get the new source channel
                let initial_channel = initial_source.channel();
                // Switch the i2s multiplexer to the correct channel
                let channel_number: u8 = initial_channel.channel_number();

                defmt::info!("Setting channel {}", channel_number);

                multiplexer
                    .set_channel(channel_number as u8)
                    .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
            } else {
                defmt::panic!("No initial channel set");
            }
        });
    }
}
*/

// End of file
