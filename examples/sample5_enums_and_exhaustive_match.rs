#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

fn main() {
    #[allow(unused)]
    enum HttpResultCode {
        Ok = 200,
        NotFound = 404,
        Teapot = 418,
    }

    let code = HttpResultCode::NotFound;
    assert_eq!(code as i32, 404);

    #[allow(unused)]
    enum Slides {
        Both,
        Single,
    }
    #[allow(unused)]
    enum Output {
        BlackAndWhite,
        Color,
    }

    fn print_slides(slides: Slides, output: Output) {
        match (slides, output) {
            (Slides::Both, Output::BlackAndWhite) => println!("Both slides in black and white"),
            (Slides::Both, Output::Color) => println!("Both slides in color"),
            (Slides::Single, Output::BlackAndWhite) => println!("Single slide in black and white"),
            (Slides::Single, Output::Color) => println!("Single slide in color"),
            /*
            どれかの条件をコメントアウトすると静的解析エラーになる。
            missing match arm: `(Single, Color)` not covered
            */
        }
    }

    print_slides(Slides::Both, Output::Color);
}
