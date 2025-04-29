#![doc = include_str!("../README.md")]
#![no_std]
#![deny(warnings, missing_docs)]

#[macro_use]
mod macros;
pub mod types;

#[cfg(test)]
extern crate alloc;

/// A layout of a digit data type in memory.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(C)]
pub struct DigitLayout {
    code: u32,
    group: u16,
    size: u16,
}

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
    Unsigned = 0xE000_0000, // 0b111...
    Real = 0xC000_0000,     // 0b110...
    Named = 0,              // 0b...
}
const UNSIGNED: u32 = DigitLayoutType::Unsigned as u32;
const SIGNED: u32 = DigitLayoutType::Real as u32;
const HEAD: u32 = UNSIGNED;

impl DigitLayout {
    /// Create a new digit layout for an unsigned integer type.
    #[inline]
    pub const fn unsigned(width: u16, group: u16) -> Self {
        assert!(width.is_power_of_two() && width >= 8);

        let body = width as u32;
        assert!(body & HEAD == 0);
        Self::new(DigitLayoutType::Unsigned, body, group, width / 8 * group)
    }

    /// Create a new digit layout for a real number type.
    #[inline]
    pub const fn real(exponent: u16, mantissa: u16, group: u16) -> Self {
        let width = 1 + exponent + mantissa;
        assert!(width.is_power_of_two() && width >= 8);

        let body = ((exponent as u32) << 16) | mantissa as u32;
        assert!(body & HEAD == 0);
        Self::new(DigitLayoutType::Real, body, group, width / 8 * group)
    }

