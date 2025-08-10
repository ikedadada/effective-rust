#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MyBooleanOption {
    #[default] // デフォルト値は Off
    Off,
    On,
}

impl std::fmt::Display for MyBooleanOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyBooleanOption::Off => write!(f, "Off"),
            MyBooleanOption::On => write!(f, "On"),
        }
    }
}

fn main() {
    // Clone
    // この型のアイテムはユーザーが定義したコードを用いて自分をコピーできる
    let a = MyBooleanOption::On;
    #[allow(clippy::clone_on_copy)]
    let b = a.clone();
    assert_eq!(a, b);
    // ポインタ比較
    assert!(!std::ptr::eq(&a, &b));

    // Copy
    // コンパイラが（ユーザー定義コードを実行するのではなく）アイテムのメモリ上での表現のビットイメージのコピーを作れば、
    // その結果が有効な新しいアイテムになる
    let c = a; // 暗黙的に Copyトレイトが呼ばれる
    // そのため、aはまだ有効である
    assert_eq!(a, c);
    // ポインタ比較
    assert!(!std::ptr::eq(&a, &c));

    // Default
    // 適切なデフォルト値で新しいインスタンスを作ることができる
    let d: MyBooleanOption = Default::default();
    assert_eq!(d, MyBooleanOption::Off);

    // PardialEq, Eq
    // #[derive(PartialEq, Eq, PartialOrd, Ord)]
    // struct Oddity(f32); // Eqトレイトは実装できない（f32::NAN == f32::NANが成り立たないため）
    #[derive(PartialEq, Debug)]
    struct Oddity(f32); // PartialEqトレイトは実装できる
    let e = Oddity(1.0);
    let f = Oddity(1.0);
    assert_eq!(e, f);

    let g = Oddity(f32::NAN);
    let h = Oddity(f32::NAN);
    assert!(g != h); // f32::NAN != f32::NAN

    // PartialOrd, Ord
    // Ordトレイトは実装できない（f32::NAN < f32::NANが成り立たないため）
    #[derive(PartialEq, PartialOrd, Debug)]
    struct Oddity2(f32);
    let i = Oddity2(1.0);
    let j = Oddity2(2.0);
    assert!(i < j);
    let k = Oddity2(f32::NAN);
    let l = Oddity2(f32::NAN);
    assert!(k != l); // f32::NANは順序付けできない

    // Hash
    // Hashトレイトは異なるアイテムに対して高い確率で互いに異なるような1つの値（ハッシュ）を生成する
    // このハッシュ値は、ハッシュバケットに基づくデータ構造（HashMapやHashSet）の基礎として用いられるため、
    // Eqトレイトも実装する必要がある
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    a.hash(&mut hasher);
    let hash_value = hasher.finish();
    println!("Hash value of MyBooleanOption::On: {}", hash_value);

    // DebugとDisplay
    // DebugトレイトとDisplayトレイトを用いると通常のフォーマット（{}）もしくはデバッグ出力（{:?}）で出力する際の出力形式を指定することができる
    // この2つのトレイトの相違は、フォーマット演算子が異なるというだけではない
    // - Debugは自動導出できるが、Displayはユーザーが実装する必要がある
    // - Debug出力のレイアウトは、Rustのバージョンが変わると変更される可能性がある
    // - Debugはプログラマ向けで、Diplayはユーザ向けである
    println!(
        "MyBooleanOption::On in Debug format: {:?}",
        MyBooleanOption::On
    );
    println!(
        "MyBooleanOption::On in Display format: {}",
        MyBooleanOption::On
    );
}
