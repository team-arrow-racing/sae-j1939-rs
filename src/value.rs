//! # Value types

/// Signal range trait.
pub trait SignalRange<T, S> {
    fn new(value: T) -> Result<Signal<T>, &'static str>;
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Signal<T> {
    Value(T),
    Indicator,
    Reserved,
    Error,
    #[default]
    NotAvailable,
}

/// 4-bit signal type.
pub struct Signal4();

impl Signal4 {
    fn new(value: u8) -> Result<Signal<u8>, &'static str> {
        type S = Signal<u8>;

        match value {
            0x0..=0xA => Ok(S::Value(value)),
            0xB => Ok(S::Indicator),
            0xC..=0xD => Ok(S::Reserved),
            0xE => Ok(S::Error),
            0xF => Ok(S::NotAvailable),
            0x10..=u8::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 8-bit signal type.
pub struct Signal8();

impl Signal8 {
    fn new(value: u8) -> Result<Signal<u8>, &'static str> {
        type S = Signal<u8>;

        match value {
            0x00..=0xFA => Ok(S::Value(value)),
            0xFB => Ok(S::Indicator),
            0xFC..=0xFD => Ok(S::Reserved),
            0xFE => Ok(S::Error),
            0xFF => Ok(S::NotAvailable),
        }
    }
}

/// 10-bit signal type.
pub struct Signal10();

impl Signal10 {
    fn new(value: u16) -> Result<Signal<u16>, &'static str> {
        type S = Signal<u16>;

        match value {
            0x00..=0x3FA => Ok(S::Value(value)),
            0x3FB => Ok(S::Indicator),
            0x3FC..=0x3FD => Ok(S::Reserved),
            0x3FE => Ok(S::Error),
            0x3FF => Ok(S::NotAvailable),
            0x400..=u16::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 12-bit signal type.
pub struct Signal12();

impl Signal12 {
    fn new(value: u16) -> Result<Signal<u16>, &'static str> {
        type S = Signal<u16>;

        match value {
            0x00..=0xFAF => Ok(S::Value(value)),
            0xFB0..=0xFBF => Ok(S::Indicator),
            0xFC0..=0xFDF => Ok(S::Reserved),
            0xFE0..=0xFEF => Ok(S::Error),
            0xFF0..=0xFFF => Ok(S::NotAvailable),
            0x1000..=u16::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 16-bit  signal type.
pub struct Signal16();

impl Signal16 {
    fn new(value: u16) -> Result<Signal<u16>, &'static str> {
        type S = Signal<u16>;

        match value {
            0x0000..=0xFAFF => Ok(S::Value(value)),
            0xFB00..=0xFBFF => Ok(S::Indicator),
            0xFC00..=0xFDFF => Ok(S::Reserved),
            0xFE00..=0xFEFF => Ok(S::Error),
            0xFF00..=0xFFFF => Ok(S::NotAvailable),
        }
    }
}

/// 20-bit signal type.
pub struct Signal20();

impl Signal20 {
    fn new(value: u32) -> Result<Signal<u32>, &'static str> {
        type S = Signal<u32>;

        match value {
            0x00000..=0xFAFFF => Ok(S::Value(value)),
            0xFB000..=0xFBFFF => Ok(S::Indicator),
            0xFC000..=0xFDFFF => Ok(S::Reserved),
            0xFE000..=0xFEFFF => Ok(S::Error),
            0xFF000..=0xFFFFF => Ok(S::NotAvailable),
            0x100000..=u32::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 24-bit signal type.
pub struct Signal24();

impl Signal24 {
    fn new(value: u32) -> Result<Signal<u32>, &'static str> {
        type S = Signal<u32>;

        match value {
            0x000000..=0xFAFFFF => Ok(S::Value(value)),
            0xFB0000..=0xFBFFFF => Ok(S::Indicator),
            0xFC0000..=0xFDFFFF => Ok(S::Reserved),
            0xFE0000..=0xFEFFFF => Ok(S::Error),
            0xFF0000..=0xFFFFFF => Ok(S::NotAvailable),
            0x1000000..=u32::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 28-bit signal type.
pub struct Signal28();

impl Signal28 {
    fn new(value: u32) -> Result<Signal<u32>, &'static str> {
        type S = Signal<u32>;

        match value {
            0x0000000..=0xFAFFFFF => Ok(S::Value(value)),
            0xFB00000..=0xFBFFFFF => Ok(S::Indicator),
            0xFC00000..=0xFDFFFFF => Ok(S::Reserved),
            0xFE00000..=0xFEFFFFF => Ok(S::Error),
            0xFF00000..=0xFFFFFFF => Ok(S::NotAvailable),
            0x10000000..=u32::MAX => Err("value provided is not withing valid range"),
        }
    }
}

/// 32-bit signal type.
pub struct Signal32();

impl Signal32 {
    fn new(value: u32) -> Result<Signal<u32>, &'static str> {
        type S = Signal<u32>;

        match value {
            0x00000000..=0xFAFFFFFF => Ok(S::Value(value)),
            0xFB000000..=0xFBFFFFFF => Ok(S::Indicator),
            0xFC000000..=0xFDFFFFFF => Ok(S::Reserved),
            0xFE000000..=0xFEFFFFFF => Ok(S::Error),
            0xFF000000..=0xFFFFFFFF => Ok(S::NotAvailable),
        }
    }
}

/// Transmitted values for discrete parameters (i.e. measured).
///
/// Reference: SAE J1939-71 Table 2.
#[derive(Default, Copy, Clone)]
pub enum Parameter {
    Disabled = 0b00,
    Enabled = 0b01,
    Error = 0b10,
    #[default]
    NotAvailable = 0b11,
}

/// Transmitted values for control commands (i.e. status).
///
/// Reference: SAE J1939-71 Table 2.
#[derive(Default, Copy, Clone)]
pub enum Control {
    /// Command to disable function (turn off).
    Disable = 0b00,
    /// Command to enable function (turn on).
    Enable = 0b01,
    // Reserved = 0b10,
    /// Take no action (leave as is).
    #[default]
    NoAction = 0b11,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_default() {
        // should default to the assigned 'undefined' value
        assert_eq!(Signal::<u8>::default(), Signal::<u8>::NotAvailable);
    }

    #[test]
    fn parameter_default() {
        // should default to the assigned 'undefined' value
        assert_eq!(Parameter::default() as u8, 0b11);
    }

    #[test]
    fn control_default() {
        // should default to the assigned 'undefined' value
        assert_eq!(Control::default() as u8, 0b11);
    }
}
