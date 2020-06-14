#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: i64, V:i64,
        B: i64, W: i64,
        T: i64
    }

    let d = V - W;

    if d > 0 {
        let distance: i64 = (B - A).abs();
        if T * d >= distance {
            println!("YES")
        } else {
            println!("NO")
        }
    } else {
        println!("NO")
    }
}
