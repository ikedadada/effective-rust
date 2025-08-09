#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            MyError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// sourceメソッドを実装することで、エラーの原因を握りつぶす危険を回避できる
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}

// Fromトレイトを実装することでライブラリ利用者がMyErrorを簡単に生成できるようにする
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}
impl From<std::string::FromUtf8Error> for MyError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(err)
    }
}

impl From<String> for MyError {
    fn from(err: String) -> Self {
        MyError::General(err)
    }
}

use std::{io::BufRead, vec};

const MAX_LINES: usize = 100;

fn first_line(filename: &str) -> Result<String, MyError> {
    let file = std::fs::File::open(filename).map_err(MyError::from)?;
    let mut reader = std::io::BufReader::new(file);

    let mut buf = vec![];
    let len = reader.read_until(b'\n', &mut buf).map_err(MyError::from)?;
    let result = String::from_utf8(buf).map_err(MyError::from)?;

    if result.len() > MAX_LINES {
        return Err(MyError::General(format!("File too long: {}", len)));
    }

    Ok(result)
}

fn main() {
    match first_line("users.txt") {
        Ok(line) => println!("First non-empty line: {}", line),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example of using the error with a custom message
    let result: Result<(), MyError> = Err(MyError::General("An error occurred".to_string()));
    if let Err(e) = result {
        eprintln!("Custom error: {}", e);
    }

    /*
    完全なエラー型を定義するにはかなりの量のコードを書く必要がある。
    David Tolnayのthiserrorクレート（ライブラリ向け）、anyhowクレート（アプリケーション向け）を使うと、
    エラー型の定義が簡単になる。
     */
}
