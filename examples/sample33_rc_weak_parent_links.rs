#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// Use a newtype for each identifier type.
struct TreeId(String);
struct BranchId(String);
#[allow(dead_code)]
struct LeafId(String);

#[allow(dead_code)]
struct Tree {
    id: TreeId,
    branches: Vec<Rc<RefCell<Branch>>>,
}

struct Branch {
    id: BranchId,
    leaves: Vec<Rc<RefCell<Leaf>>>,
    owner: Option<Weak<RefCell<Tree>>>, // 親への参照はWeak参照にすることで循環参照を避ける
}

#[allow(dead_code)]
struct Leaf {
    id: LeafId,
    owner: Option<Weak<RefCell<Branch>>>, // 親への参照はWeak参照にすることで循環参照を避ける
}

impl Branch {
    fn add_leaf(branch: Rc<RefCell<Branch>>, mut leaf: Leaf) {
        leaf.owner = Some(Rc::downgrade(&branch));
        branch.borrow_mut().leaves.push(Rc::new(RefCell::new(leaf)));
    }

    fn location(&self) -> String {
        match &self.owner {
            None => format!("<unowned>.{}", self.id.0),
            Some(owner) => {
                // Upgrade weak owner point.
                let tree = owner.upgrade().expect("owner gone!"); // Weak参照は、メインの参照カウントをincrementしないので、アイテムがなくなっていないことを明示的にチェックする必要がある
                format!("{}.{}", tree.borrow().id.0, self.id.0)
            }
        }
    }
}

fn main() {
    let tree = Rc::new(RefCell::new(Tree {
        id: TreeId("tree1".to_string()),
        branches: Vec::new(),
    }));

    let branch = Rc::new(RefCell::new(Branch {
        id: BranchId("branch1".to_string()),
        leaves: Vec::new(),
        owner: Some(Rc::downgrade(&tree)),
    }));

    // Add a leaf to the branch.
    Branch::add_leaf(
        branch.clone(),
        Leaf {
            id: LeafId("leaf1".to_string()),
            owner: None,
        },
    );

    // Print the location of the branch.
    println!("Branch location: {}", branch.borrow().location());
}
