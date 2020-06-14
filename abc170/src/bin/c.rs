#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        X: i64, N:usize,
        p: [i64; N],
    }

    let p: Vec<_> = p;

    let mut ans = -1;

    for i in -1..=101 {
        if (X - ans).abs() > (X - i).abs() && !p.contains(&i) {
            ans = i;
        }
    }
    println!("{}", ans);
}
