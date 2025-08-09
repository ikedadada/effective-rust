#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

struct S {
    field: Option<i32>,
}

fn main() {
    let s = S { field: Some(42) };

    // Using `match` to handle the Option type
    // match s.field {
    //     Some(value) => println!("Value: {}", value),
    //     None => {}
    // }

    if let Some(value) = s.field {
        println!("Value: {}", value);
    }

    #[allow(unused_mut)]
    let result = std::fs::File::open("unexpected_file.txt");
    #[allow(unused_variables)]
    let f = match result {
        Ok(ref file) => file,
        Err(e) => {
            panic!("Failed to open file: {}", e);
        }
    };

    // unwrap occurs panic if the file does not exist
    // #[allow(unused_variables)]
    // let f = result.unwrap();

    #[allow(dead_code)]
    fn find_user(username: &str) -> Result<String, std::io::Error> {
        // Not Recommended: using `match` for error handling
        // let f = match std::fs::File::open("users.txt") {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // Recommended: Using `?` operator for error propagation
        // ? operator will return an error if the file does not exist
        #[allow(unused_variables)]
        let f = std::fs::File::open("users.txt")?;

        Ok(format!("User: {}", username))
    }

    #[allow(dead_code)]
    fn find_user2(username: &str) -> Result<String, String> {
        // Unrecommended: Using `match` for error handling
        // #[allow(unused_variables)]
        // let f = match std::fs::File::open("users.txt") {
        //     Ok(file) => file,
        //     Err(e) => return Err(format!("Failed to open file: {}", e)),
        // };

        // Recommended: Using map_err to convert the error type
        #[allow(unused_variables)]
        let f =
            std::fs::File::open("users.txt").map_err(|e| format!("Failed to open file: {}", e))?;

        Ok(format!("User: {}", username))
    }
}
