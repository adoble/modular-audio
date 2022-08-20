//! This program creates an I2S interface via 3 seperate PIO state machines, toggling the
//! GPIO 9, 10, and 11 pins.
//!
//!
//! Using the "offical" example for I2S on the pico at [here](https://github.com/raspberrypi/pico-extras/tree/master/src/rp2_common/pico_audio_i2s)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use bsp::hal;
use rp_pico as bsp;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::{FunctionPio0, Pin},
    pio::PIOExt,
};


use embedded_time::rate::*;

//use libm::{round, sin};
use libm::sin;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    //let external_xtal_freq_hz = 12_000_000u32;
    // The default is to generate a 125 MHz system clock
    let clocks = init_clocks_and_plls(
        //external_xtal_freq_hz,
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();


    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // configure pins for Pio
    let _pin_data: Pin<_, FunctionPio0> = pins.gpio0.into_mode();
    let _pin_bclk: Pin<_, FunctionPio0> = pins.gpio1.into_mode();
    let _pin_lrck: Pin<_, FunctionPio0> = pins.gpio2.into_mode();
    

    // PIN id for use inside of PIO
    let pin_data_id = 0;
    let pin_bck_id = 1; // BCK and LRCK must be contigouous
    let pin_lrck_id = 2;
    let _pin25_led = 25; // TODO

    // Define some simple PIO program.
    let program_audio_i2s = pio_proc::pio_asm!(
        "
        ;
        ; Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
        ;
        ; SPDX-License-Identifier: BSD-3-Clause
        ;
        
        ; Transmit a mono or stereo I2S audio stream as stereo
        ; This is 16 bits per sample; can be altered by modifying the 'set' params,
        ; or made programmable by replacing 'set x' with 'mov x, y' and using Y as a config register.
        ;
        ; Autopull must be enabled, with threshold set to 32.
        ; Since I2S is MSB-first, shift direction should be to left.
        ; Hence the format of the FIFO word is:
        ;
        ; | 31   :   16 | 15   :    0 |
        ; | sample ws=0 | sample ws=1 |
        ;
        ; Data is output at 1 bit per clock. Use clock divider to adjust frequency.
        ; Fractional divider will probably be needed to get correct bit clock period,
        ; but for common syslck freqs this should still give a constant word select period.
        ;
        ; One output pin is used for the data output.
        ; Two side-set pins are used. Bit 0 is clock, bit 1 is word select.
        
        ; Send 16 bit words to the PIO for mono, 32 bit words for stereo
        
        .side_set 2
        
                            ;        /--- LRCLK
                            ;        |/-- BCLK
        bitloop1:           ;        ||
            out pins, 1       side 0b10
            jmp x-- bitloop1  side 0b11
            out pins, 1       side 0b00
            set x, 14         side 0b01
        
        bitloop0:
            out pins, 1       side 0b00
            jmp x-- bitloop0  side 0b01
            out pins, 1       side 0b10
        public entry_point:
            set x, 14         side 0b11
            "
    );

    // Initialize and start PIO
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    
    // Calculate the divisor for 44.1 KHz sample rate
    let sample_rate =  44100 as f32;
    let n_bits_per_channel = 16 as f32;
    let sys_clock_freq = clocks.system_clock.freq().integer() as f32;
    

    let div = (sys_clock_freq / (2. * n_bits_per_channel * sample_rate)) as f32;
    
    // TODO programatically adjust the number bits per chanell
    // install and set up the audio-i2s pio program into the state machine and get a handle to the tx fifo on it.
    let installed = pio.install(&program_audio_i2s.program).unwrap();
    let (mut sm_audio_i2s, _, mut tx_data) = hal::pio::PIOBuilder::from_program(installed)
        .out_pins(pin_data_id, 1)
        .side_set_pin_base(pin_bck_id) // BCK and LRCK are contiguous
        .autopull(true)
        .pull_threshold(32)
        .clock_divisor(div) //?? TODO
        .build(sm0);
    sm_audio_i2s.set_pindirs([
        (pin_data_id, hal::pio::PinDir::Output),
        (pin_bck_id, hal::pio::PinDir::Output),
        (pin_lrck_id, hal::pio::PinDir::Output),
    ]);

    // Start SM
    sm_audio_i2s.start();

    cortex_m::asm::delay(10);

    // Set up some i2s data - a sine wave for both left and right channel with 16 bits
    // Only the first 16 bits are set. This will be replicated during the transfer later.
    //
    // For a  2 kHz signal we need to represent 1 period before we repeat. With a 44.1 khz  sample frequency
    // we need to have (1 / 2kHZ) * 44100 samples = 22.05 samples.

    // Acually generates a tone of 1  Khz
    const N_SAMPLES: usize = 22;   // Tone of 1 kHz
    
    
    
    //const N_SAMPLES: usize = 2200;
    let mut samples: [i16; N_SAMPLES] = [0; N_SAMPLES];
    //let n_samples = 22;

    let vol = 1024.;  // Not too loud

    let sample_period: f64 = (2. * core::f64::consts::PI) / (N_SAMPLES as f64);
    for i in 0..N_SAMPLES {
        let mut sample_value: f64 = sin(i as f64 * sample_period); // Range -1..1

        // TODO see https://doc.rust-lang.org/std/primitive.u16.html#method.overflowing_mul
        sample_value *= vol; 
        samples[i] = sample_value as i16;
    }

    let mut sample_index = 0;
    //#[allow(clippy::empty_loop)]
    loop {
        if !tx_data.is_full() {
            // Write both left and write channels with the same data
            tx_data.write_u16_replicated(samples[sample_index] as u16);
            sample_index = (sample_index + 1) % N_SAMPLES;
        }
    }
}
