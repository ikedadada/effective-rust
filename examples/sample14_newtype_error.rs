#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```
//!

// ✖ orphan ruleにより自分で定義したものではないため、StringにはErrorトレイトを実装できない
// impl std::error::Error for String {}

// ✖ 型エイリアスも新しい型を作り出すわけではないのでErrorトレイトを実装できない
// type MyError = String;
// impl std::error::Error for MyError {}

// 〇 String型をラップするタプル構造体を作れば、Errorトレイとを実装できる
// ただし、Errorトレイトを実装するにはDebugトレイトと Displayトレイトを実装する必要がある
#[allow(dead_code)]
#[derive(Debug)]
struct MyError(String);
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.0)
    }
}
impl std::error::Error for MyError {}

// 文字列からMyErrorインスタンスへの変換のためのFrom<String>トレイトを実装しておくと便利
impl From<String> for MyError {
    fn from(err: String) -> Self {
        Self(err)
    }
}

fn main() {
    #[allow(dead_code)]
    fn find_user(username: &str) -> Result<String, MyError> {
        Err(MyError::from(format!("User {} not found", username)))
    }

    // コンパイラは?演算子をからResult形で指定した返り値に変換するトレイト実装を提供する
    // これを利用することで、エラー処理を簡潔に記述できる
    #[allow(dead_code)]
    fn find_user2(username: &str) -> Result<String, MyError> {
        // ?演算子はResult型の値を返す
        #[allow(unused_variables)]
        let f = std::fs::File::open("users.txt")
            .map_err(|e| format!("User {} not found: {:?}", username, e))?; // MyErrorに変換する
        Ok(format!("User: {}", username))
    }
}
