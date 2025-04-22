/// Defines an instance of [`DigitLayout`](crate::DigitLayout).
#[macro_export]
macro_rules! layout {
    ($name:ident u($bits:expr); $group:expr) => {
        // 允许使用小写常量名，因为这是用户定义的布局名称，应该保持原始大小写
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::unsigned($bits, $group);
    };
    ($name:ident e($exponent:expr)m($mantissa:expr); $group:expr) => {
        // 允许使用小写常量名，因为这是用户定义的布局名称，应该保持原始大小写
        #[allow(non_upper_case_globals)]
        pub const $name: $crate::DigitLayout = $crate::DigitLayout::real($exponent, $mantissa, $group);
    };
    ($name:ident = $text:expr; [$group:expr] in $size:expr) => {
        // 允许使用小写常量名，因为这是用户定义的布局名称，应该保持原始大小写
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
