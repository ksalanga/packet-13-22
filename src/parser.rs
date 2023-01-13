use std::str::FromStr;

/// PacketParser converts a string that represents a packet of datums: [[1], 2, 3]
/// into a Vector of PacketDatums
/// author: Kenneth Salanga

// Initial Notes:

// How can we match an opening bracket to its respective closing bracket?

// Finding all matching bracket pairs: Assuming we have valid amount of brackets (all opening matches a closing):
// - Starting from the inner most opening bracket:
//      - An opening bracket's closing bracket pair, is the closest closing bracket index THAT HASN'T BEEN SEEN to its right.
//      - put that closing bracket index into a seen index set
//      - continue on to the next innermost bracket
// - This algorithm works, might not be the most efficient
pub struct PacketParserError;

pub struct PacketParser {}

impl FromStr for PacketParser {
    type Err = PacketParserError;

    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
    }
}
