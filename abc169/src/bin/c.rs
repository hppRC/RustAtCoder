#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: i128, B: f64,
    }

    let A: i128 = A;
    let B: f64 = B;
    let B = (B * 1000f64) as i128;
    let ans = (A * B) / 1000;

    println!("{}", ans);
}
