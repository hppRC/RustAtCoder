#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

fn main() {
    input! {
        N: usize, M:usize,
        A: [usize; N],
    }

    let A: Vec<usize> = A.iter().map(|ai| ai / 2).collect();
    let exp2: Vec<u32> = A.iter().map(|ai| ai.trailing_zeros()).collect();
    let able = exp2.windows(2).all(|w| w[0] == w[1]);

    if able {
        let lcm = A.iter().fold(1, |acc, i| acc.lcm(i));
        echo!((M / lcm + 1) / 2);
    } else {
        echo!(0);
    }
}
