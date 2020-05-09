#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        X: usize,
    }
    echo!((X..).find(|&i| i.is_prime()).unwrap());
}
