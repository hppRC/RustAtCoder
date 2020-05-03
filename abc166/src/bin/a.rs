#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        S:String
    }

    println!("{}", if S == "ABC" { "ARC" } else { "ABC" })
}
