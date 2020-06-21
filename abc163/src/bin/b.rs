#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: i64, M: usize,
        A: [i64; M],
    }

    let sum: i64 = A.iter().sum();

    if sum > N {
        println!("-1")
    } else {
        println!("{}", N - sum)
    }
}
