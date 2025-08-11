#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[allow(unused)]
pub mod somemodule {
    // Making a 'struct' public does not make its fields public.
    #[derive(Debug, Default)]
    pub struct AStruct {
        // By default, fields are inaccessible.
        count: i32,
        // Fields have to be explicitly marked 'pub' to be visible.
        pub name: String,
    }

    // Likewise, methods on the struct need individual 'pub' markers.
    impl AStruct {
        // By default, methods are inaccessible.
        fn canonical_name(&self) -> String {
            self.name.to_lowercase()
        }
        // Methods have to be explicitly marked 'pub' to be visible.
        pub fn id(&self) -> String {
            format!("{}-{}", self.canonical_name(), self.count)
        }
    }

    // Making an 'enum' public also makes all of its variants public.
    #[derive(Debug)]
    pub enum AnEnum {
        VariantOne,
        // Fields in variants are also made public.
        VariantTwo(u32),
        VariantThree { name: String, value: String },
    }

    // Making a 'trait' public also makes all of its methods public.
    pub trait DoSomeThing {
        fn do_something(&self, arg: i32);
    }
}

fn main() {
    use somemodule::*;

    let mut s = AStruct::default();
    s.name = "Miles".to_string();
    println!("s = {:?}, name = '{}', id = {}", s, s.name, s.id());

    let e = AnEnum::VariantTwo(42);
    println!("e = {e:?}");

    #[derive(Default)]
    pub struct DoesSomething;
    impl DoSomeThing for DoesSomething {
        fn do_something(&self, arg: i32) {
            println!("Doing something with arg: {}", arg);
        }
    }

    #[allow(clippy::default_constructed_unit_structs)]
    let d = DoesSomething::default();
    d.do_something(42);

    // pubがついていないものには、一般的にアクセスできない
    #[allow(unused)]
    let mut s2 = AStruct::default();
    s2.name = "Miles".to_string();
    // ✖ println!("(inaccessible) s.count = {}", s.count); // ❌ compile error: `count` is private
    // ✖ println!("(inaccessible) s.canonical_name() = {}", s.canonical_name()); // ❌ compile error: `canonical_name` is private
}

