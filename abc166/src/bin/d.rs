#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        X: i64,
    }
    let X: i64 = X;

    'outer: for a in -200..200 {
        for b in -200..200 {
            if (a.pow(5) - b.pow(5)) == X {
                println!("{} {}", a, b);
                break 'outer;
            }
        }
    }
}
