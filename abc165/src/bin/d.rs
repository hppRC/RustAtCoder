#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A:f64, B:f64, N:f64,
    }
    let A: f64 = A;
    let B: f64 = B;
    let N: f64 = N;

    let tmp = if B - 1f64 > N { N } else { B - 1f64 };
    println!("{}", (A * tmp / B).trunc())
}
