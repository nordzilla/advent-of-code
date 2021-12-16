use aoc_runner_derive::*;
use flow_control::return_if;

type Input = Vec<u8>;
type Output = usize;

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

// =====================================================================================
// Types
// =====================================================================================

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

impl Header {
    fn new(version: usize, type_id: PacketType) -> Self {
        Self { version, type_id }
    }
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

#[derive(Debug, Clone)]
struct Packet {
    header: Header,
    value: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn literal(header: Header, segments: Vec<LiteralSegment>) -> Self {
        Self {
            header,
            sub_packets: Vec::new(),
            value: segments
                .into_iter()
                .rev()
                .zip((0..).step_by(4))
                .fold(0, |n, (segment, place_value)| {
                    n | segment.shift_left(place_value)
                }),
        }
    }

    fn operator(header: Header, sub_packets: Vec<Packet>) -> Self {
        Self {
            header,
            value: 0,
            sub_packets,
        }
    }

    fn version_sum(&self) -> usize {
        self.header.version
            + self
                .sub_packets
                .iter()
                .map(Packet::version_sum)
                .sum::<usize>()
    }

    fn eval(&self) -> usize {
        let mut sub_packet_values = self.sub_packets.iter().map(Packet::eval);
        match self.header.type_id {
            PacketType::Sum => sub_packet_values.sum(),
            PacketType::Product => sub_packet_values.product(),
            PacketType::Minimum => sub_packet_values.min().unwrap(),
            PacketType::Maximum => sub_packet_values.max().unwrap(),
            PacketType::Literal => self.value,
            PacketType::GreaterThan => {
                (sub_packet_values.next().unwrap() > sub_packet_values.next().unwrap()) as usize
            }
            PacketType::LessThan => {
                (sub_packet_values.next().unwrap() < sub_packet_values.next().unwrap()) as usize
            }
            PacketType::EqualTo => {
                (sub_packet_values.next().unwrap() == sub_packet_values.next().unwrap()) as usize
            }
        }
    }
}

// =====================================================================================
// Parser combinator primitives
// =====================================================================================

type ParseOption<'a, T> = Option<(T, &'a [u8])>;

trait MapFront<Front, Mapped> {
    type Output;
    fn map_front(self, f: impl Fn(Front) -> Mapped) -> Self::Output;
}

impl<Front, Mapped, Back> MapFront<Front, Mapped> for Option<(Front, Back)> {
    type Output = Option<(Mapped, Back)>;
    fn map_front(self, f: impl Fn(Front) -> Mapped) -> Self::Output {
        self.map(|(front, back)| (f(front), back))
    }
}

fn take(n: usize, bits: &[u8]) -> ParseOption<&[u8]> {
    (n <= bits.len()).then(|| bits.split_at(n))
}

fn take_as_number(n: usize, bits: &[u8]) -> ParseOption<usize> {
    take(n, bits).map_front(number_from)
}

fn require<T, PredicateFn, ParseFn>(
    predicate: PredicateFn,
    parse: ParseFn,
    bits: &[u8],
) -> ParseOption<T>
where
    PredicateFn: Fn(&T) -> bool,
    ParseFn: Fn(&[u8]) -> ParseOption<T>,
{
    parse(bits).filter(|(parsed, _)| predicate(parsed))
}

fn parse_exactly<T, ParseFn>(n: usize, parse: ParseFn, mut bits: &[u8]) -> ParseOption<Vec<T>>
where
    ParseFn: Fn(&[u8]) -> ParseOption<T>,
{
    let mut values = Vec::new();
    while let Some((parsed, tail)) = parse(bits) {
        bits = tail;
        values.push(parsed);
        return_if!(values.len() == n, Some((values, bits)));
    }
    None
}

fn parse_one_or_more<T, ParseFn>(parse: ParseFn, mut bits: &[u8]) -> ParseOption<Vec<T>>
where
    ParseFn: Fn(&[u8]) -> ParseOption<T>,
{
    let mut values = Vec::new();
    while let Some((parsed, tail)) = parse(bits) {
        bits = tail;
        values.push(parsed);
    }
    (!values.is_empty()).then(|| (values, bits))
}

