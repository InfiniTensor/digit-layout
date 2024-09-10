/// 定义一个 [`DigitLayout`](crate::DigitLayout) 实例。
#[macro_export]
macro_rules! layout {
    ($name:ident u($bits:expr)) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::unsigned($bits);
    };
    ($name:ident i($bits:expr)) => {
        layout!($name e(0)m($bits));
    };
    ($name:ident e($exponent:expr)m($mantissa:expr)) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::real($exponent, $mantissa);
    };
    ($name:ident = $text:expr) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::named($text);
    };
    ($name:ident) => {
        layout!($name = stringify!($name));
    };
}
