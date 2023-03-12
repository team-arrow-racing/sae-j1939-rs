//! # Protocol data unit
//!

#[derive(Default, Copy, Clone)]
pub struct ParameterGroupNumber {
    pub specific: u8,
    pub format: u8,
    pub data_page: bool,
    pub extended_data_page: bool,
}

impl ParameterGroupNumber {
    pub fn destination_address(self) -> Option<u8> {
        if self.format < 240 {
            Some(self.specific)
        } else {
            None
        }
    }

    pub fn broadcast_group_extension(self) -> Option<u8> {
        if self.format >= 240 {
            Some(self.specific)
        } else {
            None
        }
    }

    pub fn to_bits(self) -> u32 {
        let s = self.specific as u32;
        let f = self.format as u32;
        let d = self.data_page as u32;
        let e = self.extended_data_page as u32;

        e << 17 | d << 16 | f << 8 | s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        // default should leave no bits set
        assert_eq!(ParameterGroupNumber::default().to_bits(), 0);
    }

    #[test]
    fn destination_address() {
        let pgn = ParameterGroupNumber {
            format: 239,
            specific: 0x23,
            ..Default::default()
        };

        // if the format number is below 240, this should return the
        // destination address
        assert_eq!(pgn.destination_address(), Some(0x23));
    }

    #[test]
    fn broadcast_group_extension() {
        let pgn = ParameterGroupNumber {
            format: 240,
            specific: 0x78,
            ..Default::default()
        };

        assert_eq!(pgn.broadcast_group_extension(), Some(0x78));
    }
}
