// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod bit;

use std::str::FromStr;

use anyhow::Context;
use anyhow::Error;
use bit::Bit;
use bit::BitVector;

#[derive(Debug)]
pub struct Version([Bit; 3]);

impl Version {
    pub fn read(bv: &mut BitVector) -> Option<Self> {
        bv.take_array().map(Self)
    }
}

#[derive(Debug)]
pub struct PacketType([Bit; 3]);

impl PacketType {
    pub fn read(bv: &mut BitVector) -> Option<Self> {
        bv.take_array().map(Self)
    }
}

#[derive(Debug)]
enum LengthType {
    // Zero
    TotalLength([Bit; 15]),
    // One
    SubPackets([Bit; 11]),
}

impl LengthType {
    pub fn read(bv: &mut BitVector) -> Option<Self> {
        match bv.pop() {
            Some(Bit::Zero) => bv.take_array().map(Self::TotalLength),
            Some(Bit::One) => bv.take_array().map(Self::SubPackets),
            None => None,
        }
    }
}

#[derive(Debug, macros::TryFromStr)]
pub enum Packet {
    Literal {
        version: Version,
        // always [One, Zero, Zero]
        // packet_type: PacketType,
        value: Vec<Bit>,
    },
    Operator {
        version: Version,
        packet_type: PacketType,
        // length_type: LengthType,
        children: Vec<Packet>,
    },
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        match self {
            Self::Literal { version, .. } => version.0.iter().sum(),
            Self::Operator {
                version, children, ..
            } => {
                version.0.iter().sum::<usize>()
                    + children.iter().map(Packet::version_sum).sum::<usize>()
            }
        }
    }

    pub fn evaluate(&self) -> usize {
        match self {
            Self::Literal { value, .. } => value.iter().sum(),
            Self::Operator {
                packet_type,
                children,
                ..
            } => {
                let typ = packet_type.0.iter().sum::<usize>();
                let mut children = children.iter().map(Packet::evaluate);
                match typ {
                    // sum
                    0 => children.sum::<usize>(),
                    // product
                    1 => children.product::<usize>(),
                    // min
                    2 => children.min().unwrap(),
                    // max
                    3 => children.max().unwrap(),
                    // greater than
                    5 => {
                        let lhs = children.next().unwrap();
                        let rhs = children.next().unwrap();
                        assert!(children.next().is_none());
                        if lhs > rhs {
                            1
                        } else {
                            0
                        }
                    }
                    // less than
                    6 => {
                        let lhs = children.next().unwrap();
                        let rhs = children.next().unwrap();
                        assert!(children.next().is_none());
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    // equal to
                    7 => {
                        let lhs = children.next().unwrap();
                        let rhs = children.next().unwrap();
                        assert!(children.next().is_none());
                        if lhs == rhs {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn read(bv: &mut BitVector) -> Option<Self> {
        let version = Version::read(bv)?;
        let packet_type = PacketType::read(bv)?;
        if packet_type.0.iter().sum::<usize>() == 4 {
            let mut value = Vec::new();
            loop {
                let next = bv.pop()?;
                let v = bv.take_array::<4>()?;
                value.extend(v);
                if matches!(next, Bit::Zero) {
                    break;
                }
            }
            Some(Self::Literal {
                version,
                // packet_type,
                value,
            })
        } else {
            let length_type = LengthType::read(bv)?;
            let mut children = Vec::new();
            match length_type {
                LengthType::TotalLength(ref v) => {
                    let need = v.iter().sum::<usize>();
                    let len = bv.len();
                    assert!(
                        need <= len,
                        "invalid state; need {need} bits but only have {len}"
                    );
                    let remain = len - need;
                    while remain < bv.len() {
                        children.push(Packet::read(bv)?);
                    }
                }
                LengthType::SubPackets(ref v) => {
                    let count = v.iter().sum::<usize>();
                    for _ in 0..count {
                        children.push(Packet::read(bv)?);
                    }
                }
            }
            Some(Self::Operator {
                version,
                packet_type,
                // length_type,
                children,
            })
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bv = BitVector::from_str(s)?;
        Packet::read(&mut bv).context("parsing packet from input")
    }
}
