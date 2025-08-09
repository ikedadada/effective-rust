#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

#[allow(unused)]
struct TextMatch(usize, String);

fn main() {
    let m = TextMatch(12, "needle".to_owned());
    assert_eq!(m.0, 12);
}
