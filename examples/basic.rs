use digit_layout::DigitLayout;

fn main() {
    // 基本类型示例
    let u8_layout = DigitLayout::unsigned(8, 1);
    let u16_layout = DigitLayout::unsigned(16, 1);
    let u32_layout = DigitLayout::unsigned(32, 1);
    let u64_layout = DigitLayout::unsigned(64, 1);

    println!("无符号整数类型:");
    println!("u8: {}", u8_layout);
    println!("u16: {}", u16_layout);
    println!("u32: {}", u32_layout);
    println!("u64: {}", u64_layout);

    // 浮点数类型示例
    let f16_layout = DigitLayout::real(5, 10, 1);
    let f32_layout = DigitLayout::real(8, 23, 1);
    let f64_layout = DigitLayout::real(11, 52, 1);

    println!("\n浮点数类型:");
    println!("f16: {}", f16_layout);
    println!("f32: {}", f32_layout);
    println!("f64: {}", f64_layout);

    // 数组类型示例
    let u8_array = DigitLayout::unsigned(8, 4);
    let f32_array = DigitLayout::real(8, 23, 4);

    println!("\n数组类型:");
    println!("[u8; 4]: {}", u8_array);
    println!("[f32; 4]: {}", f32_array);

    // 自定义类型示例
    let custom_layout = DigitLayout::named("custom", 1, 4);
    println!("\n自定义类型:");
    println!("custom: {}", custom_layout);
}