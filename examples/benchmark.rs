use digit_layout::DigitLayout;
use std::time::Instant;
use std::hint::black_box;

fn main() {
    println!("开始性能测试...\n");

    // 测试创建布局
    println!("测试创建布局:");
    
    // 创建无符号整数布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::unsigned(8, 1));
        black_box(DigitLayout::unsigned(16, 1));
        black_box(DigitLayout::unsigned(32, 1));
        black_box(DigitLayout::unsigned(64, 1));
    }
    let duration = start.elapsed();
    println!("创建无符号整数布局: {:?}", duration / 20);

    // 创建浮点数布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::real(5, 10, 1));
        black_box(DigitLayout::real(8, 23, 1));
        black_box(DigitLayout::real(11, 52, 1));
    }
    let duration = start.elapsed();
    println!("创建浮点数布局: {:?}", duration / 15);

    // 创建自定义布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::named("custom", 1, 4));
    }
    let duration = start.elapsed();
    println!("创建自定义布局: {:?}", duration / 5);

    println!("\n测试解码布局:");
    
    let u8_layout = DigitLayout::unsigned(8, 1);
    let f32_layout = DigitLayout::real(8, 23, 1);
    let custom_layout = DigitLayout::named("custom", 1, 4);

    // 解码无符号整数布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(u8_layout.decode());
    }
    let duration = start.elapsed();
    println!("解码无符号整数布局: {:?}", duration / 5);

    // 解码浮点数布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(f32_layout.decode());
    }
    let duration = start.elapsed();
    println!("解码浮点数布局: {:?}", duration / 5);

    // 解码自定义布局
    let start = Instant::now();
    for _ in 0..5 {
        black_box(custom_layout.decode());
    }
    let duration = start.elapsed();
    println!("解码自定义布局: {:?}", duration / 5);
} 