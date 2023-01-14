use super::*;

#[test]
fn empty_packet_comparison() {
    let packet_1: Vec<PacketDatum> = vec![];
    let packet_2: Vec<PacketDatum> = vec![];

    assert!(packet_1 == packet_2);
}

#[test]
fn packet_datum_new_list() {
    let packet_1 = vec![PacketDatum::list(vec![8, 7, 6])];

    let packet_2 = vec![PacketDatum::list(vec![8, 7, 6])];

    assert!(packet_1 == packet_2);
}

mod all_int_packet_datums {
    use crate::datum::PacketDatum as pd;
    // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
    // packet 2: [1,1,5,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

    // packet 1 < packet 2
    #[test]
    fn same_length_packets_less_than_comparison() {
        let packet_1 = pd::list(vec![1, 1, 3, 1, 1]);
        let packet_2 = pd::list(vec![1, 1, 5, 1, 1]);

        assert!(packet_1 < packet_2);
    }

    // packet 1: [1,1,5,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
    // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

    // packet 1 > packet 2
    #[test]
    fn same_length_packets_greater_than_comparison() {
        let packet_1 = pd::list(vec![1, 1, 5, 1, 1]);

        let packet_2 = pd::list(vec![1, 1, 3, 1, 1]);

        assert!(packet_1 > packet_2);
    }

    // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
    // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

    // packet 1 == packet 2
    #[test]
    fn same_length_packets_equal_comparison() {
        let packet_1 = pd::list(vec![1, 1, 3, 1, 1]);

        let packet_2 = pd::list(vec![1, 1, 3, 1, 1]);

        assert!(packet_1 == packet_2);
    }

    // packet 1: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]
    // packet 2: [1,1,5] => Vec<PacketDatum>: [Integer, Integer, Integer...]

    // packet 1 < packet 2
    #[test]
    fn longer_packet_less_than_shorter_packet_comparison() {
        let packet_1 = pd::list(vec![1, 1, 3, 1, 1]);
        let packet_2 = pd::list(vec![1, 1, 5]);

        assert!(packet_1 < packet_2);
    }

    // packet 1: [1,1,3] => Vec<PacketDatum>: [Integer, Integer, Integer...]
    // packet 2: [1,1,3,1,1] => Vec<PacketDatum>: [Integer, Integer, Integer...]

    // packet 1 < packet 2
    #[test]
    fn prefix_packet_less_than_comparison() {
        let packet_1 = pd::list(vec![1, 1, 3]);
        let packet_2 = pd::list(vec![1, 1, 3, 1, 1]);

        assert!(packet_1 < packet_2);
    }
}

mod all_int_list_packet_datums {
    use crate::datum::PacketDatum as pd;

    // packet 1: [[1],[2,3,4]] => Vec<PacketDatum>: [List, List]
    // packet 2: [[1],[2,4,4]] => Vec<PacketDatum>: [List, List]

    // packet 1 < packet 2
    #[test]
    fn same_length_packets_less_than_comparison() {
        let packet_1 = vec![pd::list(vec![1]), pd::list(vec![2, 3, 4])];

        let packet_2 = vec![pd::list(vec![1]), pd::list(vec![2, 4, 4])];

        assert!(packet_1 < packet_2);
    }

    // packet 1: [[5]] => Vec<PacketDatum>: [List]
    // packet 2: [[8,7,6]] => Vec<PacketDatum>: [List]

    // packet 1 < packet 2
    #[test]
    fn shorter_length_packet_less_than_longer_length_packet_comparison() {
        let packet_1 = vec![pd::list(vec![5])];

        let packet_2 = vec![pd::list(vec![8, 7, 6])];

        assert!(packet_1 < packet_2);
    }

    // packet 1: [[9]] => Vec<PacketDatum>: [List]
    // packet 2: [[8,7,6]] => Vec<PacketDatum>: [List]

    // packet 1 > packet 2
    #[test]
    fn shorter_length_packet_greater_than_longer_length_packet_comparison() {
        let packet_1 = vec![pd::list(vec![9])];

        let packet_2 = vec![pd::list(vec![8, 7, 6])];
        assert!(packet_1 > packet_2);
    }
}

mod mixed_packet_datums {
    use crate::datum::PacketDatum as pd;

    #[test]
    // packet 1: [9]
    // packet 2: [[8,7,6]]
    // packet 1 > packet 2
    fn int_greater_than_list() {
        let packet_1 = vec![pd::Integer(9)];
        let packet_2 = vec![pd::list(vec![8, 7, 6])];

        assert!(packet_1 > packet_2);
    }

