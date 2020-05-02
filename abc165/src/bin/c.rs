#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, M:usize, Q:usize,
        abcd: [(Usize1, Usize1, usize, usize); Q]
    }
    let abcd: Vec<(usize, usize, usize, usize)> = abcd;
    let ans: usize = (1..M + 1)
        .combinations_with_replacement(N)
        .map(|A| {
            abcd.iter()
                .filter_map(|&(a, b, c, d)| if A[b] == A[a] + c { Some(d) } else { None })
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
