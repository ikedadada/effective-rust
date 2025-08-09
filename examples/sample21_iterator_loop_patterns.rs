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
    // 以下の3つのコードは同値（2,3はiter変換を行っているのでcollectionが消費されず、以降も利用可能）
    let collection = vec![Thing(1), Thing(2), Thing(3)];

    for item in collection {
        println!("Item: {}", item);
    }

    #[allow(clippy::useless_vec)]
    let collection = vec![Thing(4), Thing(5), Thing(6)];
    let mut iter = collection.iter();
    #[allow(clippy::while_let_loop)]
    loop {
        let item = match iter.next() {
            Some(value) => value,
            None => break,
        };
        println!("Item: {}", item);
    }

    #[allow(clippy::useless_vec)]
    let collection = vec![Thing(7), Thing(8), Thing(9)];
    let mut iter = collection.iter();
    #[allow(clippy::while_let_on_iterator)]
    while let Some(item) = iter.next() {
        println!("Item: {}", item);
    }
}

