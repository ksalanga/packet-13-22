use std::str::FromStr;

use super::PacketDatum;

/// PacketParser converts a nested list string that represents a packet: [[1], 2, 3]
/// into a List PacketDatum variant
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

// How we can include ints into the tree hierarchy while making our PacketDatum Structures

// Instead of creating a tree structure with just the "bracket pair" nodes,
// We parse the characters in the strings and build the nested structure from the PacketDatum enum.

// PacketDatum enum already contains Variants:
// - List
// - Integer
// so just create the nested "tree" structure from that.
// this will now also include Integers instead of just brackets / List variant.

// New Parsing Algorithm that includes ints:

// Assumes: string starts off as a list. ex: "[ ]"
// if string doesn't start off with [, abort: invalid list string

// Create an opening brackets index stack: Stack<usize> which represents the opening bracket index
// Create a list hashmap: HashMap<usize, Rc<RefCell<PacketDatum::List>>:
// - List can have parent lists so we need
// - quick access to all of the List PacketDatums
// - so that a parent list can add a child list to its vector of "children"

// For each enumerated character in the input string:
//  if char is an opening bracket:
//  - we've encountered a new PacketDatum List
//  - if the bracket stack is empty: this is the root list
//      - put char index key + new Rc<RefCell<PacketDatum::List> into list hashmap
//  - else peek the top: the new char is a child PacketDatum List and the top of the stack is its outer parent List
//      - create new ref list: Rc<RefCell<PacketDatum::List>
//      - put char index key + ref list value into list hashmap
//      - get the parent List reference from the <index, PacketDatum> hashmap
//          - make a copy of ref list and add it as a child to the parent list
//  - put char index into opening brackets index stack
//  if char is an int:
//  - if the stack is empty: INVALID Bracket String. integer should always be inside a list
//  - else: peek the top of the opening bracket index stack. that is the int's parent
//      - get the parent List ref from the <index, PacketDatum> hashmap
//      - add the child Integer PacketDatum to the Parent List
//  if char is a closing bracket:
//      - if stack is empty:
//          - INVALID bracket string.
//          - Closing bracket must always accompany an innermost opening bracket up to that point.
//      - else:
//          - pop the opening brackets index stack.
//          - that is the end (no more PacketDatum items) of that list where the opening bracket is @ the popped index

// if opening brackets stack is not empty: invalid bracket string.
//  - every opening bracket has a close bracket pair

// This should get us the "Tree Hierarchy" where the root PacketDatum List is in the hashmap @ index 0

pub struct PacketParseError;

impl FromStr for PacketDatum {
    type Err = PacketParseError;

    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
    }
}
