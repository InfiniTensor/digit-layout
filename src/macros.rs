/// 定义一个 [`DigitLayout`](crate::DigitLayout) 实例。
#[macro_export]
macro_rules! layout {
    ($name:ident i($bits:expr)x($packed:expr)) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout =
            $crate::DigitLayout::new($packed, true, 0, $bits - 1);
    };
    ($name:ident u($bits:expr)x($packed:expr)) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout =
            $crate::DigitLayout::new($packed, false, 0, $bits);
    };
    ($name:ident e($exp:expr)m($mant:expr)x($packed:expr)) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout =
            $crate::DigitLayout::new($packed, true, $exp, $mant);
    };

    ($name:ident i($bits:expr)) => {
        layout!($name i($bits)x(1));
    };
    ($name:ident u($bits:expr)) => {
        layout!($name u($bits)x(1));
    };
    ($name:ident e($exp:expr)m($mant:expr)) => {
        layout!($name e($exp)m($mant)x(1));
    };
}

/// 为类型实现与 [`DigitLayout`](crate::DigitLayout) 相关的 trait。
#[macro_export]
macro_rules! impl_digit {
    ($ty:ty : $digit:expr) => {
        impl $crate::AsDigit for $ty {
            const LAYOUT: $crate::DigitLayout = $digit;
        }
    };

    ($ty:ty : $digit:expr, $const:ident) => {
        impl_digit!($ty : $digit);

        const $const: u32 = $digit.to_u32();
        impl $crate::TypeOf_<$const> for $crate::TypeOf<$const> {
            type Type = $ty;
        }
    };
}
