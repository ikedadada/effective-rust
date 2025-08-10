#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#![allow(unused)]

use std::char;
#[derive(Clone, Debug)]
struct Guest {
    name: String,
    address: String,
    // ... many other fields
}

// Local error type
#[derive(Clone, Debug)]
struct Error(String);

// Register of guests recorded in order of arrival
#[derive(Default, Debug)]
struct GuestRegister(Vec<Guest>);

impl GuestRegister {
    fn register(&mut self, guest: Guest) {
        self.0.push(guest);
    }
    fn nth(&self, idx: usize) -> Option<&Guest> {
        self.0.get(idx)
    }
}
// 到着順序と名前のアルファベット順の双方で効率的に検索する必要が出た

mod cloned {
    use super::Guest;

    #[derive(Default, Debug)]
    struct GuestRegister {
        by_arrival: Vec<Guest>,
        by_name: std::collections::BTreeMap<String, Guest>,
    }

    impl GuestRegister {
        fn register(&mut self, guest: Guest) {
            self.by_arrival.push(guest.clone());
            // Not checking for duplicate names to keep this
            self.by_name.insert(guest.name.clone(), guest);
        }

        fn nth(&self, idx: usize) -> Option<&Guest> {
            self.by_arrival.get(idx)
        }

        fn named(&self, name: &str) -> Option<&Guest> {
            self.by_name.get(name)
        }
    }
}

// ゲストを更新する必要が出た

mod indexed {
    use super::Guest;

    #[derive(Default, Debug)]
    struct GuestRegister {
        by_arrival: Vec<Guest>,
        by_name: std::collections::BTreeMap<String, usize>,
    }

    impl GuestRegister {
        fn register(&mut self, guest: Guest) {
            let idx = self.by_arrival.len();
            // Store the index of the guest in the BTreeMap
            self.by_name.insert(guest.name.clone(), idx);
            self.by_arrival.push(guest);
        }

        fn named(&self, name: &str) -> Option<&Guest> {
            self.by_name
                .get(name)
                .and_then(|&idx| self.by_arrival.get(idx))
        }

        fn named_mut(&mut self, name: &str) -> Option<&mut Guest> {
            let idx = *self.by_name.get(name)?;
            self.nth_mut(idx)
        }

        fn nth(&self, idx: usize) -> Option<&Guest> {
            self.by_arrival.get(idx)
        }

        fn nth_mut(&mut self, idx: usize) -> Option<&mut Guest> {
            self.by_arrival.get_mut(idx)
        }
    }
}

// 登録解除できなければならなくなった

mod rc {
    use super::{Error, Guest};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default, Debug)]
    pub struct GuestRegister {
        by_arrival: Vec<Rc<RefCell<Guest>>>,
        by_name: std::collections::BTreeMap<String, Rc<RefCell<Guest>>>,
    }

    impl GuestRegister {
        pub fn register(&mut self, guest: Guest) {
            let name = guest.name.clone();
            let rc_guest = Rc::new(RefCell::new(guest));
            self.by_arrival.push(rc_guest.clone());
            self.by_name.insert(name, rc_guest.clone());
        }

        pub fn nth(&self, idx: usize) -> Option<Rc<RefCell<Guest>>> {
            self.by_arrival.get(idx).cloned()
        }

        pub fn named(&self, name: &str) -> Option<Rc<RefCell<Guest>>> {
            self.by_name.get(name).cloned()
        }

        pub fn deregister(&mut self, idx: usize) -> Result<(), Error> {
            if idx >= self.by_arrival.len() {
                return Err(Error("Index out of bounds".to_string()));
            }
            let guest = self.by_arrival.remove(idx);
            self.by_name.remove(&guest.borrow().name);
            Ok(())
        }
    }
}

fn main() {
    let mut ledger = rc::GuestRegister::default();
    let alice = Guest {
        name: "Alice".to_string(),
        address: "Wonderland".to_string(),
    };
    let bob = Guest {
        name: "Bob".to_string(),
        address: "Builderland".to_string(),
    };
    let charlie = Guest {
        name: "Charlie".to_string(),
        address: "Chocolate Factory".to_string(),
    };

    ledger.register(alice);
    ledger.register(bob);
    ledger.register(charlie);
    println!("Registered guests: {ledger:?}");

    ledger.deregister(0).unwrap();
    println!("After deregistering first guest: {ledger:?}");

    let also_alice = ledger.named("Alice");
    println!("Looking for Alice after deregistering: {also_alice:?}");

    let also_bob = ledger.named("Bob");
    println!("Looking for Bob: {also_bob:?}");

    let also_charlie = ledger.named("Charlie");
    println!("Looking for Charlie: {also_charlie:?}");
}
