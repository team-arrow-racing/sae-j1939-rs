//! # Parameter group number

/// Parameter group number variant representation
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Pgn {
    Destination(Number),
    Broadcast(Number),
}

impl Pgn {
    /// Create a new PGN.
    ///
    /// This does not explicitly check the resulting variant of the PGN like
    /// `new_destination()` and `new_broadcast()`.
    pub const fn new(number: Number) -> Self {
        match number.format < 240 {
            true => Pgn::Destination(number),
            false => Pgn::Broadcast(number),
        }
    }

    /// Create a new destination address PGN.
    ///
    /// Returns Err if the format is not below 240.
    pub const fn new_destination(number: Number) -> Result<Self, &'static str> {
        match number.format < 240 {
            true => Ok(Pgn::Destination(number)),
            false => Err("format must be below 240 for destination PGNs."),
        }
    }

    /// Create a new broadcast PGN.
    ///
    /// Returns Err if the format is not above 239.
    pub const fn new_broadcast(number: Number) -> Result<Self, &'static str> {
        match number.format >= 240 {
            true => Ok(Pgn::Destination(number)),
            false => Err("format must be above 239 for broadcast PGNs."),
        }
    }

    pub fn to_bits(self) -> u32 {
        match self {
            Pgn::Broadcast(v) => v.to_bits(),
            Pgn::Destination(v) => v.to_bits(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Number {
    pub specific: u8,
    pub format: u8,
    pub data_page: bool,
    pub extended_data_page: bool,
}

impl Number {
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
        assert_eq!(Number::default().to_bits(), 0);
    }

    #[test]
    fn destination_address() {
        let pgn = Pgn::new(Number {
            specific: 0,
            format: 0,
            data_page: false,
            extended_data_page: false,
        });

        // if the format number is below 240, this should return the
        // destination address
        assert_eq!(
            Pgn::Destination(Number {
                ..Default::default()
            }),
            pgn
        );
    }

    #[test]
    fn broadcast_group_extension() {
        let pgn = Pgn::new(Number {
            specific: 0,
            format: 0,
            data_page: false,
            extended_data_page: false,
        });

        // if the format number is below 240, this should return the
        // destination address
        assert_eq!(
            Pgn::Destination(Number {
                ..Default::default()
            }),
            pgn
        );
    }
}
