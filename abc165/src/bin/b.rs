#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        X: f64,
    }
    let mut cnt: f64 = 100f64;
    let X: f64 = X;
    let mut t = 0;
    loop {
        t += 1;
        cnt = (cnt * 1.01f64).trunc();
        if cnt >= X {
            println!("{}", t);
            break;
        }
    }
}
