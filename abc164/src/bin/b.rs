#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        mut A:isize,
        B:isize,
        mut C:isize,
        D:isize,
    }

    loop {
        C -= B;
        if C <= 0 {
            println!("Yes");
            break;
        }
        A -= D;
        if A <= 0 {
            println!("No");
            break;
        }
    }
}
