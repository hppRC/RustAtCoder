#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(u64, u64); N]
    }
    let AB: Vec<(u64, u64)> = AB;

    let mut A: Vec<u64> = AB.iter().map(|&(a, b)| a).collect();
    let mut B: Vec<u64> = AB.iter().map(|&(a, b)| b).collect();
    A.sort();
    B.sort();

    let Alen = A.len();
    let Blen = B.len();

    let medianA = if Alen % 2 == 0 {
        let midA = Alen / 2;
        A[midA] + A[midA - 1]
    } else {
        A[Alen / 2]
    };
    let medianB = if Blen % 2 == 0 {
        let midB = Blen / 2;
        B[midB] + B[midB - 1]
    } else {
        B[Blen / 2]
    };

    println!("{:?}", medianB - medianA + 1);
}
