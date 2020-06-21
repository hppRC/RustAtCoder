#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        R: f64
    }

    println!("{}", R * 2f64 * std::f64::consts::PI)
}
