#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

struct Item {
    content: i32,
}

fn check_item(item: Option<&Item>) {
    match item {
        Some(i) => println!("Item content: {}", i.content),
        None => println!("No item found"),
    }
}

fn main() {
    // ✖ Cannot compile
    // let x = Some(Rc::new(RefCell::new(Item { content: 42 })));
    // check_item(x.as_ref().map(|r| r.borrow().deref()));
    /*
    cannot return value referencing temporary value
    returns a value referencing data owned by the current functionrustcClick for full compiler diagnostic
    */

    // Explicitly annotate the type
    // ✖ Cannot compile
    // let x: Option<Rc<RefCell<Item>>> = Some(Rc::new(RefCell::new(Item { content: 42 })));
    // let x1: Option<&Rc<RefCell<Item>>> = x.as_ref();
    // let x2: Option<Ref<Item>> = x1.map(|r| r.borrow());
    // let x3: Option<&Item> = x2.map(|r| r.deref());
    /*
    cannot return value referencing function parameter `r`
    returns a value referencing data owned by the current functionrustcClick for full compiler diagnostic
     */
    // check_item(x3);

    // Can compile
    let x: Option<Rc<RefCell<Item>>> = Some(Rc::new(RefCell::new(Item { content: 42 })));
    let x1: Option<&Rc<RefCell<Item>>> = x.as_ref();
    let x2: Option<Ref<Item>> = x1.map(|r| r.borrow());
    match x2 {
        Some(r) => {
            let item: &Item = r.deref();
            check_item(Some(item));
        }
        None => check_item(None),
    }

    // Can compile with deref coercion
    let x = Some(Rc::new(RefCell::new(Item { content: 42 })));
    match x.as_ref().map(|r| r.borrow()) {
        Some(r) => {
            let item: &Item = r.deref();
            check_item(Some(item));
        }
        None => check_item(None),
    }
}
