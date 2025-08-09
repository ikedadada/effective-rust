#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn main1() {
    // Integer value from an IANA-controlled range.
    #[derive(Clone, Copy, Debug)]
    struct IanaAllocated(pub u64);

    impl From<u64> for IanaAllocated {
        fn from(value: u64) -> Self {
            IanaAllocated(value)
        }
    }

    fn is_iana_reserved(s: IanaAllocated) -> bool {
        // Check if the value is within the IANA reserved range.
        s.0 == 0 || s.0 == 65535
    }

    let s = IanaAllocated(42);
    println!("{:?} reserved: {}", s, is_iana_reserved(s));

    // ✖ is_iana_reserved(42) cannot be called because the type of 42 is not IanaAllocated.
    // if is_iana_reserved(42) {
    //     println!("42 is reserved");
    // }
}

fn main2() {
    // Integer value from an IANA-controlled range.
    #[derive(Clone, Copy, Debug)]
    struct IanaAllocated(pub u64);

    impl From<u64> for IanaAllocated {
        fn from(value: u64) -> Self {
            IanaAllocated(value)
        }
    }

    fn is_iana_reserved<T>(s: T) -> bool
    where
        T: Into<IanaAllocated>,
    {
        // Check if the value is within the IANA reserved range.
        let s = s.into();
        s.0 == 0 || s.0 == 65535
    }

    let s = IanaAllocated(42);
    println!("{:?} reserved: {}", s, is_iana_reserved(s));

    // Now we can call is_iana_reserved with an integer.
    if is_iana_reserved(42) {
        println!("42 is reserved");
    }
}

fn main() {
    main1();
    main2();
}
