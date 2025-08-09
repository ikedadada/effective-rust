#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn main() {
    #[allow(unused)]
    struct Rgb(u8, u8, u8);
    #[allow(unused)]
    enum Color {
        Monochrome,
        Rgb(u8, u8, u8),
    }
    #[allow(unused)]
    struct DisplayProps {
        pub x: u32,
        pub y: u32,
        pub color: Color,
    }
}
