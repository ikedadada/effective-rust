#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Clone, Debug)]
#[allow(dead_code)]
// Definition of an item of some kind.
struct Item {
    contents: u32,
}

fn consuming_fn(item: Item) {
    // This function consumes the item, so it can no longer be used after this point.
    println!("consuming_fn called with item: {item:?}");
}

fn main1() {
    let item1 = Item { contents: 1 }; // 'item1' created here
    let item2 = Item { contents: 2 }; // 'item2' created here
    println!("item1 = {item1:?}, item2 = {item2:?}",);
    consuming_fn(item2); // 'item2' is moved here
} // 'item1' dropped here

fn main2() {
    #[allow(unused_variables)]
    let r: &Item;
    #[allow(unused_assignments)]
    {
        let item = Item { contents: 3 };
        r = &item; // 'item' is borrowed here
    }
    // ✖ println!("r = {r:?}"); // This would cause a compile-time error because 'item' is dropped here
}

fn main() {
    main1();
    main2();
}
