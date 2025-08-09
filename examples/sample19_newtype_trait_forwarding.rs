#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

// newtypeの制限
// newtypeを生成すると内部型に対するトレイト実装が失われる
// 導出できるトレイトは,newTypeの宣言にderive属性を付けることで実装できる
// ただし、複雑なトレイトについては内部型の実装を復活させるために定型的なフォワードコードを書く必要がある。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InnerType(i32);
trait Double {
    fn double(&self) -> Self;
}
impl Double for InnerType {
    fn double(&self) -> Self {
        InnerType(self.0 * 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NewType(InnerType);
// NewTypeはInnerTypeのdoubleメソッドをフォワードする
impl Double for NewType {
    fn double(&self) -> Self {
        NewType(self.0.double())
    }
}

fn main() {
    let inner = InnerType(10);
    let newtype = NewType(inner);

    println!("InnerType: {:?}", inner.double());
    println!("NewType: {:?}", newtype.double());
}

