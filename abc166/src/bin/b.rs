#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, K:usize,
    }

    let mut sunuke = vec![0; N];

    for i in 0..K {
        input! {
            d: usize,
            A: [Usize1; d],
        }
        for a in A {
            sunuke[a] += 1;
        }
    }

    println!(
        "{}",
        sunuke
            .iter()
            .fold(0, |acc, &i| if i == 0 { acc + 1 } else { acc })
    )
}
