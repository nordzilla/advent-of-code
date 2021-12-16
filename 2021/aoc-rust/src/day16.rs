#![allow(unused)]

use aoc_runner_derive::*;
use flow_control::{break_if, return_if};
type Input = Vec<u8>;
type Output = usize;
use std::collections::BTreeSet;

// first 3 bits are the packet version -- number
// next 3 bits encode the packet type ID -- number
// // TypeId(4) == literal value
// // // split into groups of 5 bits, starting with 1, except the last which starts with 0
// // TypeId(ohter) == operator packet
// // // bit after the header is the length type ID
// // // // if 0, then the next 15 bits are a number that is the length of sub-packets in bits
// // // // if 1, then the next 11 bits are a number that is the count of sub-packets in this packet

const BINARY: [[u8; 4]; 16] = [
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

fn number_from(bits: &[u8]) -> usize {
    bits.iter()
        .rev()
        .zip(0..)
        .fold(0, |n, (&bit, place_value)| {
            n | (bit as usize) << place_value
        })
}

fn hex_to_binary(byte: u8) -> [u8; 4] {
    match byte {
        b'0' => BINARY[00],
        b'1' => BINARY[01],
        b'2' => BINARY[02],
        b'3' => BINARY[03],
        b'4' => BINARY[04],
        b'5' => BINARY[05],
        b'6' => BINARY[06],
        b'7' => BINARY[07],
        b'8' => BINARY[08],
        b'9' => BINARY[09],
        b'A' => BINARY[10],
        b'B' => BINARY[11],
        b'C' => BINARY[12],
        b'D' => BINARY[13],
        b'E' => BINARY[14],
        b'F' => BINARY[15],
        _ => unreachable!(),
    }
}

#[aoc_generator(day16, part1, nordzilla)]
#[aoc_generator(day16, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .bytes()
        .flat_map(|byte| hex_to_binary(byte))
        .collect()
}

// ====================================================
// Types
// ====================================================

