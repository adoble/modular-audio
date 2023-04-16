pub mod source;

// The different source types
pub mod source_bluetooth;
pub mod source_wireless_lan;
// TODO add more here

/// Reexports
/// This allows the structs to be used as
/// ```
///    use crate::sources::SourceBluetooth
/// ```
///
pub use source::{Source, SourceError};
pub use source_bluetooth::SourceBluetooth;
pub use source_wireless_lan::SourceWirelessLan;
//TODO add more here
