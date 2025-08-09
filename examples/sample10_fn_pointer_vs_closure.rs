#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn main() {
    // In real code, an 'Iterator' method would be more appropriate.
    fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
        for item in data {
            *item = mutator(*item);
        }
    }

    fn add2(v: u32) -> u32 {
        v + 2
    }
    let mut data = vec![1, 2, 3];
    modify_all(&mut data, add2);
    assert_eq!(data, vec![3, 4, 5]);

    // Cannot Compile
    #[allow(unused)]
    let amount = 2;
    // fn add_n(v: u32) -> u32 { v + amount } // ❌ compile error: `amount` is not in scope
    // modify_all(&mut data, add_n); // ❌ compile error: `add_n` cannot capture `amount`
    // To fix this, we can use a closure:

    // Using a closure to capture `amount_to_add`
    let amount_to_add = 2;
    #[allow(unused)]
    let add_n = |v: u32| v + amount_to_add; // ✅
    // modify_all(&mut data, add_n); // ❌ fn add_n is now a closure that captures `amount_to_add`, so it cannot compile as a function pointer.

    fn modify_all_closure<F>(data: &mut [u32], mutator: F)
    where
        F: Fn(u32) -> u32,
        // FnOnce: can be used if the closure consumes `mutator` (i.e., it can only be called once).
        // FnMut: can be used if the closure can be called multiple times and may mutate its environment.
        // Fn: can be used if the closure does not mutate its environment and can be called
    {
        for item in data {
            *item = mutator(*item);
        }
    }

    modify_all_closure(&mut data, add_n); // ✅ Now it works with a closure.
    assert_eq!(data, vec![5, 6, 7]); // The data has been modified correctly.
}
