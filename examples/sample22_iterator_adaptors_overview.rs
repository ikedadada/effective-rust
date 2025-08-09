#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Clone, Debug, Eq, PartialEq)]
struct Thing(u64);
impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thing({})", self.0)
    }
}

fn main() {
    // Base collections (arrays are fine for iteration demos)
    let things = [Thing(1), Thing(2), Thing(3), Thing(4), Thing(5), Thing(6)];
    let nums = [10, 20, 30, 40, 50, 60];

    // 1) take, skip, step_by, rev
    let slice1: Vec<u64> = things
        .iter()
        .map(|t| t.0)
        .skip(1) // [2,3,4,5,6]
        .take(4) // [2,3,4,5]
        .step_by(2) // [2,4]
        .rev() // [4,2]
        .collect();
    println!("slice1 = {:?}", slice1);

    // 2) map, filter, take_while, skip_while
    let filtered: Vec<u64> = things
        .iter()
        .map(|t| t.0 * 2) // [2,4,6,8,10,12]
        .skip_while(|&x| x < 5) // [6,8,10,12]
        .take_while(|&x| x <= 10) // [6,8,10]
        .filter(|&x| x % 4 != 0) // [6,10]
        .collect();
    println!("filtered = {:?}", filtered);

    // 3) cloned, copied, enumerate
    // cloned: for iterators of &T where T: Clone — clone only what you need
    let cloned_vec: Vec<Thing> = things.iter().take(3).cloned().collect();
    println!("cloned_vec = {:?}", cloned_vec);

    // copied: for iterators of &CopyType
    let copied_vec: Vec<i32> = [1, 2, 3, 4, 5].iter().copied().skip(2).collect();
    println!("copied_vec = {:?}", copied_vec);

    // enumerate pairs each item with its index
    for (idx, val) in cloned_vec.iter().enumerate() {
        println!("enumerate {} -> {}", idx, val);
    }

    // 4) chain, zip
    // Use iter() on arrays and copy out values to avoid explicit into_iter calls
    let chained: Vec<i32> = [1, 2, 3].iter().chain([4, 5].iter()).copied().collect();
    println!("chained = {:?}", chained);

    // zip two streams; length is min of both; pass arrays directly (IntoIterator is auto)
    let zipped: Vec<(u64, i32)> = things.iter().map(|t| t.0).zip(nums).collect();
    println!("zipped = {:?}", zipped);

    // 5) cycle (be careful to take to avoid infinite loop)
    let cycled: Vec<i32> = [7, 8].iter().copied().cycle().take(5).collect();
    println!("cycled = {:?}", cycled);
}

