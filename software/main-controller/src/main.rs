//!
//! Selects a source based on the interruts delivered by the source_select_processor and changes
//! the LEDS on the source_select_processor accordingly.
//!
//! Circuit:
//!     Source select button (pulled up, active low) -> MCP23017::GPB0
//!     MCP23017::GPA[0..5]  -> LEDS for each channel
//!     MCP23017::INTB (change, active low) -> MCU::GPIO1
//!     MCP23017::[SDA, SCL] -> MCU::I2C1[SDA, SCL]
//!  - Process the interrupt over GPIO 1 (button has been pressed).  MCP23017 Pin used for the button is GPB0.

#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

//use defmt::*;
use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

mod channel;
// mod source;
mod sources;

//mod source_channel_map;
mod source_select_driver;

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

                    multiplexer
                        .set_channel(channel_number as u8)
                        .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
                };
            });
    }

    #[task(shared = [select_source_driver, i2s_multiplexer, sources_iterator])]
    fn activate_initial_source(ctx: activate_initial_source::Context) {
        defmt::info!("Task activate_initial_source");

        let select_source_driver = ctx.shared.select_source_driver;
        let i2s_multiplexer_driver = ctx.shared.i2s_multiplexer;
        let sources_iterator = ctx.shared.sources_iterator;

        (
            select_source_driver,
            i2s_multiplexer_driver,
            sources_iterator,
        )
            .lock(|selecter, multiplexer, sources_iter| {
                if let Some(initial_source) = sources_iter.peek() {
                    // Get the new source channel
                    let initial_channel = initial_source.channel();
                    // Switch the i2s multiplexer to the correct channel
                    let channel_number: u8 = initial_channel.channel_number();

                    multiplexer
                        .set_channel(channel_number as u8)
                        .unwrap_or_else(|_| defmt::panic!("Cannot set channel"))
                } else {
                    defmt::panic!("No initial channel set");
                }
            });
    }
}

// End of file
