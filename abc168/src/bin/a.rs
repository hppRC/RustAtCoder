#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize
    }

    println!(
        "{}",
        match N % 10 {
            2 | 4 | 5 | 7 | 9 => "hon",
            0 | 1 | 6 | 8 => "pon",
            _ => "bon",
        }
    )
}
