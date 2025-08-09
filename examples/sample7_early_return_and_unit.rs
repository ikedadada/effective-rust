#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

// Return 'x' divided by 'y'
fn div(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        // Terminate the function and return a value
        return f64::NAN; // Not a Number
    }
    // The last expression in the function body is implicitly returned
    x / y
}

// Function called just for its side effects, with no return value
// Can also write the return value as '-> ()'
fn show(x: f64) {
    println!("x = {x}");
}


fn main() {
    let x = 10.0;
    let y = 0.0;

    // Call the function and store the result
    let result = div(x, y);
    show(result); // Show the result

    // Call the function just for its side effects
    show(42.0);
}
