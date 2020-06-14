#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        VW: [(i64, i64); N],
        Q: usize,
        vL: [(Usize1, i64); Q]
    }

    let max_L: usize = *vL.iter().map(|(v, L)| L).max().unwrap() as usize;

    let mut dp = vec![vec![0; N + 1]; max_L + 1];

    for (v, L) in vL {
        println!("{} {}", v, L);
    }
}
