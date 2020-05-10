#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let (_, broken) = A.iter().fold((1, 0), |(order, broken), &ai| {
        if ai == order {
            (order + 1, broken)
        } else {
            (order, broken + 1)
        }
    });

    println!("{}", if broken > N - 1 { -1 } else { broken as isize });
}
