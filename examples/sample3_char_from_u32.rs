#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

fn main() {
    let raw: u32 = 1_000_000_000;
    let safe = char::from_u32(raw);
    match safe {
        Some(c) => println!("Valid char: {}", c),
        None => println!("Invalid Unicode scalar value: {}", raw),
    }
    let usafe = unsafe { char::from_u32_unchecked(raw) }; // ⚠️ Don't do this with invalid values.
    println!("Unsafe char: {}", usafe); // This line is just to show the unsafe
}
