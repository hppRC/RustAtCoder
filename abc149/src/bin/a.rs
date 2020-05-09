#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        S: String,
        T: String,
    }

    echo!(T + &S);
}
