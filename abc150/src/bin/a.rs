#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        K:usize, X:usize
    }

    println!("{}", YesNo(500 * K >= X))
}
