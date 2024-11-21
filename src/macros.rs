/// 定义一个 [`DigitLayout`](crate::DigitLayout) 实例。
#[macro_export]
macro_rules! layout {
    ($name:ident u($bits:expr); $group:expr) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::unsigned($bits, $group);
    };
    ($name:ident e($exponent:expr)m($mantissa:expr); $group:expr) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::real($exponent, $mantissa, $group);
    };
    ($name:ident = $text:expr; [$group:expr] in $size:expr) => {
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::named($text, $group, $size);
    };

    ($name:ident u($bits:expr)) => {
        $crate::layout!($name u($bits); 1);
    };
    ($name:ident i($bits:expr)) => {
        $crate::layout!($name e(0)m($bits - 1); 1);
    };
    ($name:ident e($exponent:expr)m($mantissa:expr)) => {
        $crate::layout!($name e($exponent)m($mantissa); 1);
    };
    ($name:ident; [$group:expr] in $size:expr) => {
        $crate::layout!($name = stringify!($name); [$group] in $size);
    };
}
