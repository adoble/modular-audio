use defmt;

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Bluetooth = 0,
    WirelessLAN = 1,
    CD = 2,
    InternetRadio = 3, //TODO could provide the url as well.
    DABRadio = 4,
    Aux = 5,
}

impl Source {
    pub fn init() -> Self {
        Source::Bluetooth
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Bluetooth => Self::WirelessLAN,
            Self::WirelessLAN => Self::CD,
            Self::CD => Self::InternetRadio,
            Self::InternetRadio => Self::DABRadio,
            Self::DABRadio => Self::Aux,
            Self::Aux => Self::Bluetooth,
        }
    }

    pub fn activate(&self) {
        // TODO
        defmt::info!("Activating source {}", *self as u8)
    }
}
