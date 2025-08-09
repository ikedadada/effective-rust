#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn sum (x: i32, y: i32) -> i32 {
    x + y
}



fn main() {
    // 明示的にfn型を指定する
    let op:fn(i32,i32) -> i32 = sum;
    // fn型はCopyを実装する
    let op1 = op;
    let op2 = op;
    // fn型はEqを実装する 
    assert!( std::ptr::fn_addr_eq(op1,op2) );
    // fn型はstd::fmt::Pointerを実装する
    println!("op = {:p}", op);
    // fn型はstd::fmt::Debugを実装する
    // ただし、fn型は関数ポインタなので、デバッグ出力はアドレスになる
    println!("op = {:?}", op);
    // fn型はstd::fmt::Displayは実装しない
    // println!("op = {}", op);
}
