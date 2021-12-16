use std::collections::VecDeque;

use bitvec::prelude::*;

use super::prelude::*;
use crate::util::read_file;

const TYPE_LITERAL: u8 = 4;

#[derive(Debug, Eq, PartialEq)]
enum Data {
    Literal(u64),
    Operator { operator: u8, inner: Vec<Packet> },
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    data: Data,
}

struct Input {
    data: BitBox<u8, Msb0>,
}

impl Input {
    fn from_reader<R: BufRead>(reader: R) -> Input {
        let line = reader.lines().next().unwrap().unwrap();
        let data: BitVec<u8, Msb0> = line
            .as_bytes()
            .chunks_exact(2)
            .map(|chunk| u8::from_str_radix(String::from_utf8_lossy(chunk).as_ref(), 16).unwrap())
            .collect();
        Input {
            data: data.into_boxed_bitslice(),
        }
    }
}

struct Parser<'a> {
    bits: &'a BitSlice<u8, Msb0>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a Input) -> Parser<'a> {
        Parser {
            bits: input.data.as_bitslice(),
        }
    }

    fn consume_bits(&mut self, n: usize) -> Option<&'a BitSlice<u8, Msb0>> {
        if self.bits.len() < n {
            None
        } else {
            let (bits, rest) = self.bits.split_at(n);
            self.bits = rest;
            Some(bits)
        }
    }

    fn sub_parser(&mut self, n: usize) -> Option<Parser<'a>> {
        self.consume_bits(n).map(|bits| Parser { bits })
    }

    fn read_u8(&mut self, n: usize) -> Option<u8> {
        self.consume_bits(n).map(|bits| bits.load_be())
    }

    fn read_u64(&mut self, n: usize) -> Option<u64> {
        self.consume_bits(n).map(|bits| bits.load_be())
    }

    fn parse_version(&mut self) -> Option<u8> {
        self.read_u8(3)
    }

    fn parse_type_id(&mut self) -> Option<u8> {
        self.read_u8(3)
    }

    fn parse_length_type_id(&mut self) -> Option<u8> {
        self.read_u8(1)
    }

    fn parse_literal(&mut self) -> Option<u64> {
        let mut value = 0u64;
        loop {
            let more = self.consume_bits(1)?[0];
            let next = self.read_u64(4)?;
            value <<= 4;
            value += next;
            if !more {
                break;
            }
        }
        Some(value)
    }

    fn parse_sub_packets(&mut self) -> Option<Vec<Packet>> {
        match self.parse_length_type_id()? {
            0 => {
                let length = self.read_u64(15)?;
                let mut sub_parser = self.sub_parser(length as usize)?;
                Some(sub_parser.parse_packets())
            }
            1 => {
                let count = self.read_u64(11)?;
                Some(
                    std::iter::from_fn(|| self.parse_packet())
                        .take(count as usize)
                        .collect(),
                )
            }
            _ => panic!("impossible"),
        }
    }

    fn parse_packet(&mut self) -> Option<Packet> {
        let version = self.parse_version()?;
        let type_id = self.parse_type_id()?;

        if type_id == TYPE_LITERAL {
            Some(Packet {
                version,
                data: Data::Literal(self.parse_literal()?),
            })
        } else {
            let inner = self.parse_sub_packets()?;
            Some(Packet {
                version,
                data: Data::Operator {
                    operator: type_id,
                    inner,
                },
            })
        }
    }

    fn parse_packets(&mut self) -> Vec<Packet> {
        std::iter::from_fn(|| self.parse_packet()).collect()
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let input = Input::from_reader(reader);
    let mut parser = Parser::new(&input);
    let packets = parser.parse_packets();

    let mut sum = 0u64;
    let mut queue: VecDeque<&Packet> = VecDeque::new();
    queue.extend(packets.iter());
    while let Some(packet) = queue.pop_front() {
        sum += packet.version as u64;
        if let Data::Operator { inner, .. } = &packet.data {
            queue.extend(inner.iter());
        }
    }

    Ok(sum.to_string())
}

fn part2<R: BufRead>(_reader: R) -> crate::Result<String> {
    Err(crate::Error::Other("not implemented".into()))
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day16_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day16_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::read_str;

    #[test]
    fn test_parser_basics() {
        let input = Input::from_reader(read_str("D2FE28"));

        let mut parser = Parser::new(&input);
        assert_eq!(parser.parse_version(), Some(6));
        assert_eq!(parser.parse_type_id(), Some(4));
        assert_eq!(parser.parse_literal(), Some(2021));

        let mut parser = Parser::new(&input);
        assert_eq!(
            parser.parse_packet(),
            Some(Packet {
                version: 6,
                data: Data::Literal(2021)
            })
        );
        assert_eq!(parser.parse_packet(), None);
    }

    #[test]
    fn test_parser_sub_packet_length() {
        let input = Input::from_reader(read_str("38006F45291200"));
        let mut parser = Parser::new(&input);
        assert_eq!(
            parser.parse_packet(),
            Some(Packet {
                version: 1,
                data: Data::Operator {
                    operator: 6,
                    inner: vec![
                        Packet {
                            version: 6,
                            data: Data::Literal(10),
                        },
                        Packet {
                            version: 2,
                            data: Data::Literal(20),
                        },
                    ],
                },
            })
        );
    }

    #[test]
    fn test_parser_sub_packet_count() {
        let input = Input::from_reader(read_str("EE00D40C823060"));
        let mut parser = Parser::new(&input);
        assert_eq!(
            parser.parse_packet(),
            Some(Packet {
                version: 7,
                data: Data::Operator {
                    operator: 3,
                    inner: vec![
                        Packet {
                            version: 2,
                            data: Data::Literal(1),
                        },
                        Packet {
                            version: 4,
                            data: Data::Literal(2),
                        },
                        Packet {
                            version: 1,
                            data: Data::Literal(3),
                        },
                    ],
                },
            })
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(read_str("8A004A801A8002F478")).unwrap(), "16");
        assert_eq!(part1(read_str("620080001611562C8802118E34")).unwrap(), "12");
        assert_eq!(
            part1(read_str("C0015000016115A2E0802F182340")).unwrap(),
            "23"
        );
        assert_eq!(
            part1(read_str("A0016C880162017C3686B18A3D4780")).unwrap(),
            "31"
        );
        assert_eq!(part1(read_file("data/day16_input.txt")).unwrap(), "821");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                ???
            "}))
            .unwrap(),
            "???"
        );
        assert_eq!(part2(read_file("data/day16_input.txt")).unwrap(), "???");
    }
}
