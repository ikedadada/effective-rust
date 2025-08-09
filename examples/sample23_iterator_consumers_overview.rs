#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2];

    // sum, product
    let s: i32 = nums.iter().copied().sum();
    let p: i32 = [2, 3, 4].iter().copied().product();
    println!("sum = {s}, product = {p}");

    // min, max
    let mn = nums.iter().min();
    let mx = nums.iter().max();
    println!("min = {:?}, max = {:?}", mn, mx);

    // min_by, max_by (compare absolute difference to 5)
    let min_by = nums
        .iter()
        .min_by(|a, b| (a.abs_diff(5)).cmp(&b.abs_diff(5)));
    let max_by = nums
        .iter()
        .max_by(|a, b| (a.abs_diff(5)).cmp(&b.abs_diff(5)));
    println!(
        "min_by(|a,b| abs_diff to 5) = {:?}, max_by = {:?}",
        min_by, max_by
    );

    // reduce (requires Copy)
    let sum_reduce = nums.iter().copied().reduce(|acc, x| acc + x);
    println!("reduce sum = {:?}", sum_reduce);

    // fold (with initial value and different type)
    let concat = ["a", "b", "c"].iter().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    });
    println!("fold concat = {}", concat);

    // scan (running total)
    let running: Vec<i32> = nums
        .iter()
        .copied()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("scan running sums = {:?}", running);

    // find, position, nth
    let found = nums.iter().find(|&&x| x > 4);
    let pos = nums.iter().position(|&x| x == 4);
    // For slices, prefer indexing; keep nth on a mapped iterator to demonstrate consumption
    let nth2 = (0..).zip(nums.iter()).map(|(i, &v)| (i, v)).nth(2);
    println!(
        "find(>4) = {:?}, position(==4) = {:?}, nth(2) = {:?}",
        found, pos, nth2
    );

    // any, all
    let any_gt_8 = nums.iter().any(|&x| x > 8);
    let all_nonneg = nums.iter().all(|&x| x >= 0);
    println!("any > 8 = {any_gt_8}, all >= 0 = {all_nonneg}");
}