fn parse_until<T, PredicateFn, ParseFn>(
    predicate: PredicateFn,
    parse: ParseFn,
    mut bits: &[u8],
) -> ParseOption<Vec<T>>
where
    PredicateFn: Fn(&T) -> bool,
    ParseFn: Fn(&[u8]) -> ParseOption<T>,
{
    let mut values = Vec::new();
    while let Some((parsed, tail)) = parse(bits) {
        bits = tail;
        values.push(parsed);
        return_if!(predicate(values.last().unwrap()), Some((values, bits)));
    }
    None
}

// =====================================================================================
// Parsers for types
// =====================================================================================

fn parse_version(bits: &[u8]) -> ParseOption<usize> {
    take_as_number(3, bits)
}

fn parse_packet_type(bits: &[u8]) -> ParseOption<PacketType> {
    take_as_number(3, bits).map_front(|n| match n {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        4 => PacketType::Literal,
        5 => PacketType::GreaterThan,
        6 => PacketType::LessThan,
        7 => PacketType::EqualTo,
        n => panic!("Unsupported packet type! {}", n),
    })
}

fn parse_header(bits: &[u8]) -> ParseOption<Header> {
    parse_version(bits).and_then(|(version, bits)| {
        parse_packet_type(bits).map_front(|type_id| Header::new(version, type_id))
    })
}

fn parse_packet_length(bits: &[u8]) -> ParseOption<PacketLength> {
    take_as_number(1, bits).and_then(|(bit, bits)| match bit {
        0 => take_as_number(15, bits).map_front(PacketLength::Bits),
        1 => take_as_number(11, bits).map_front(PacketLength::Packets),
        _ => None,
    })
}

fn parse_literal_segment(bits: &[u8]) -> ParseOption<LiteralSegment> {
    take_as_number(1, bits).and_then(|(bit, bits)| match bit {
        0 => take_as_number(4, bits).map_front(LiteralSegment::End),
        1 => take_as_number(4, bits).map_front(LiteralSegment::Part),
        _ => None,
    })
}

fn parse_packet(bits: &[u8]) -> ParseOption<Packet> {
    parse_literal_packet(bits).or_else(|| parse_operator_packet(bits))
}

fn parse_literal_packet(bits: &[u8]) -> ParseOption<Packet> {
    require(|header| header.type_id.is_literal(), parse_header, bits).and_then(|(header, bits)| {
        parse_until(
            |segment| matches!(segment, LiteralSegment::End(_)),
            parse_literal_segment,
            bits,
        )
        .map_front(|segments| Packet::literal(header, segments))
    })
}

fn parse_operator_packet(bits: &[u8]) -> ParseOption<Packet> {
    require(|header| header.type_id.is_operator(), parse_header, bits).and_then(|(header, bits)| {
        parse_packet_length(bits)
            .and_then(|(length, bits)| match length {
                PacketLength::Bits(n) => take(n, bits).and_then(|(sub_packet_bits, bits)| {
                    parse_one_or_more(parse_packet, sub_packet_bits)
                        .map(|(sub_packets, _)| (sub_packets, bits))
                }),
                PacketLength::Packets(n) => parse_exactly(n, parse_packet, bits),
            })
            .map_front(|sub_packets| Packet::operator(header, sub_packets))
    })
}

// =====================================================================================
// Top-level parser
// =====================================================================================

fn parse(bits: &[u8]) -> Option<Packet> {
    parse_packet(bits).map(|(packet, _)| packet)
}

#[aoc(day16, part1, nordzilla)]
fn solve_part1(bits: &Input) -> Output {
    parse(bits).unwrap().version_sum()
}

#[aoc(day16, part2, nordzilla)]
fn solve_part2(bits: &Input) -> Output {
    parse(bits).unwrap().eval()
}
