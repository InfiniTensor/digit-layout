# digit-layout

[![CI](https://github.com/InfiniTensor/digit-layout/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/InfiniTensor/digit-layout/actions)
[![Latest version](https://img.shields.io/crates/v/digit-layout.svg)](https://crates.io/crates/digit-layout)
[![Documentation](https://docs.rs/digit-layout/badge.svg)](https://docs.rs/digit-layout)
[![license](https://img.shields.io/github/license/InfiniTensor/digit-layout)](https://mit-license.org/)
![GitHub repo size](https://img.shields.io/github/repo-size/InfiniTensor/digit-layout)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/InfiniTensor/digit-layout)

[![GitHub Issues](https://img.shields.io/github/issues/InfiniTensor/digit-layout)](https://github.com/InfiniTensor/digit-layout/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/InfiniTensor/digit-layout)](https://github.com/InfiniTensor/digit-layout/pulls)
![GitHub contributors](https://img.shields.io/github/contributors/InfiniTensor/digit-layout)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/InfiniTensor/digit-layout)

这个库提供了一个统一的数据类型定义，可以高效地编码类型信息，避免重复定义数据类型。

## 特性

- 支持无符号整数类型
- 支持浮点数类型
- 支持自定义命名类型
- 紧凑的内存布局
- 无标准库依赖（no_std）
- 可选的 half 浮点数支持

## 安装

在 Cargo.toml 中添加：

```toml
[dependencies]
digit-layout = "0.3.0"
```

## 使用示例

### 基本用法

```rust
use digit_layout::{DigitLayout, LayoutContent};

// 创建无符号整数类型布局
let u8_layout = DigitLayout::unsigned(8, 1);
assert_eq!(u8_layout.to_string(), "u8");

// 创建浮点数类型布局
let f32_layout = DigitLayout::real(8, 23, 1);
assert_eq!(f32_layout.to_string(), "f32_e8m23");

// 创建自定义类型布局
let custom_layout = DigitLayout::named("custom", 1, 4);
assert_eq!(custom_layout.to_string(), "custom");
```

### 数组类型

```rust
use digit_layout::DigitLayout;

// 创建无符号整数数组布局
let u8_array = DigitLayout::unsigned(8, 4);
assert_eq!(u8_array.to_string(), "[u8; 4]");

// 创建浮点数数组布局
let f32_array = DigitLayout::real(8, 23, 4);
assert_eq!(f32_array.to_string(), "[f32_e8m23; 4]");
```

### 解码布局

```rust
use digit_layout::{DigitLayout, LayoutContent};

// 解码无符号整数布局
let u8_layout = DigitLayout::unsigned(8, 1);
match u8_layout.decode() {
    LayoutContent::Unsigned { width } => {
        assert_eq!(width, 8);
    }
    _ => panic!("Expected unsigned layout"),
}

// 解码浮点数布局
let f32_layout = DigitLayout::real(8, 23, 1);
match f32_layout.decode() {
    LayoutContent::Real { exponent, mantissa } => {
        assert_eq!(exponent, 8);
        assert_eq!(mantissa, 23);
    }
    _ => panic!("Expected real layout"),
}
```

## 性能测试

项目包含一个性能测试示例，可以测量各种操作的执行时间。运行性能测试：

```bash
cargo run --example benchmark
```

性能测试会测量以下操作的执行时间：

1. 创建布局
   - 创建无符号整数布局
   - 创建浮点数布局
   - 创建自定义布局

2. 解码布局
   - 解码无符号整数布局
   - 解码浮点数布局
   - 解码自定义布局

测试结果会显示每个操作的平均执行时间。

## 文档

完整的 API 文档可以在 [docs.rs](https://docs.rs/digit-layout) 上找到。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。
