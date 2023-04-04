#![no_std]
//#![allow(dead_code, non_camel_case_types)]
#![allow(dead_code)]

use embedded_hal as hal;

pub struct I2SMultiplexer {
    source_address: u8,
}
