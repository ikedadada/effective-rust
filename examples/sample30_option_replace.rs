#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Debug)]
#[allow(unused)]
struct Item {
    pub contents: i64,
}

#[allow(unused)]
fn replace(item: &mut Option<Item>, val: Item) -> Option<Item> {
    // ✖ 可変参照をmoveすることはできない
    // let previous = *item;
    // *item = Some(val);
    // previous

    // 標準ライブラリをりようしてこれを実現することができる
    // std::mem::replace(item, Some(val))
    // もしくは
    item.replace(val)
}

fn main() {
    let mut item: Option<Item> = None;
    println!("item = {item:?}");

    // 置き換え
    let previous = replace(&mut item, Item { contents: 42 });
    println!("item = {item:?}, previous = {previous:?}");

    // 再度置き換え
    let previous = replace(&mut item, Item { contents: 43 });
    println!("item = {item:?}, previous = {previous:?}");
}
