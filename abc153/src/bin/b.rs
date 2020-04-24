#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H: usize, N:usize,
        A: [usize; N]
    }

    println!("{}", YesNo(A.iter().sum::<usize>() >= H))
}
