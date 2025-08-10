#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

// Calulate the overlap between two rectangles, or 'None' if there is no overlap.
fn overlap(a: Bounds, b: Bounds) -> Option<Bounds> {
    let left = a.top_left.x.max(b.top_left.x);
    let right = a.bottom_right.x.min(b.bottom_right.x);
    let top = a.top_left.y.max(b.top_left.y);
    let bottom = a.bottom_right.y.min(b.bottom_right.y);
    if left < right && top < bottom {
        Some(Bounds {
            top_left: Point { x: left, y: top },
            bottom_right: Point {
                x: right,
                y: bottom,
            },
        })
    } else {
        None
    }
}

// Trait for object that can be drawn graphically.
trait Draw {
    // Return the bounding rectangle that encompassed the object.
    fn bounds(&self) -> Bounds;
}

impl Draw for Bounds {
    fn bounds(&self) -> Bounds {
        *self
    }
}

const SCREEN_BOUNDS: Bounds = Bounds {
    top_left: Point { x: 0, y: 0 },
    bottom_right: Point { x: 800, y: 600 },
};

// Indicate whether an object is on-screen.
fn on_screen<T: Draw>(draw: &T) -> bool {
    overlap(draw.bounds(), SCREEN_BOUNDS).is_some()
}

#[derive(Clone)]
struct Square {
    top_left: Point,
    size: i64,
}

impl Draw for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: self.top_left,
            bottom_right: Point {
                x: self.top_left.x + self.size,
                y: self.top_left.y + self.size,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Circle {
    center: Point,
    radius: i64,
}

impl Draw for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
            },
            bottom_right: Point {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
            },
        }
    }
}

fn main() {
    let a = Bounds {
        top_left: Point { x: 100, y: 100 },
        bottom_right: Point { x: 200, y: 200 },
    };
    let b = Bounds {
        top_left: Point { x: 150, y: 150 },
        bottom_right: Point { x: 250, y: 250 },
    };
    let c = Bounds {
        top_left: Point { x: 300, y: 300 },
        bottom_right: Point { x: 400, y: 400 },
    };

    println!("Overlap between a and b: {:?}", overlap(a, b));
    println!("Overlap between a and c: {:?}", overlap(a, c));

    // Example usage of on_screen
    if on_screen(&a) {
        println!("Bounds a is on screen.");
    } else {
        println!("Bounds a is off screen.");
    }

    let square = Square {
        top_left: Point { x: 50, y: 50 },
        size: 100,
    };
    if on_screen(&square) {
        println!("Square is on screen.");
    } else {
        println!("Square is off screen.");
    }

    let circle = Circle {
        center: Point { x: 400, y: 300 },
        radius: 50,
    };
    if on_screen(&circle) {
        println!("Circle is on screen.");
    } else {
        println!("Circle is off screen.");
    }

    /*
    - ジェネリクスを使うとコードサイズが大きくなる傾向がある。コンパイラはジェネリックなon_screenをこの関数を用いるすべての型に対して、新しくコピーをしてコードを生成するため。トレイトオブジェクト（&dyn Draw）を利用する場合は関数のインスタンスは1つだけでよい
    - ジェネリクスのトレイトメソッドwの呼び出しは、トレイトオブジェクトを使った場合よりも、ごくごくわずかにだけ高速だ。これは、トレイトオブジェクトを用いるとvtableを挟む2段階参照解決が行われるため
    - ジェネリクスの方がコンパイル時間が長くなる傾向がある。コンパイラより多くのコードを出力するし、リンカでは重複を解消するために余計に仕事しなければならない。
    */

    // ジェネリックなトレイト制約を用いると型パラメータが「複数の」トレイトを実装している場合にだけ機能を利用可能にできる

    // area関数は'Draw'トレイトを実装しているコンテナすべてで利用できる
    #[allow(dead_code)]
    fn area<T>(draw: &T) -> i64
    where
        T: Draw,
    {
        let bounds = draw.bounds();
        let width = bounds.bottom_right.x - bounds.top_left.x;
        let height = bounds.bottom_right.y - bounds.top_left.y;
        width * height
    }

    // show関数は'Draw'トレイトと’Debug'トレイトを実装していなければならない
    fn show<T>(draw: &T)
    where
        T: Draw + std::fmt::Debug,
    {
        println!("Drawing: {:?}", draw);
        println!("Bounds: {:?}", draw.bounds());
    }

    // CircleはDebugトレイトを実装しているので、show関数を呼び出せる
    show(&circle);
    // SquareはDebugトレイトを実装していないので、show関数を呼び出せない
    // ✖ show(&square); // これはコンパイルエラーになる

    // トレイト制約はトレイト定義そのものにも適用できる
    #[allow(dead_code)]
    trait Shape: Draw {
        // Render that portion of the shape that falls within 'bounds'.
        fn render_in(&self, bounds: Bounds);
        fn render(&self) {
            // Default implementation renders that portion of the shape
            // that falls within the screen area.
            if let Some(visible) = overlap(self.bounds(), SCREEN_BOUNDS) {
                self.render_in(visible);
            }
        }
    }

    // トレイトオブジェクトにはトレイトオブジェクト安全性による制限がある
    // - トレイトメソッドはジェネリクスであってはならない。
    // - トレイトメソッドでは、レシーバ（メソッドが起動される対象のオブジェクト）以外に、Selfを含む型を使ってはならない
}
