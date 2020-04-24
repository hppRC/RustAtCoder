#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H: u64,
    }

    let n = H.log2_trunc() as u32 + 1u32;
    println!("{}", 2u64.pow(n) - 1);
}
