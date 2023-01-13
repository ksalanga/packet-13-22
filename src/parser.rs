use std::str::FromStr;

/// PacketParser converts a string that represents a packet of datums: [[1], 2, 3]
/// into a Vector of PacketDatums
/// author: Kenneth Salanga

pub struct PacketParserError;

pub struct PacketParser {}

impl FromStr for PacketParser {
    type Err = PacketParserError;

    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
    }
}
