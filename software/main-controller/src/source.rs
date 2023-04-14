use enum_map::{Enum, EnumMap};

#[derive(Debug, Copy, Clone)]
pub struct Source {
    source_type: SourceType,
}

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
pub enum SourceType {
    Bluetooth = 0,
    WirelessLAN = 1,
    CD = 2,
    InternetRadio = 3, //TODO could provide the url as well.
    DABRadio = 4,
    Aux = 5,
}

impl Source {
    pub fn new() -> Self {
        Source {
            source_type: SourceType::Bluetooth,
        }
    }

    pub fn next(&self) -> Self {
        let new_source_type = match self.source_type {
            SourceType::Bluetooth => SourceType::WirelessLAN,
            SourceType::WirelessLAN => SourceType::CD,
            SourceType::CD => SourceType::InternetRadio,
            SourceType::InternetRadio => SourceType::DABRadio,
            SourceType::DABRadio => SourceType::Aux,
            SourceType::Aux => SourceType::Bluetooth,
        };

        Self {
            source_type: new_source_type,
        }
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    // Returns a u8 value of the source type.
    // No guarentees are given as to what these values are.
    pub fn as_u8(&self) -> u8 {
        self.source_type as u8
    }

    pub fn activate(&self) {
        // TODO
        defmt::info!("Activating source {}", self.source_type as u8)
    }
}
