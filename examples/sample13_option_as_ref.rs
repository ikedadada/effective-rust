#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn encrypt(data: &[u8]) -> bool {
    // Placeholder for encryption logic
    !data.is_empty()
}

struct InputData {
    payload: Option<Vec<u8>>,
}

impl InputData {
    #[allow(dead_code)]
    fn encrypted(&self) -> bool {
        // ✖ Cannot move payload which is behind a shared reference
        // encrypt(self.payload.unwrap_or(vec![]))
        // ✔ Use a as_ref() to get a reference to the payload
        // as_ref() は Optionへの参照を、参照のOptionに変換します。
        encrypt(self.payload.as_ref().unwrap_or(&vec![]))
    }
}

fn main() {}
