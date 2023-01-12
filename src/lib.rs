/// Advent of Code 2022 Day 13:
///
/// Learning Rust's Ord Trait
/// By learning how to order nested structures
///
/// Also learning how to recursively parse a list of list of lists.
///
/// Packet crate that has the:
/// PacketDatum Data Structure Enum
/// TODO: PacketParser
///
/// author: Kenny Salanga
///
// Initial Notes:
// we a have a packet that contains a block of: lists of integers, integers, or nothing

// enum PacketDatum:
// - List(Box<Vec<PacketDatum>>)
// - Integer(i32)

// can only compare lists with lists and integers with integers.
// if we compare a list with an integer, that integer needs to become a list.
use std::cmp::Ordering;

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
}

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

impl PartialOrd for PacketDatum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_packet_comparison() {
        let packet_1: Vec<PacketDatum> = vec![];
        let packet_2: Vec<PacketDatum> = vec![];

        assert!(packet_1 == packet_2);
    }

    #[test]
    fn packet_datum_new_list() {
        let packet_1 = vec![PacketDatum::List(Box::new(vec![
            PacketDatum::Integer(8),
            PacketDatum::Integer(7),
            PacketDatum::Integer(6),
        ]))];

        let packet_2 = vec![PacketDatum::new_list(vec![8, 7, 6])];

        assert!(packet_1 == packet_2);
    }

    mod all_int_packet_datums {
        use super::*;
        // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
        // packet 2: [1,1,5,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

        // packet 1 < packet 2
        #[test]
        fn same_length_packets_less_than_comparison() {
            let packet_1 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];
            let packet_2 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(5),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            assert!(packet_1 < packet_2);
        }

        // packet 1: [1,1,5,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
        // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

        // packet 1 > packet 2
        #[test]
        fn same_length_packets_greater_than_comparison() {
            let packet_1 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(5),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            let packet_2 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            assert!(packet_1 > packet_2);
        }

        // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
        // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

        // packet 1 == packet 2
        #[test]
        fn same_length_packets_equal_comparison() {
            let packet_1 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            let packet_2 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            assert!(packet_1 == packet_2);
        }

        // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
        // packet 2: [1,1,5] => Vec<PacketDatum>: [Integer, Integer, Integer...]

        // packet 1 < packet 2
        #[test]
        fn longer_packet_less_than_shorter_packet_comparison() {
            let packet_1 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];
            let packet_2 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(5),
            ];

            assert!(packet_1 < packet_2);
        }

        // packet 1: [1,1,3] => Vec<PacketDatum>: [Integer, Integer, Integer...]
        // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

        // packet 1 < packet 2
        #[test]
        fn prefix_packet_less_than_comparison() {
            let packet_1 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
            ];
            let packet_2 = vec![
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
                PacketDatum::Integer(3),
                PacketDatum::Integer(1),
                PacketDatum::Integer(1),
            ];

            assert!(packet_1 < packet_2);
        }
    }

    mod all_int_list_packet_datums {
        use super::*;

        // packet 1: [[1],[2,3,4]] => Vec<PacketDatum>: [List, List]
        // packet 2: [[1],[2,4,4]] => Vec<PacketDatum>: [List, List]

        // packet 1 < packet 2
        #[test]
        fn same_length_packets_less_than_comparison() {
            let packet_1 = vec![
                PacketDatum::List(Box::new(vec![PacketDatum::Integer(1)])),
                PacketDatum::List(Box::new(vec![
                    PacketDatum::Integer(2),
                    PacketDatum::Integer(3),
                    PacketDatum::Integer(4),
                ])),
            ];

            let packet_2 = vec![
                PacketDatum::List(Box::new(vec![PacketDatum::Integer(1)])),
                PacketDatum::List(Box::new(vec![
                    PacketDatum::Integer(2),
                    PacketDatum::Integer(4),
                    PacketDatum::Integer(4),
                ])),
            ];

            assert!(packet_1 < packet_2);
        }

        // packet 1: [[5]] => Vec<PacketDatum>: [List]
        // packet 2: [[8,7,6]] => Vec<PacketDatum>: [List]

        // packet 1 < packet 2
        #[test]
        fn shorter_length_packet_less_than_longer_length_packet_comparison() {
            let packet_1 = vec![PacketDatum::List(Box::new(vec![PacketDatum::Integer(5)]))];

            let packet_2 = vec![PacketDatum::List(Box::new(vec![
                PacketDatum::Integer(8),
                PacketDatum::Integer(7),
                PacketDatum::Integer(6),
            ]))];

            assert!(packet_1 < packet_2);
        }

        // packet 1: [[9]] => Vec<PacketDatum>: [List]
        // packet 2: [[8,7,6]] => Vec<PacketDatum>: [List]

        // packet 1 > packet 2
        #[test]
        fn shorter_length_packet_greater_than_longer_length_packet_comparison() {
            let packet_1 = vec![PacketDatum::List(Box::new(vec![PacketDatum::Integer(9)]))];

            let packet_2 = vec![PacketDatum::List(Box::new(vec![
                PacketDatum::Integer(8),
                PacketDatum::Integer(7),
                PacketDatum::Integer(6),
            ]))];

            assert!(packet_1 > packet_2);
        }
    }

    mod mixed_packet_datums {
        use super::*;

        #[test]
        // packet 1: [9]
        // packet 2: [[8,7,6]]
        // packet 1 > packet 2
        fn int_greater_than_list() {
            let packet_1 = vec![PacketDatum::Integer(9)];
            let packet_2 = vec![PacketDatum::List(Box::new(vec![
                PacketDatum::Integer(8),
                PacketDatum::Integer(7),
                PacketDatum::Integer(6),
            ]))];

            assert!(packet_1 > packet_2);
        }

        #[test]
        // packet 1: [5]
        // packet 2: [[8,7,6]]
        // packet 1 < packet 2
        fn int_less_than_list() {
            let packet_1 = vec![PacketDatum::Integer(5)];
            let packet_2 = vec![PacketDatum::List(Box::new(vec![
                PacketDatum::Integer(8),
                PacketDatum::Integer(7),
                PacketDatum::Integer(6),
            ]))];

            assert!(packet_1 < packet_2);
        }
    }
}
