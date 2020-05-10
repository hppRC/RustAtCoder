#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        K: isize,
    }

    println!(
        "{}",
        if K <= A {
            K
        } else if K <= A + B {
            A
        } else {
            let tmp = K - A - B;
            A - tmp
        }
    )
}
