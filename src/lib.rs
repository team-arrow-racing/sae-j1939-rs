#![no_std]
#![allow(dead_code)]

/// 11-bit standard identifier.
pub struct IdStandard {
    priority: u8,
    source_address: u8,
}

impl IdStandard {
    /// Create standard identifier from raw bits.
    pub fn new(raw: u16) -> Self {
        let priority = (raw >> 8) as u8;
        assert!(priority < 8);
        let source_address = raw as u8;

        IdStandard {
            priority,
            source_address,
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

/// 29-bit extended identifier.
pub struct IdExtended {
    priority: u8,
    ext_data_page: bool,
    data_page: bool,
    pdu_format: u8,
    pdu_specific: u8,
    source_address: u8,
}

impl IdExtended {
    /// Create extended identifier from raw bits.
    pub fn new(raw: u32) -> Self {
        let priority = (raw >> 26) as u8;
        assert!(priority < 8);
        let ext_data_page = ((raw >> 25) & 0b1) != 0;
        let data_page = ((raw >> 24) & 0b1) != 0;
        let pdu_format = (raw >> 16) as u8;
        let pdu_specific = (raw >> 8) as u8;
        let source_address = raw as u8;

        IdExtended {
            priority,
            ext_data_page,
            data_page,
            pdu_format,
            pdu_specific,
            source_address,
        }
    }

    /// Convert to raw bits.
    pub fn to_bits(&self) -> u32 {
        assert!(self.priority < 8);

        // ref: J1939-21 Table 1
        let p = self.priority as u32;
        let edp = self.ext_data_page as u32;
        let dp = self.data_page as u32;
        let pf = self.pdu_format as u32;
        let ps = self.pdu_specific as u32;
        let sa = self.source_address as u32;
        p << 26 | edp << 25 | dp << 24 | pf << 16 | ps << 8 | sa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_to_standard_id() {
        let id = IdStandard::new(0x6FE);

        assert_eq!(id.priority, 6);
        assert_eq!(id.source_address, 0xFE);
    }
    
    #[test]
    fn id_standard_to_bits() {
        // Example PGN 0xF333: High Voltage Energy Storage Pack 26 Data 1
        let id = IdStandard {
            priority: 6,
            source_address: 0xFE,
        };
        assert_eq!(id.to_bits(), 0x6FE);
    }

    #[test]
    fn bits_to_extended_id() {
        let id = IdExtended::new(0x0CF004FE);

        assert_eq!(id.priority, 3);
        assert_eq!(id.ext_data_page, false);
        assert_eq!(id.data_page, false);
        assert_eq!(id.pdu_format, 0xF0);
        assert_eq!(id.pdu_specific, 0x04);
        assert_eq!(id.source_address, 0xFE);
    }

    #[test]
    fn id_extended_to_bits() {
        // Example PGN 0xF004: Electronic Engine Controller 1
        let id = IdExtended {
            priority: 3,
            ext_data_page: false,
            data_page: false,
            pdu_format: 0xF0,
            pdu_specific: 0x04,
            source_address: 0xFE,
        };
        assert_eq!(id.to_bits(), 0x0CF004FE);

        // Example PGN 0xF122: DC/DC Converter 4 Control
        let id = IdExtended {
            priority: 6,
            ext_data_page: false,
            data_page: false,
            pdu_format: 0xF1,
            pdu_specific: 0x22,
            source_address: 0xFE,
        };
        assert_eq!(id.to_bits(), 0x18F122FE);
    }
}
