#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: usize, B: usize,
    }

    println!("{}", A.lcm(&B));
}
