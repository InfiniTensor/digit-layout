use digit_layout::DigitLayout;
use std::time::Instant;
use std::hint::black_box;

fn main() {
    println!("Starting performance test...\n");

    println!("Testing layout creation:");
    
    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::unsigned(8, 1));
        black_box(DigitLayout::unsigned(16, 1));
        black_box(DigitLayout::unsigned(32, 1));
        black_box(DigitLayout::unsigned(64, 1));
    }
    let duration = start.elapsed();
    println!("Creating unsigned integer layouts: {:?}", duration / 20);

    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::real(5, 10, 1));
        black_box(DigitLayout::real(8, 23, 1));
        black_box(DigitLayout::real(11, 52, 1));
    }
    let duration = start.elapsed();
    println!("Creating floating point layouts: {:?}", duration / 15);

    let start = Instant::now();
    for _ in 0..5 {
        black_box(DigitLayout::named("custom", 1, 4));
    }
    let duration = start.elapsed();
    println!("Creating custom layouts: {:?}", duration / 5);

    println!("\nTesting layout decoding:");
    
    let u8_layout = DigitLayout::unsigned(8, 1);
    let f32_layout = DigitLayout::real(8, 23, 1);
    let custom_layout = DigitLayout::named("custom", 1, 4);

    let start = Instant::now();
    for _ in 0..5 {
        black_box(u8_layout.decode());
    }
    let duration = start.elapsed();
    println!("Decoding unsigned integer layouts: {:?}", duration / 5);

    let start = Instant::now();
    for _ in 0..5 {
        black_box(f32_layout.decode());
    }
    let duration = start.elapsed();
    println!("Decoding floating point layouts: {:?}", duration / 5);

    let start = Instant::now();
    for _ in 0..5 {
        black_box(custom_layout.decode());
    }
    let duration = start.elapsed();
    println!("Decoding custom layouts: {:?}", duration / 5);
} 