/// Advent of Code 2022 Day 13 Crate:
/// author: Kenny Salanga
/// 
/// Lexicographically Ordering and Comparing Nested Structures
/// 
/// Crate contains:
/// - Datum Module
///     - PacketDatum enum
///         - nested structure that has lexicographic ordering
/// - Parser Module
///     - PacketParser structure
///         - takes a string: nested list of ints and gives a Vec of PacketDatum
/// 
mod datum;
mod parser;
