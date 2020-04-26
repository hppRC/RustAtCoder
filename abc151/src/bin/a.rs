#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        c: char
    }

    println!("{}", (c as u8 + 1) as char)
}
