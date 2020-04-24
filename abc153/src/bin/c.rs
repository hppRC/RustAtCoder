#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, K: usize,
        mut H: [usize; N]
    }

    H.sort();
    H.reverse();

    println!("{}", (K..H.len()).fold(0, |sum, i| sum + H[i]))
}
