#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars, T: Chars,
    }

    let S: Vec<char> = S;
    let T: Vec<char> = T;

    let ans: String = S
        .iter()
        .zip(T)
        .map(|(&s, t)| format!("{}{}", s, t))
        .collect();
    echo!(ans);
}
