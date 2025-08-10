#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn caller() -> u64 {
    let x = 42u64;
    let y = 19u64;
    f(x) + g(y)
}

fn f(f_param: u64) -> u64 {
    let two = 2u64;
    f_param + two
}

fn g(g_param: u64) -> u64 {
    let arr = [2u64, 3u64];
    g_param + arr[1]
}

fn main() {
    let result = caller();
    /*
      1. caller Stack
        | 42 | 19 |
      2. caller -> f Stack
        | 42 | 19 | | 42 (f_param) | 2 (two)|
      3. caller Stack
        | 42 | 19 |
      4. caller -> g Stack
        | 42 | 19 | | 19 (g_param) | 2 (arr[0]) | 3 (arr[1]) |
    */
    println!("Result: {}", result);
    assert_eq!(result, 66); // 42 + 2 + 19 + 3
}
