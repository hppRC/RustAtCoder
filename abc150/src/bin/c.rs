#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
        Q: [usize; N],
    }

    let mut a: isize = 0;
    let mut b: isize = 0;

    for (i, perm) in (1..=N).permutations(N).into_iter().enumerate() {
        if perm == P {
            a = i as isize
        }
        if perm == Q {
            b = i as isize
        }
    }

    echo!(abs(a - b));
}
