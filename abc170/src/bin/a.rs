#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: [i64; 5]
    }

    for i in 0..5 {
        if A[i] == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