    #[test]
    // packet 1: [5]
    // packet 2: [[8,7,6]]
    // packet 1 < packet 2
    fn int_less_than_list() {
        let packet_1 = vec![pd::Integer(5)];
        let packet_2 = vec![pd::list(vec![8, 7, 6])];

        assert!(packet_1 < packet_2);
    }

    #[test]
    // packet 1: [[8,7,6]]
    // packet 2: [10]
    // packet 1 < packet 2
    fn list_less_than_int() {
        let packet_1 = vec![pd::list(vec![8, 7, 6])];
        let packet_2 = vec![pd::Integer(10)];

        assert!(packet_1 < packet_2);
    }

    #[test]
    // packet 1: [[8,7,6]]
    // packet 2: [5]
    // packet 1 > packet 2
    fn list_greater_than_int() {
        let packet_1 = vec![pd::list(vec![8, 7, 6])];
        let packet_2 = vec![pd::Integer(5)];

        assert!(packet_1 > packet_2);
    }
}

mod advent_of_code_examples_comparisons {
    use super::*;
    use crate::datum::PacketDatum as pd;
    // already tested pair 1 in all_int_packet_datums::same_length_packets_less_than_comparison test

    #[test]
    // [[1],[2,3,4]] vs [[1],4]
    // left side smaller
    fn pair_2() {
        let packet_1 = vec![pd::list(vec![1]), pd::list(vec![2, 3, 4])];

        let packet_2 = vec![pd::list(vec![1]), PacketDatum::Integer(4)];

        assert!(packet_1 < packet_2);
    }

    #[test]
    // [9] vs [[8,7,6]]
    // Right side is smaller, so inputs are not in the right order
    fn pair_3() {
        let packet_1 = vec![pd::Integer(9)];
        let packet_2 = vec![pd::list(vec![8, 7, 6])];

        assert!(packet_1 > packet_2)
    }

    #[test]
    // [[4,4],4,4] vs [[4,4],4,4,4]
    // Left side ran out of items, so inputs are in the right order
    fn pair_4() {
        let packet_1 = vec![pd::list(vec![4, 4]), pd::Integer(4), pd::Integer(4)];
        let packet_2 = vec![
            pd::list(vec![4, 4]),
            pd::Integer(4),
            pd::Integer(4),
            pd::Integer(4),
        ];

        assert!(packet_1 < packet_2);
    }

    #[test]
    // [7,7,7,7] vs [7,7,7]
    // Right side ran out of items, so inputs are not in the right order (left greater than right)
    fn pair_5() {
        let packet_1 = vec![
            pd::Integer(7),
            pd::Integer(7),
            pd::Integer(7),
            pd::Integer(7),
        ];
        let packet_2 = vec![pd::Integer(7), pd::Integer(7), pd::Integer(7)];

        assert!(packet_1 > packet_2);
    }

    #[test]
    // [] vs [3]
    // Left side ran out of items, so inputs are in the right order
    fn pair_6() {
        let packet_1: Vec<PacketDatum> = vec![];
        let packet_2 = vec![pd::Integer(3)];

        assert!(packet_1 < packet_2);
    }

    #[test]
    // [[[]]] vs [[]]
    // Right side ran out of items, so inputs are not in the right order
    fn pair_7() {
        let mut list_of_list = pd::list(vec![]);
        list_of_list.add_list(Rc::new(RefCell::new(pd::list(vec![]))));

        let packet_1: Vec<PacketDatum> = vec![list_of_list];
        let packet_2 = vec![pd::list(vec![])];

        assert!(packet_1 > packet_2);
    }

    #[test]
    // [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
    // Right side is smaller, so inputs are not in the right order
    fn pair_8() {
        fn nested_list(inside_list: Vec<i32>) -> PacketDatum {
            let five_six_seven = pd::list(inside_list);
            let mut four = pd::list(vec![4]);
            four.add_list(Rc::new(RefCell::new(five_six_seven)));
            let mut three = pd::list(vec![3]);
            three.add_list(Rc::new(RefCell::new(four)));
            let mut two = pd::list(vec![2]);
            two.add_list(Rc::new(RefCell::new(three)));
            let mut one = pd::list(vec![1]);
            one.add_list(Rc::new(RefCell::new(two)));
            one.add_list(Rc::new(RefCell::new(PacketDatum::Integer(8))));
            one.add_list(Rc::new(RefCell::new(PacketDatum::Integer(9))));
            one
        }

        let packet_1 = vec![nested_list(vec![5, 6, 7])];
        let packet_2 = vec![nested_list(vec![5, 6, 0])];

        assert!(packet_1 > packet_2);
    }
}
