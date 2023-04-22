// The different source types
pub mod source_bluetooth;
pub mod source_cd;
pub mod source_wireless_lan;
// TODO add more here

/// Reexports
/// This allows the structs to be used as
/// ```
///    use crate::sources::SourceBluetooth
/// ```
///
//pub use source::{Source, SourceError};
pub use source_bluetooth::SourceBluetooth;
pub use source_cd::SourceCd;
pub use source_wireless_lan::SourceWirelessLan;
//TODO add more here

use crate::channel::Channel;

pub trait Source {
    fn activate(&self) -> Result<(), SourceError>;

    /// Get the channel associated with this source
    fn channel(&self) -> Channel;

    /// Get the display position of the source
    fn display_position(&self) -> DisplayPosition;
}

/// Defines the position that a source is displayed at.
/// Also can be converted  into and from u8, i.e.:
/// ```
/// let index: u8 = 4;
/// let dp: DisplayPosition = index.into();
/// assert!(dp.0 == 4);
/// ```
///
/// ```
/// let index = 2;
/// let dp = DisplayPosition::from(index);
/// assert!((dpp.0 == 2));
/// ```
///
/// ```
/// let dpu = DisplayPosition(5);
/// let index : u8 = dpu.into();
/// assert!((index == 5));
/// ```
///
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

#[derive(Debug)]
#[non_exhaustive]
pub enum SourceError {
    IncorrectChannel,
    ActivationFailed,
    NotImplemented,
    Creation,
}
