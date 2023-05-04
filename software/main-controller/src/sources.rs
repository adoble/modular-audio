//extern crate alloc;

//use alloc::boxed::Box;
//use alloc::vec::Vec;
//use core::ops::{Index, IndexMut};

use crate::channel::Channel;

pub enum SourceKind {
    Bluetooth(Source),
    WirelessLan(Source),
    Cd(Source),
    InternetRadio(Source),
    DabRadio(Source),
    Aux(Source),
}

pub struct Source {
    channel: Channel,
    display_position: DisplayPosition,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayPosition(pub u8);

impl From<u8> for DisplayPosition {
    fn from(index: u8) -> Self {
        DisplayPosition(index)
    }
}

impl From<DisplayPosition> for u8 {
    fn from(dp: DisplayPosition) -> Self {
        dp.0
    }
}
