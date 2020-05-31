#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: usize, B: usize
    }

    println!("{}", A * B)
}
