#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        S: Chars
    }

    println!("{}", S[0..3].iter().collect::<String>())
}
