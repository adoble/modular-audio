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

//use defmt::*;
use defmt as _;
use defmt_rtt as _;
use panic_probe as _;

mod source_select_driver;

#[rtic::app(
    device = rp_pico::hal::pac, dispatchers = [TIMER_IRQ_1]
)]
mod app {

    use rp_pico::hal::gpio::PullUp;
    use rp_pico::hal::{
        clocks, gpio, gpio::pin::bank0::Gpio1, gpio::pin::bank0::Gpio4, gpio::pin::bank0::Gpio5,
        gpio::pin::Input, i2c::I2C, pac, sio::Sio, watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use rp2040_monotonic::{fugit::ExtU64, Rp2040Monotonic};

    use core::mem::MaybeUninit;

    // Time handling traits:
    use fugit::RateExtU32;

    use crate::source_select_driver::{Source, SourceSelectDriver};

    #[monotonic(binds = TIMER_IRQ_0, default = true)]
    type Rp2040Mono = Rp2040Monotonic;

    type I2CBus = I2C<
        pac::I2C0,
        (
            gpio::Pin<Gpio4, gpio::FunctionI2C>,
            gpio::Pin<Gpio5, gpio::FunctionI2C>,
        ),
    >;

    const DEBOUNCE_DURATION: u64 = 100; // Milliseconds

    // Shared resources
    #[shared]
    struct Shared {
        select_source_driver: &'static mut SourceSelectDriver<I2CBus>,
        source_selected: Source, // TODO change this to a enum
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
        select_source_driver_ctx: MaybeUninit<SourceSelectDriver<I2CBus>> = MaybeUninit::uninit()
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
        let select_source_driver_initialised: &'static mut _ = ctx
            .local
            .select_source_driver_ctx
            .write(select_source_driver);

        // Activate the initial source
        let initial_source = Source::init();
        initial_source.activate();

        (
            Shared {
                select_source_driver: select_source_driver_initialised,
                source_selected: initial_source,
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
        select_source::spawn_after(DEBOUNCE_DURATION.millis())
            .unwrap_or_else(|_| defmt::panic!("Unable to spawn select_source"));
    }

    /// RTIC task to select a source.
    /// The selected source is stored as a shared resource.
    #[task(shared = [source_selected, select_source_driver])]
    fn select_source(ctx: select_source::Context) {
        defmt::info!("Task select_source");

        let select_source_driver = ctx.shared.select_source_driver;
        let source_selected = ctx.shared.source_selected;

        (select_source_driver, source_selected).lock(|driver, source_selected| {
            if let Some(new_source) = driver.changed_source(*source_selected).unwrap_or_else(|_| {
                // defmt::panic!("Unable to determine changed source: error {:?}", err)  // TODO provide some formatting on the error type
                defmt::panic!("Unable to determine changed source")
            }) {
                *source_selected = new_source;
                new_source.activate();
            }
        });
    }
}

// End of file
