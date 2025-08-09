#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<i64> = vec![0, 1, 2, 3, 4, 5];
    // これだとresult内の個々のResult型に対して繰り返しエラーを確認する必要がある
    #[allow(unused_variables)]
    #[allow(clippy::redundant_closure)]
    let result: Vec<Result<u8, _>> = input.into_iter().map(|x| u8::try_from(x)).collect();
    // _ は型推論させるためのプレースホルダ

    let input: Vec<i64> = vec![0, 1, 2, 3, 4, 5];
    // これだと一度のエラーで全体が失敗する
    #[allow(unused_variables)]
    #[allow(clippy::redundant_closure)]
    let result: Vec<u8> = input
        .into_iter()
        .map(|x| u8::try_from(x))
        .collect::<Result<Vec<_>, _>>()?; // collect::Result<Vec<_>, _>はターボフィッシュ記法
    Ok(())
}

