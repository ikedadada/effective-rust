#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

trait Sort {
    // Rearrenge contents into sorted order.
    fn sort(&mut self);
}

impl Sort for Vec<i32> {
    fn sort(&mut self) {
        self.sort_unstable(); // Using unstable sort for demonstration
    }
}

impl Sort for Vec<&str> {
    fn sort(&mut self) {
        self.sort_unstable(); // Using unstable sort for demonstration
    }
}

// Marker trait to indicate that a ['Sort'] sorts stably.
#[allow(unused)]
trait StableSort: Sort {}

fn dump_sorted<T>(mut collection: T)
where
    T: Sort + IntoIterator,
    T::Item: std::fmt::Debug,
{
    collection.sort();
    for item in collection {
        println!("{:?}", item);
    }
}

fn main() {
    // Example usage with a vector of integers
    let numbers = vec![5, 3, 8, 1, 2];
    // Implementing Sort for Vec<i32>
    dump_sorted(numbers);

    // Example usage with a vector of strings
    let words = vec!["banana", "apple", "cherry"];
    dump_sorted(words);
}
