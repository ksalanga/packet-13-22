use std::cmp::Ordering;

/// PacketDatum Enum:
/// Nested Data structure that can take variants:
/// - Vector of other PacketDatum
/// - Integer
/// author: Kenneth Salanga

// we a have a packet that contains a list of packet blocks:
// those packet blocks can be: An integer, or another list of packet blocks.
#[derive(PartialEq, Eq)]
pub enum PacketDatum {
    List(Box<Vec<PacketDatum>>),
    Integer(i32),
}

impl PacketDatum {
    pub fn new_list(list: Vec<i32>) -> PacketDatum {
        PacketDatum::List(Box::new(
            list.iter().map(|i| PacketDatum::Integer(*i)).collect(),
        ))
    }

    pub fn add_list(&mut self, packet_datum: PacketDatum) {
        match self {
            PacketDatum::List(l) => {
                l.push(packet_datum);
            }
            PacketDatum::Integer(_) => panic!("cannot add item to Integer PacketDatum"),
        }
    }
}

// can only compare lists with lists and integers with integers.
// if we compare a list with an integer, that integer needs to become a list.
impl Ord for PacketDatum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(i1), Self::Integer(i2)) => i1.cmp(&i2),
            (Self::List(l1), Self::List(l2)) => l1.cmp(l2),
            (Self::List(l1), Self::Integer(i2)) => {
                let l2 = Box::new(vec![PacketDatum::Integer(*i2)]);
                l1.cmp(&l2)
            }
            (Self::Integer(i1), Self::List(l2)) => {
                let l1 = Box::new(vec![PacketDatum::Integer(*i1)]);
                l1.cmp(&l2)
            }
        }
    }
}

// PartialOrd is the trait that replaces <, >, = operators
impl PartialOrd for PacketDatum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

mod parser;

#[cfg(test)]
mod tests;