    /// Create a new digit layout for a named type.
    pub const fn named(name: &str, group: u16, size: u16) -> Self {
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
            const GUARD: u32 = 0xC000_0000; // 0b110...
            assert!(body & GUARD != GUARD);
            assert!(exp & GUARD != GUARD);
            exp *= 37; // 37 = 10 + 26 + 1
        }
        Self::new(DigitLayoutType::Named, body, group, size)
    }

    #[inline(always)]
    const fn new(ty: DigitLayoutType, body: u32, group: u16, size: u16) -> Self {
        Self {
            code: (ty as u32) | body,
            group,
            size,
        }
    }

    /// Raw transmutation to `u64`.
    #[inline]
    pub const fn to_u64(self) -> u64 {
        unsafe { core::mem::transmute(self) }
    }

    /// Get the number of bytes occupied by this layout.
    pub const fn group_size(self) -> usize {
        self.group as usize
    }

    /// Get the number of bytes occupied by this layout.
    pub const fn nbytes(self) -> usize {
        self.size as usize
    }

    /// Decode the content of the digit layout.
    pub const fn decode(self) -> LayoutContent {
        let head = self.code & HEAD;
        match head {
            UNSIGNED => LayoutContent::Unsigned {
                width: self.decode_unsigned(),
            },
            SIGNED => LayoutContent::Real {
                exponent: self.decode_exponent(),
                mantissa: self.decode_mantissa(),
            },
            _ => {
                let mut name = [0; 8];
                let mut body = self.code;
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

    #[inline(always)]
    const fn decode_unsigned(self) -> u32 {
        self.code & !HEAD
    }

    #[inline(always)]
    const fn decode_exponent(self) -> u32 {
        ((self.code & !HEAD) >> 16) & 0xFF
    }

    #[inline(always)]
    const fn decode_mantissa(self) -> u32 {
        self.code & 0xFFFF
    }
}

use core::fmt;

impl fmt::Display for DigitLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LayoutContent::*;
        match self.decode() {
            Unsigned { width } => {
                if self.group == 1 {
                    write!(f, "u{width}")
                } else {
                    write!(f, "[u{width}; {}]", self.group)
                }
            }
            Real { exponent, mantissa } => {
                let width = 1 + exponent + mantissa;
                if self.group == 1 {
                    write!(f, "f{width}_e{exponent}m{mantissa}")
                } else {
                    write!(f, "[f{width}_e{exponent}m{mantissa}; {}]", self.group)
                }
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
            mantissa: 7,
        }
    ));

    assert!(matches!(
        types::I16.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 15,
        }
    ));

    assert!(matches!(
        types::I32.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 31,
        }
    ));

    assert!(matches!(
        types::I64.decode(),
        LayoutContent::Real {
            exponent: 0,
            mantissa: 63,
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

    let q8_0 = DigitLayout::named("Q8_0", 32, 34);
    assert!(matches!(
        q8_0.decode(),
        LayoutContent::Named {
            name: [b'q', b'8', b'0', 0, 0, 0, 0, 0]
        }
    ));

    let iq2xxs = DigitLayout::named("IQ2XXS", 256, 66);
    assert!(matches!(
        iq2xxs.decode(),
        LayoutContent::Named {
            name: [b'i', b'q', b'2', b'x', b'x', b's', 0, 0]
        }
    ));

    let zzzzzz = DigitLayout::named("zzzzzz", 1, 1);
    assert!(matches!(
        zzzzzz.decode(),
        LayoutContent::Named {
            name: [b'z', b'z', b'z', b'z', b'z', b'z', 0, 0]
        }
    ));
}

#[test]
fn test_decode_methods() {
    // 测试decode_unsigned方法
    let u8_layout = DigitLayout::unsigned(8, 1);
    assert_eq!(u8_layout.decode_unsigned(), 8);
    
    let u16_layout = DigitLayout::unsigned(16, 1);
    assert_eq!(u16_layout.decode_unsigned(), 16);
    
    let f32_layout = DigitLayout::real(8, 23, 1);
    assert_eq!(f32_layout.decode_exponent(), 8);
    assert_eq!(f32_layout.decode_mantissa(), 23);
    
    let f64_layout = DigitLayout::real(11, 52, 1);
    assert_eq!(f64_layout.decode_exponent(), 11);
    assert_eq!(f64_layout.decode_mantissa(), 52);
}

#[test]
fn test_group_size_and_nbytes() {
    // 测试group_size和nbytes方法
    let layout1 = DigitLayout::unsigned(32, 4);
    assert_eq!(layout1.group_size(), 4);
    assert_eq!(layout1.nbytes(), 16); // 32/8 * 4 = 16
    
    let layout2 = DigitLayout::real(8, 23, 2);
    assert_eq!(layout2.group_size(), 2);
    assert_eq!(layout2.nbytes(), 8); // (1+8+23)/8 * 2 = 8
    
    let layout3 = DigitLayout::named("test", 3, 12);
    assert_eq!(layout3.group_size(), 3);
    assert_eq!(layout3.nbytes(), 12);
}

#[test]
fn test_to_u64() {
    let layout = DigitLayout::unsigned(32, 1);
    let u64_value = layout.to_u64();
    assert_ne!(u64_value, 0);
    
    let same_layout = DigitLayout::unsigned(32, 1);
    assert_eq!(layout.to_u64(), same_layout.to_u64());
    let different_layout = DigitLayout::unsigned(64, 1);
    assert_ne!(layout.to_u64(), different_layout.to_u64());
}

#[test]
fn test_display_impl() {
    use core::fmt::Write;
    use alloc::string::String;
    
    struct TestWriter(String);
    
    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let u8_layout = DigitLayout::unsigned(8, 1);
    let mut writer = TestWriter(String::new());
    write!(writer, "{}", u8_layout).unwrap();
    assert_eq!(writer.0, "u8");
    
    let u8_array_layout = DigitLayout::unsigned(8, 4);
    let mut writer = TestWriter(String::new());
    write!(writer, "{}", u8_array_layout).unwrap();
    assert_eq!(writer.0, "[u8; 4]");
    
    let f32_layout = DigitLayout::real(8, 23, 1);
    let mut writer = TestWriter(String::new());
    write!(writer, "{}", f32_layout).unwrap();
    assert_eq!(writer.0, "f32_e8m23");
    
    let f32_array_layout = DigitLayout::real(8, 23, 2);
    let mut writer = TestWriter(String::new());
    write!(writer, "{}", f32_array_layout).unwrap();
    assert_eq!(writer.0, "[f32_e8m23; 2]");
    
    let named_layout = DigitLayout::named("test", 1, 4);
    let mut writer = TestWriter(String::new());
    write!(writer, "{}", named_layout).unwrap();
    assert_eq!(writer.0, "test");
}

#[test]
fn test_named_edge_cases() {
    use core::fmt::Write;
    use alloc::string::String;
    
    struct TestWriter(String);
    
    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }
    
    let empty_name = DigitLayout::named("", 1, 1);
    let mut writer = TestWriter(String::new());
    let _ = write!(writer, "{}", empty_name);
    
    let alphanumeric = DigitLayout::named("a1B2c3", 1, 1);
    assert!(matches!(
        alphanumeric.decode(),
        LayoutContent::Named {
            name: [b'a', b'1', b'b', b'2', b'c', b'3', 0, 0]
        }
    ));
    
    let with_special = DigitLayout::named("a_b.c", 1, 1);
    assert!(matches!(
        with_special.decode(),
        LayoutContent::Named {
            name: [b'a', b'b', b'c', 0, 0, 0, 0, 0]
        }
    ));
}
