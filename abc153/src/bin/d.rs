#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H: f64,
    }
    let n = H.log2().trunc() as u32 + 1u32;
    println!("{}", 2u64.pow(n) - 1);
}
