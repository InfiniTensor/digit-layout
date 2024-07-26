//! 为一些基本类型和常用类型提供预定义布局。

#![allow(missing_docs)]

layout!(BOOL   u( 1)          );
layout!(I8     i( 8)          );
layout!(I16    i(16)          );
layout!(I32    i(32)          );
layout!(I64    i(64)          );
layout!(U8     u( 8)          );
layout!(U16    u(16)          );
layout!(U32    u(32)          );
layout!(U64    u(64)          );
layout!(F16    e(10)m( 5)     );
layout!(BF16   e( 7)m( 8)     );
layout!(F32    e(23)m( 8)     );
layout!(F64    e(52)m(11)     );

layout!(F16x2  e(10)m( 5)x(2) );
layout!(BF16x2 e( 7)m( 8)x(2) );

impl_digit!(bool: BOOL, CBOOL);
impl_digit!(i8  : I8  , CI8  );
impl_digit!(i16 : I16 , CI16 );
impl_digit!(i32 : I32 , CI32 );
impl_digit!(i64 : I64 , CI64 );
impl_digit!(u8  : U8  , CU8  );
impl_digit!(u16 : U16 , CU16 );
impl_digit!(u32 : U32 , CU32 );
impl_digit!(u64 : U64 , CU64 );
impl_digit!(f32 : F32 , CF32 );
impl_digit!(f64 : F64 , CF64 );

#[cfg(feature = "half")]
mod half_impl {
    use half::{bf16, f16};

    impl_digit!( f16        : super::F16   , CF16   );
    impl_digit!([f16 ; 2]   : super::F16x2 , CF16_2 );
    impl_digit!((f16 ,  f16): super::F16x2          );

    impl_digit!( bf16       : super::BF16  , CBF16  );
    impl_digit!([bf16; 2]   : super::BF16x2, CBF16_2);
    impl_digit!((bf16, bf16): super::BF16x2         );
}
