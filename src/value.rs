//! # Value types

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