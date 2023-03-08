//! # Protocol data unit
//!

#[derive(Copy, Clone)]
pub struct ProtocolDataUnit {
    format: u8,
    specific: u8,
    broadcast: bool,
}

impl ProtocolDataUnit {
    /// Creates a new data unit without asserting anything.
    pub fn new_unchecked(format: u8, specific: u8) -> Self {
        Self {
            format,
            specific,
            broadcast: format > 239,
        }
    }

    /// Creates a new broadcast data unit.
    pub fn new_broadcast(format: u8, group_extension: u8) -> Result<Self, &'static str> {
        if format > 239 {
            return Err("format must be over 239 (0xEF)");
        }
        assert!(format >= 240);

        Ok(Self {
            format,
            specific: group_extension,
            broadcast: true,
        })
    }

    /// Creates a new addressable data unit.
    pub fn new_addressable(format: u8, destination: u8) -> Result<Self, &'static str> {
        if format < 240 {
            return Err("format must be below 240 (0xF0)");
        }

        Ok(Self {
            format,
            specific: destination,
            broadcast: false,
        })
    }

    /// PDU format byte.
    pub fn format(self) -> u8 {
        self.format
    }

    /// PDU specific byte.
    pub fn specific(self) -> u8 {
        self.specific
    }

    /// Returns true if the message is formatted to be broadcast.
    pub fn broadcast(self) -> bool {
        self.broadcast
    }
}
