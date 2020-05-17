#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        K: usize,
        S: String,
    }

    let K: usize = K;
    let S: String = S;

    println!(
        "{}",
        if S.len() > K {
            S[..K].to_owned() + "..."
        } else {
            S
        }
    )
}
