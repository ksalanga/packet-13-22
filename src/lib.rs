/// Advent of Code 2022 Day 13:
///
/// PacketDatum Data Structure Enum
///
/// Learning Rust's Ord Trait
/// By learning how to order nested structures
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

#[derive(PartialEq, Eq, PartialOrd)]
pub enum PacketDatum {
    // List(Box<Vec<PacketDatum>>),
    Integer(i32),
}

impl Ord for PacketDatum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(i1), Self::Integer(i2)) => i1.cmp(&i2),
        }
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

    mod all_ints {
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

    // [[1],[2,3,4]] => Vec<PacketDatum>: [List, List]
    // [[1],4] => Vec<PacketDatum>: [List, Integer]
}
