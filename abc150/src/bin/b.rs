#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        _: usize,
        S: Chars,
    }

    let S: Vec<char> = S;

    println!(
        "{}",
        S.windows(3).fold(0, |acc, chars| {
            if chars.into_iter().collect::<String>() == "ABC" {
                acc + 1
            } else {
                acc
            }
        })
    )
}
