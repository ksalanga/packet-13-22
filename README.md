# Advent of Code 2022 Day 13 Crate

- ### Learning and implementing Rust's Ord Trait:

  - [Lexicographically](https://en.wikipedia.org/wiki/Lexicographic_order) ordering and comparing nested int list structures

- ### Implementing parsing nested list of lists and/or ints.

  - ex: [1, 2, [2, [3, 4], 4], [7, 9]]

## packet-13-22 crate contains

- ### Datum module
  - PacketDatum Enum:
    - Nested data structure that represents a list of lists and/or ints
  - _Note: I could have abstracted the wording to be a ListItem Enum but in the spirit of the advent of code challenge I made it a PacketDatum since the challenge was getting a signal of packets_
- ### Parser module
  - Parses a nested list string input into a List PacketDatum Variant
