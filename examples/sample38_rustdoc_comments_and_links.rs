#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

// doc comments are used to generate documentation
//! or /// to provide inline documentation for code elements
// This example demonstrates how to define a simple struct and a function
// [''] makes it easy to refer to types in documentation.
// # Examples section is used to provide usage examples in documentation
// and sample code is excecuted when cargo test is run.
// # Panics section is used to provide information about potential panics
// # Safety section is used to provide information about unsafe code usage

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

/// A simple struct representing a bounding box in 2D space.
#[allow(unused)]
struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Calculate the ['BoundingBox'] that exactly encompassed a pair
/// of ['BoundingBox'] objects.
#[allow(unused)]
fn union(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    let width = (a.x + a.width).max(b.x + b.width) - x;
    let height = (a.y + a.height).max(b.y + b.height) - y;

    BoundingBox {
        x,
        y,
        width,
        height,
    }
}

fn main() {}

