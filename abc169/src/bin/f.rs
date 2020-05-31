#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, S: usize,
        A: [usize; N]
    }

    let mut count = vec![0; S + 1];
    let mut dp = vec![0; S + 1];
    let A: Vec<usize> = A;

    for i in 0..N {
        if A[i] <= S {
            count[A[i]] += 1;
        }
    }

    println!("{:?}", count);

    for i in 1..S + 1 {
        dp[i] = (1..i).map(|j| dp[i - j] * count[j]).sum::<i64>() + count[i];
        println!("{:?}", dp[i]);
    }
}
