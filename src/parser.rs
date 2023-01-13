use crate::datum::PacketDatum;
use std::str::FromStr;

/// PacketParser converts a string that represents a packet of datums: [[1], 2, 3]
/// into a Vector of PacketDatums
/// author: Kenneth Salanga

// Initial Notes:

// First: to clean out any incorrectly nested brackets:
// - make a stack of opening brackets: [ and closing brackets: ]
// - if those stacks are unequal lengths:
//      - there isn't a closing bracket for every opening bracket.
//      - that is an incorrectly nested group of brackets.

// How can we match an opening bracket to its respective closing bracket?

// Finding all matching bracket pairs: Assuming we have valid amount of brackets (all opening matches a closing):
// - Starting from the inner most opening bracket:
//      - An opening bracket's closing bracket pair, is the closest closing bracket index THAT HASN'T BEEN SEEN to its right.
//      - put that closing bracket index into a seen index set
//      - continue on to the next innermost bracket
// - This algorithm works, might not be the most efficient
// - This gets us all bracket pairs, but it doesn't show the bracket hierarchy

// What brackets are inside other brackets?
// What brackets are siblings to other brackets?

// 1. a bracket has a single parent
// 2. a bracket can have multiple children

// You are a parent of a bracket pair if:
// your bracket pair overlaps the other bracket pair

// you are a child of a parent if:
// your bracket pair is inside some bracket pair

// This algorithm does not work. counter example:
// [[], [], []], [[], [], []]
// Creating bracket pair hierarchy: Initial Algorithm
// - Starting from the innermost opening bracket, bracket pair as the Current Level Bracket Pair
// - Create a siblings / same level vector
// - While the bracket pairs are not empty,
//      - Pop the next innermost opening bracket, bracket pair:
//          - if popped bracket isn't their parent, they're siblings:
//              - add this popped bracket pair into the siblings vector
//          - if it is their parent:
//              - set all aggregated siblings + the current bracket pair as children to that parent
//              - make the current level bracket pair to this parent. "moved up the tree"

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
