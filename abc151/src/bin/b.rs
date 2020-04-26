#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: isize, K: isize, M: isize,
        A: [isize; N-1],
    }

    println!(
        "{}",
        if N * M <= A.iter().sum::<isize>() + K {
            max(N * M - A.iter().sum::<isize>(), 0)
        } else {
            -1
        }
    );
}
