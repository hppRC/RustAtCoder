#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: i64, K:i64
    }

    let mut ans = ModInt::new(0i64);
    for i in K..=N + 1 {
        let left = (i * (i - 1)) / 2;
        let right = (N * (N + 1)) / 2 - ((N - i) * (N - i + 1)) / 2;

        ans = ans + (right - left + 1);
    }
    println!("{}", ans);
}
