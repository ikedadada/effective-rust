#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

struct Point {
    x: f64,
    y: f64,
}

fn show(pt: &Point) {
    println!("Point at ({}, {})", pt.x, pt.y);
}

fn main() {
    let pt = Point { x: 3.0, y: 4.0 };
    show(&pt);

    // 明示的に型を指定する
    let pt_ref: &Point = &pt;
    // もちろんPointの参照は実行できる
    show(pt_ref);
    // &Box<Point>のような参照も可能
    //   Box<T>がDerefトレイトをTarget=Tとして実装しているため
    //   &Box<T>はTの参照として扱える
    show(&Box::new(pt));
}

