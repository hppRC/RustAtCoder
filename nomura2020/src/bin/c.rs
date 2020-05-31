#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N+1]
    }

    let N: usize = N;
    let A: Vec<i64> = A;

    println!("{}", if flag { ans } else { -1 })
}
