#![doc = include_str!("../README.md")]
#![no_std]
#![deny(warnings, missing_docs)]

#[macro_use]
mod macros;
pub mod types;

use core::{
    alloc::Layout,
    mem::{align_of, transmute},
};

/// A layout of a digit data type in memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct DigitLayout {
    signed_nbyte: u8,
    packed: u8,
    exponent: u8,
    mantissa: u8,
}

/// A trait for types that can be represented as a digit data type.
pub trait AsDigit {
    /// The layout of the digit data type.
    const LAYOUT: DigitLayout;
}

/// Allow a const [`DigitLayout`] value associated with a type.
pub trait TypeOf_<const N: u32> {
    /// The type associated with the layout.
    type Type;
}

/// A type template for the [`TypeOf_`] trait to implement.
pub struct TypeOf<const N: u32>;

const _8: usize = u8::BITS as usize;
const _7: usize = _8 - 1;
const MAX_ALIGN: usize = align_of::<usize>();

impl DigitLayout {
    /// Creates a new [`DigitLayout`] value.
    #[inline]
    pub const fn new(packed: usize, signed: bool, exponent: usize, mantissa: usize) -> Self {
        assert!(packed <= u8::MAX as usize);
        assert!(exponent <= u8::MAX as usize);
        assert!(mantissa <= u8::MAX as usize);
        let signed = if signed { 1 } else { 0 };

        let total_bits = packed * (signed + exponent + mantissa);
        let nbyte = ((total_bits + _7) / _8).next_power_of_two();
        assert!(nbyte < (1 << _7));

        Self {
            packed: packed as _,
            signed_nbyte: ((signed << _7) | nbyte) as _,
            exponent: exponent as _,
            mantissa: mantissa as _,
        }
    }

    /// Converts the layout to a `u32` code.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        unsafe { transmute(self) }
    }

    /// Gets the packed count of the digit data type.
    #[inline]
    pub const fn packed(self) -> usize {
        self.packed as _
    }

    /// Gets the signedness of the digit data type.
    #[inline]
    pub const fn signed(self) -> bool {
        self.signed_nbyte >> _7 == 1
    }

    /// Gets the exponent bits of the digit data type.
    #[inline]
    pub const fn exponent(self) -> usize {
        self.exponent as _
    }

    /// Gets the mantissa bits of the digit data type.
    #[inline]
    pub const fn mantissa(self) -> usize {
        self.mantissa as _
    }

    /// Gets the padding bits of the digit data type.
    #[inline]
    pub const fn padding(self) -> usize {
        self.nbits() - self.packed() * (self.signed() as usize + self.exponent() + self.mantissa())
    }

    /// Gets the total bits of the digit data type.
    #[inline]
    pub const fn nbits(self) -> usize {
        self.nbytes() * _8
    }

    /// Gets the number of bytes of the digit data type.
    #[inline]
    pub const fn nbytes(self) -> usize {
        (self.signed_nbyte & ((1 << _7) - 1)) as _
    }

    /// Gets the layout of the digit data type.
    #[inline]
    pub const fn layout(self) -> Layout {
        let size = self.nbytes();
        let align = if size < MAX_ALIGN { size } else { MAX_ALIGN };
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }
}
