#![doc = include_str!("../README.md")]
#![no_std]
#![deny(warnings, missing_docs)]

#[macro_use]
mod macros;
pub mod types;

/// A layout of a digit data type in memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct DigitLayout(u32);

/// The content of a digit layout.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LayoutContent {
    /// An unsigned integer type.
    Unsigned {
        /// The width of the integer in bits.
        width: u32,
    },
    /// A real number type.
    Real {
        /// The width of the exponent in bits.
        exponent: u32,
        /// The width of the mantissa in bits.
        mantissa: u32,
    },
    /// A named type.
    Named {
        /// The name of the type.
        name: [u8; 8],
    },
}

#[repr(u32)]
enum DigitLayoutType {
    Unsigned = 0xe0_00_00_00, // 0b111...
    Real = 0xc0_00_00_00,     // 0b110...
    Named = 0,                // 0b...
}
const HEAD: u32 = DigitLayoutType::Unsigned as _;

impl DigitLayout {
    /// Create a new digit layout for an unsigned integer type.
    #[inline]
    pub const fn unsigned(width: u32) -> Self {
        assert!(width & HEAD == 0);
        Self::new(DigitLayoutType::Unsigned, width)
    }

    /// Create a new digit layout for a real number type.
    #[inline]
    pub const fn real(exponent: u32, mantissa: u32) -> Self {
        let body = (exponent << 16) | mantissa;
        assert!(body & HEAD == 0);
        Self::new(DigitLayoutType::Real, body)
    }

    /// Create a new digit layout for a named type.
    pub const fn named(name: &str) -> Self {
        let mut exp = 1;
        let mut bytes = name.as_bytes();
        let mut body = 0;
        while let [b, tail @ ..] = bytes {
            bytes = tail;

            let b = match b {
                b'0'..=b'9' => *b - b'0',
                b'a'..=b'z' => *b - b'a' + 10,
                b'A'..=b'Z' => *b - b'A' + 10,
                b'_' | b'.' => continue,
                _ => panic!("Invalid character in digit name"),
            };
            body += (b as u32 + 1) * exp;
            const GUARD: u32 = 0xc0_00_00_00; // 0b110...
            assert!(body & GUARD != GUARD);
            assert!(exp & GUARD != GUARD);
            exp *= 37; // 37 = 10 + 26 + 1
        }
        Self::new(DigitLayoutType::Named, body)
    }

    #[inline(always)]
    const fn new(ty: DigitLayoutType, body: u32) -> Self {
        Self((ty as u32) | body)
    }

    /// Raw transmutation to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }

    /// Decode the content of the digit layout.
    #[inline]
    pub const fn decode(self) -> LayoutContent {
        const UNSIGNED: u32 = DigitLayoutType::Unsigned as _;
        const SIGNED: u32 = DigitLayoutType::Real as _;

        let head = self.0 & HEAD;
        match head {
            UNSIGNED => LayoutContent::Unsigned {
                width: self.0 & !HEAD,
            },
            SIGNED => LayoutContent::Real {
                exponent: ((self.0 & !HEAD) >> 16) & 0xff,
                mantissa: self.0 & 0xffff,
            },
            _ => {
                let mut name = [0; 8];
                let mut body = self.0;
                let mut i = 0;
                while body > 0 {
                    let b = (body % 37) as u8 - 1;
                    name[i] = b + if b < 10 { b'0' } else { b'a' - 10 };
                    body /= 37;
                    i += 1;
                }
                LayoutContent::Named { name }
            }
        }
    }
}

use core::fmt;

impl fmt::Display for DigitLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LayoutContent::*;
        match self.decode() {
            Unsigned { width } => write!(f, "u{width}"),
            Real { exponent, mantissa } => {
                write!(f, "f{}(e{exponent}m{mantissa})", 1 + exponent + mantissa)
            }
            Named { name } => {
                for c in name {
                    if c == 0 {
                        break;
                    }
                    write!(f, "{}", c as char)?;
                }
                Ok(())
            }
        }
    }
}

#[test]
fn test_unsigned() {
    assert!(matches!(
        types::U8.decode(),
        LayoutContent::Unsigned { width: 8 }
    ));

    assert!(matches!(
        types::U16.decode(),
        LayoutContent::Unsigned { width: 16 }
    ));

    assert!(matches!(
        types::U32.decode(),
        LayoutContent::Unsigned { width: 32 }
    ));

    assert!(matches!(
        types::U64.decode(),
        LayoutContent::Unsigned { width: 64 }
    ));
}

#[test]
fn test_real() {
    assert!(matches!(
        types::I8.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 8,
        }
    ));

    assert!(matches!(
        types::I16.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 16,
        }
    ));

    assert!(matches!(
        types::I32.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 32,
        }
    ));

    assert!(matches!(
        types::I64.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 64,
        }
    ));

    assert!(matches!(
        types::F16.decode(),
        LayoutContent::Real {
            exponent: 5,
            mantissa: 10,
        }
    ));

    assert!(matches!(
        types::BF16.decode(),
        LayoutContent::Real {
            exponent: 8,
            mantissa: 7,
        }
    ));

    assert!(matches!(
        types::F32.decode(),
        LayoutContent::Real {
            exponent: 8,
            mantissa: 23,
        }
    ));

    assert!(matches!(
        types::F64.decode(),
        LayoutContent::Real {
            exponent: 11,
            mantissa: 52,
        }
    ));
}

#[test]
fn test_named() {
    assert!(matches!(
        types::Bool.decode(),
        LayoutContent::Named {
            name: [b'b', b'o', b'o', b'l', 0, 0, 0, 0]
        }
    ));

    let q8_0 = DigitLayout::named("Q8_0");
    assert!(matches!(
        q8_0.decode(),
        LayoutContent::Named {
            name: [b'q', b'8', b'0', 0, 0, 0, 0, 0]
        }
    ));

    let iq2xxs = DigitLayout::named("IQ2XXS");
    assert!(matches!(
        iq2xxs.decode(),
        LayoutContent::Named {
            name: [b'i', b'q', b'2', b'x', b'x', b's', 0, 0]
        }
    ));

    let zzzzzz = DigitLayout::named("zzzzzz");
    assert!(matches!(
        zzzzzz.decode(),
        LayoutContent::Named {
            name: [b'z', b'z', b'z', b'z', b'z', b'z', 0, 0]
        }
    ));
}
