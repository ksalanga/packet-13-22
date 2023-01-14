use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

/// PacketDatum Enum:
/// Nested Data structure that can take variants:
/// - Vector of other PacketDatum
/// - Integer
/// author: Kenneth Salanga
///
/// Contains:
/// - Parser Module
///     - takes a nested list of ints string & ouputs a List PacketDatum Variant

// we a have a packet that contains a list of packet blocks:
// those packet blocks can be: An integer, or another list of packet blocks.
#[derive(PartialEq, Eq, Clone)]
pub enum PacketDatum {
    List(Vec<Rc<RefCell<PacketDatum>>>),
    Integer(i32),
}

impl PacketDatum {
    #[allow(dead_code)]
    fn list(list: Vec<Rc<RefCell<PacketDatum>>>) -> PacketDatum {
        let mut packet_datum_list = PacketDatum::List(vec![]);

        for packet_datum in list {
            packet_datum_list.add_list(packet_datum);
        }

        packet_datum_list
    }

    #[allow(dead_code)]
    fn int_list(list: Vec<i32>) -> PacketDatum {
        PacketDatum::List(
            list.iter()
                .map(|i| Rc::new(RefCell::new(PacketDatum::Integer(*i))))
                .collect(),
        )
    }

    fn add_list(&mut self, packet_datum: Rc<RefCell<PacketDatum>>) {
        match self {
            PacketDatum::List(l) => {
                l.push(packet_datum);
            }
            PacketDatum::Integer(_) => panic!("cannot add item to Integer PacketDatum"),
        }
    }

    #[allow(dead_code)]
    fn rc_i_list(list: Vec<i32>) -> Rc<RefCell<PacketDatum>> {
        Rc::new(RefCell::new(PacketDatum::int_list(list)))
    }

    #[allow(dead_code)]
    fn rc_int(i: i32) -> Rc<RefCell<PacketDatum>> {
        Rc::new(RefCell::new(PacketDatum::Integer(i)))
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
                let l2 = vec![Rc::new(RefCell::new(PacketDatum::Integer(*i2)))];
                l1.cmp(&l2)
            }
            (Self::Integer(i1), Self::List(l2)) => {
                let l1 = vec![Rc::new(RefCell::new(PacketDatum::Integer(*i1)))];
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
