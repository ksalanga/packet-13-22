use crate::datum::PacketDatum;
use std::str::FromStr;

/// PacketParser converts a string that represents a packet of datums: [[1], 2, 3]
/// into a Vector of PacketDatums
/// author: Kenneth Salanga

// Initial Notes:

// Optimal Bracket algorithm that gets us:
// - the bracket pair indices
// - the bracket hierarchy tree
// - cleans out any incorrect bracket formations

// in O(N) time where N is the size of the string

// Observations:
// 1. Closing bracket ALWAYS pairs with the innermost opening bracket up to that point.
//      - it can't get the second innermost or third innermost up to that point because:
//      - if we get anything other than the inner most bracket, we get [[], [[[], etc. this is an invalid bracket pair.
// 2. Any bracket has one or no parent
// 3. Any bracket has zero or more children

// what our algorithm is going to do is:
// create an opening brackets stack.

// starting from the left of the string going right until end:
// - keep adding opening brackets to the stack
// - if we encounter a closing bracket:
//      - if the stack is empty:
//          - INVALID bracket string.
//          - Closing bracket must always accompany an innermost opening bracket up to that point.
//      - else pop the top of the opening bracket stack:
//          - From our observation: this is the inner most bracket up to that point of the closing bracket
//          - This gets us two important things:
//              1. The starting and ending index for this bracket pair
//              2. We know that the parent of this bracket pair is the next innermost opening bracket index from the stack
//                  - if the next innermost is empty: then the parent is the root
//                  - else: peak the top and make that opening bracket index its parent
//                  - for a bracket pair to be a child of some outer bracket, it must be exactly one inner level from the outer bracket.
//                  - the stack exactly shows us this relationship.
//                  - up to this point of the bracket pair:
//                      - the innermost opening bracket is the child, and the second innermost is its one outer level parent.

// - if opening brackets stack is not empty: invalid bracket string.
//      - every opening bracket has a close bracket pair

pub struct PacketParserError;

pub struct PacketParser {
    packet: Vec<PacketDatum>,
}

impl FromStr for PacketParser {
    type Err = PacketParserError;

    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
    }
}
