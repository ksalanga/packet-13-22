use super::PacketDatum;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct PacketParseError;

impl FromStr for PacketDatum {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut open_bracket_indices: Vec<usize> = Vec::new();
        let mut lists: HashMap<usize, Rc<RefCell<PacketDatum>>> = HashMap::new();

        if s.is_empty() {
            return Err(PacketParseError);
        }

        if s.chars().next().unwrap() != '[' {
            return Err(PacketParseError);
        }

        for (idx, token) in get_valid_tokens(s).iter().enumerate() {
            match token.as_str() {
                "[" => {
                    let new_list = Rc::new(RefCell::new(PacketDatum::List(vec![])));

                    if let Some(parent_bracket_idx) = open_bracket_indices.last() {
                        let parent_list = lists.get(parent_bracket_idx).unwrap();
                        parent_list.borrow_mut().add_list(Rc::clone(&new_list));
                    }

                    lists.entry(idx).or_insert(new_list);

                    open_bracket_indices.push(idx);
                }
                "]" => {
                    if open_bracket_indices.is_empty() {
                        return Err(PacketParseError);
                    }

                    open_bracket_indices.pop();
                }
                token => match open_bracket_indices.last() {
                    Some(parent_bracket_idx) => {
                        let i: i32 = token.parse().unwrap();
                        let i = Rc::new(RefCell::new(PacketDatum::Integer(i)));
                        let parent_list = lists.get(parent_bracket_idx).unwrap();
                        parent_list.borrow_mut().add_list(i);
                    }
                    None => return Err(PacketParseError),
                },
            }
        }

        if !open_bracket_indices.is_empty() {
            return Err(PacketParseError);
        }

        let root_list = lists.get(&0).unwrap().borrow().clone();

        Ok(root_list)
    }
}

fn get_valid_tokens(s: &str) -> Vec<String> {
    let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    let mut valid_tokens = vec![];

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' | ']' => valid_tokens.push(c.to_string()),
            '0'..='9' | '-' => {
                let mut int_str = String::from(c);

                while let Some(c) = chars.peek() {
                    if !c.is_numeric() {
                        break;
                    }

                    int_str.push(chars.next().unwrap());
                }

                valid_tokens.push(int_str);
            }
            _ => (),
        }
    }

    valid_tokens
}

#[cfg(test)]
mod tests {
    use crate::datum::PacketDatum;

    #[test]
    fn all_ints() {
        let parsed_list: PacketDatum = "[1,2,3]".parse().unwrap();

        let expected_list = PacketDatum::list(vec![1, 2, 3]);

        assert!(parsed_list == expected_list);
    }

    #[test]
    fn negative_ints() {
        let parsed_list: PacketDatum = "[1,-220,3]".parse().unwrap();

        let expected_list = PacketDatum::list(vec![1, -220, 3]);

        assert!(parsed_list == expected_list);
    }
}
