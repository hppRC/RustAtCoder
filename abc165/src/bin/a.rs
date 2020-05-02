#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        K:usize,
        A:usize, B:usize
    }

    let mut tmp = K;
    loop {
        if tmp >= A && tmp <= B {
            println!("OK");
            break;
        }

        if tmp > B {
            println!("NG");
            break;
        }
        tmp += K;
    }
}
