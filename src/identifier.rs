//! # Identifier types

use crate::ParameterGroupNumber;

/// 11-bit standard identifier.
pub struct StandardId {
    /// Source address
    pub source_address: u8,
    /// Message priority
    pub priority: u8,
}

impl StandardId {
    /// Create standard identifier from raw bits.
    pub fn new(raw: u16) -> Self {
        let priority = (raw >> 8) as u8;
        assert!(priority < 8);
        let source_address = raw as u8;

        StandardId {
            source_address,
            priority,
        }
    }

    /// Convert to raw bits.
    pub fn to_bits(&self) -> u16 {
        assert!(self.priority < 8);

        let p = self.priority as u16;
        let sa = self.source_address as u16;

        p << 8 | sa
    }
}

#[cfg(feature = "bxcan")]
impl From<bxcan::StandardId> for StandardId {
    fn from(id: bxcan::StandardId) -> Self {
        StandardId::new(id.as_raw())
    }
}

/// 29-bit extended identifier.
pub struct ExtendedId {
    /// Source address
    pub source_address: u8,
    /// Parameter group number
    pub pgn: ParameterGroupNumber,
    /// Message priority
    pub priority: u8,
}

impl ExtendedId {
    /// Create extended identifier from raw bits.
    pub fn new(raw: u32) -> Self {
        let priority = (raw >> 26) as u8;
        assert!(priority < 8);
        let ext_data_page = ((raw >> 25) & 0b1) != 0;
        let data_page = ((raw >> 24) & 0b1) != 0;
        let pdu_format = (raw >> 16) as u8;
        let pdu_specific = (raw >> 8) as u8;
        let source_address = raw as u8;

        ExtendedId {
            source_address,
            pgn: ParameterGroupNumber {
                specific: pdu_specific,
                format: pdu_format,
                data_page: data_page,
                extended_data_page: ext_data_page,
            },
            priority,
        }
    }

    /// Convert to raw bits.
    pub fn to_bits(&self) -> u32 {
        assert!(self.priority < 8);

        let sa = self.source_address as u32;
        let pgn: u32 = self.pgn.to_bits();
        let p = self.priority as u32;

        p << 26 | pgn << 8 | sa
    }
}

#[cfg(feature = "bxcan")]
impl From<bxcan::ExtendedId> for ExtendedId {
    fn from(id: bxcan::ExtendedId) -> Self {
        ExtendedId::new(id.as_raw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_to_standard_id() {
        let id = StandardId::new(0x6FE);

        assert_eq!(id.priority, 6);
        assert_eq!(id.source_address, 0xFE);
    }

    #[test]
    fn id_standard_to_bits() {
        // Example PGN 0xF333: High Voltage Energy Storage Pack 26 Data 1
        let id = StandardId {
            priority: 6,
            source_address: 0xFE,
        };
        assert_eq!(id.to_bits(), 0x6FE);
    }

    #[test]
    fn bits_to_extended_id() {
        let id = ExtendedId::new(0x0CF004FE);

        assert_eq!(id.priority, 3);
        assert_eq!(id.pgn.extended_data_page, false);
        assert_eq!(id.pgn.data_page, false);
        assert_eq!(id.pgn.format, 0xF0);
        assert_eq!(id.pgn.specific, 0x04);
        assert_eq!(id.source_address, 0xFE);
    }

    #[test]
    fn id_extended_to_bits() {
        // Example PGN 0xF004: Electronic Engine Controller 1
        let id = ExtendedId {
            source_address: 0xFE,
            pgn: ParameterGroupNumber {
                specific: 0x04,
                format: 0xF0,
                data_page: false,
                extended_data_page: false,
            },
            priority: 3,
        };
        assert_eq!(id.to_bits(), 0x0CF004FE);

        // Example PGN 0xF122: DC/DC Converter 4 Control
        let id = ExtendedId {
            source_address: 0xFE,
            pgn: ParameterGroupNumber {
                specific: 0x22,
                format: 0xF1,
                data_page: false,
                extended_data_page: false,
            },
            priority: 6,
        };
        assert_eq!(id.to_bits(), 0x18F122FE);
    }
}
