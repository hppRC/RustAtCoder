#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1]
    }
    let A: Vec<usize> = A;

    let mut g: Vec<usize> = vec![0; N + 1];
    for (i, &ai) in A.iter().enumerate() {
        g[ai] += 1;
    }

    for i in 1usize..=N {
        println!("{}", g[i]);
    }
}
