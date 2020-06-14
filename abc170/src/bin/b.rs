#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        X: i64, Y: i64,
    }

    for x in 0..=100 {
        for y in 0..=100 {
            if x + y == X && 2 * x + 4 * y == Y {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}
