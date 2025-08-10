#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use std::sync::Mutex;

#[derive(Debug)]
struct ThreadSafeInt {
    value: Mutex<i32>,
}

impl ThreadSafeInt {
    fn new(val: i32) -> Self {
        ThreadSafeInt {
            value: Mutex::new(val),
        }
    }

    fn add(&self, delta: i32) {
        // .. more code here that doesn't need the lock
        {
            // ブロックを使うことでRAIIアイテムのスコープを制限する
            // インデントはおかしくなるが、安全性と生存期間の制度が向上する
            let mut val = self.value.lock().unwrap();
            *val += delta;
        }
        // .. more code here that doesn't need the lock
    }
}

impl Drop for ThreadSafeInt {
    fn drop(&mut self) {
        // Mutexのロックを解放するために、Mutexの値をドロップする
        // Mutexの値はスコープを抜けると自動的にドロップされる
        println!("ThreadSafeInt is being dropped, value: {self:?}");
    }
}

fn main() {
    let ts_int = ThreadSafeInt::new(10);
    ts_int.add(5);
    println!("Current value: {:?}", ts_int.value.lock().unwrap());
    // ✖ ts_int.drop(); // 実装したdropメソッドは明示的に実行することはできない
    // ThreadSafeIntはスコープを抜けると自動的にドロップされる

    // drop(ts_int); // drop()を利用することで明示的にドロップすることはできるが、通常は不要
}
