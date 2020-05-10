#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: usize, B: usize,
    }

    println!(
        "{}",
        match A + B {
            3 => 3,
            4 => 2,
            5 => 1,
            _ => 0,
        }
    )
}