#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl PacketType {
    fn is_literal(self) -> bool {
        matches!(self, PacketType::Literal)
    }

    fn is_operator(self) -> bool {
        !self.is_literal()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PacketLength {
    Bits(usize),
    Packets(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    version: usize,
    type_id: PacketType,
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralSegment {
    Part(usize),
    End(usize),
}

impl LiteralSegment {
    fn shift_left(self, place_value: usize) -> usize {
        match self {
            LiteralSegment::Part(n) => n << place_value,
            LiteralSegment::End(n) => n << place_value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiteralPacket {
    header: Header,
    value: usize,
}

impl LiteralPacket {
    fn new(header: Header, segments: Vec<LiteralSegment>) -> Self {
        Self {
            header,
            value: segments
                .into_iter()
                .rev()
                .zip((0..).step_by(4))
                .fold(0, |n, (segment, place_value)| {
                    n | segment.shift_left(place_value)
                }),
        }
    }
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    header: Header,
    packets: Vec<Packet>,
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal(packet) => packet.header.version,
            Packet::Operator(packet) => {
                packet.header.version
                    + packet
                        .packets
                        .iter()
                        .map(|packet| packet.version_sum())
                        .sum::<usize>()
            }
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal(packet) => packet.value,
            Packet::Operator(packet) => {
                let mut packet_values = packet.packets.iter().map(Packet::eval);
                match packet.header.type_id {
                    PacketType::Sum => packet_values.sum(),
                    PacketType::Product => packet_values.product(),
                    PacketType::Minimum => packet_values.min().unwrap(),
                    PacketType::Maximum => packet_values.max().unwrap(),
                    PacketType::GreaterThan => {
                        (packet_values.next().unwrap() > packet_values.next().unwrap()) as usize
                    }
                    PacketType::LessThan => {
                        (packet_values.next().unwrap() < packet_values.next().unwrap()) as usize
                    }
                    PacketType::EqualTo => {
                        (packet_values.next().unwrap() == packet_values.next().unwrap()) as usize
                    }
                    PacketType::Literal => panic!("at the disco!"),
                }
            }
        }
    }
}

// ====================================================
// Parse primitives
// ====================================================

type ParseResult<'a, T> = Option<(T, &'a [u8])>;

fn empty_input(bits: &[u8]) -> bool {
    bits.is_empty() || bits.iter().all(|&bit| bit == 0)
}

fn take(n: usize, bits: &[u8]) -> ParseResult<&[u8]> {
    return_if!(bits.len() < n, None);
    Some(bits.split_at(n))
}

fn take_as_number(n: usize, bits: &[u8]) -> ParseResult<usize> {
    take(n, bits).map(|(head, tail)| (number_from(head), tail))
}

fn require<T, PredicateFn, ParseFn>(
    predicate: PredicateFn,
    parse: ParseFn,
    mut bits: &[u8],
) -> ParseResult<T>
where
    PredicateFn: Fn(&T) -> bool,
    ParseFn: Fn(&[u8]) -> ParseResult<T>,
{
    parse(bits).filter(|(out, _)| predicate(out))
}

fn parse_many<T, ParseFn>(n: usize, parse: ParseFn, mut bits: &[u8]) -> ParseResult<Vec<T>>
where
    ParseFn: Fn(&[u8]) -> ParseResult<T>,
{
    let mut result = Vec::new();
    while let Some((out, tail)) = parse(bits) {
        result.push(out);
        return_if!(result.len() == n, Some((result, tail)));
        bits = tail;
    }
    None
}

fn parse_until<T, PredicateFn, ParseFn>(
    predicate: PredicateFn,
    parse: ParseFn,
    mut bits: &[u8],
) -> ParseResult<Vec<T>>
where
    PredicateFn: Fn(&T) -> bool,
    ParseFn: Fn(&[u8]) -> ParseResult<T>,
{
    let mut result = Vec::new();
    while let Some((out, tail)) = parse(bits) {
        result.push(out);
        return_if!(predicate(result.last().unwrap()), Some((result, tail)));
        bits = tail;
    }
    None
}

// ====================================================
// Parse functions
// ====================================================

fn parse_version(bits: &[u8]) -> ParseResult<usize> {
    take_as_number(3, bits)
}

fn parse_packet_type(bits: &[u8]) -> ParseResult<PacketType> {
    take_as_number(3, bits).map(|(n, tail)| match n {
        0 => (PacketType::Sum, tail),
        1 => (PacketType::Product, tail),
        2 => (PacketType::Minimum, tail),
        3 => (PacketType::Maximum, tail),
        4 => (PacketType::Literal, tail),
        5 => (PacketType::GreaterThan, tail),
        6 => (PacketType::LessThan, tail),
        7 => (PacketType::EqualTo, tail),
        n => panic!("Unsupported packet type! {}", n),
    })
}

fn parse_header(bits: &[u8]) -> ParseResult<Header> {
    parse_version(bits).and_then(|(version, tail)| {
        parse_packet_type(tail).map(|(type_id, tail)| (Header { version, type_id }, tail))
    })
}

fn parse_packet_length(bits: &[u8]) -> ParseResult<PacketLength> {
    take_as_number(1, bits).and_then(|(bit, tail)| match bit {
        0 => take_as_number(15, tail).map(|(n, tail)| (PacketLength::Bits(n), tail)),
        1 => take_as_number(11, tail).map(|(n, tail)| (PacketLength::Packets(n), tail)),
        _ => None,
    })
}

fn parse_literal_segment(bits: &[u8]) -> ParseResult<LiteralSegment> {
    take_as_number(1, bits).and_then(|(bit, tail)| match bit {
        0 => take_as_number(4, tail).map(|(n, tail)| (LiteralSegment::End(n), tail)),
        1 => take_as_number(4, tail).map(|(n, tail)| (LiteralSegment::Part(n), tail)),
        _ => None,
    })
}

fn parse_packet(bits: &[u8]) -> ParseResult<Packet> {
    parse_literal_packet(bits).or_else(|| parse_operator_packet(bits))
}

fn parse_packets(mut bits: &[u8]) -> ParseResult<Vec<Packet>> {
    let mut result = Vec::new();
    while let Some((packet, tail)) = parse_packet(bits) {
        result.push(packet);
        return_if!(empty_input(tail), Some((result, tail)));
        bits = tail;
    }
    None
}

fn parse_literal_packet(bits: &[u8]) -> ParseResult<Packet> {
    require(|header| header.type_id.is_literal(), parse_header, bits).and_then(|(header, tail)| {
        parse_until(
            |segment| matches!(segment, LiteralSegment::End(_)),
            parse_literal_segment,
            tail,
        )
        .map(|(segments, tail)| (Packet::Literal(LiteralPacket::new(header, segments)), tail))
    })
}

fn parse_operator_packet(bits: &[u8]) -> ParseResult<Packet> {
    require(|header| header.type_id.is_operator(), parse_header, bits).and_then(|(header, tail)| {
        parse_packet_length(tail)
            .and_then(|(length, tail)| match dbg!(length) {
                PacketLength::Bits(n) => take(n, tail).and_then(|(head, tail)| {
                    parse_packets(head).map(|(packets, _)| (packets, tail))
                }),
                PacketLength::Packets(n) => parse_many(n, parse_packet, tail),
            })
            .map(|(packets, tail)| (Packet::Operator(OperatorPacket { header, packets }), tail))
    })
}

fn parse(bits: &[u8]) -> Option<Vec<Packet>> {
    parse_packets(bits).map(|(packets, _)| packets)
}

#[aoc(day16, part1, nordzilla)]
fn solve_part1(bits: &Input) -> Output {
    let packets = parse(bits).unwrap();
    packets.into_iter().map(|packet| packet.version_sum()).sum()
}

#[aoc(day16, part2, nordzilla)]
fn solve_part2(bits: &Input) -> Output {
    let packets = parse(bits).unwrap();
    packets.into_iter().map(|packet| packet.eval()).sum()
}
