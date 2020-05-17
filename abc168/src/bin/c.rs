#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        A: f64,
        B: f64,
        H: f64,
        M: f64,
    }
    let t: f64 = ((30f64 * H + 0.5f64 * M - 6f64 * M) * (2.0f64 * PI) / 360f64).cos();
    let ans = (A * A + B * B - 2f64 * A * B * t).sqrt();
    println!("{}", ans);
}
