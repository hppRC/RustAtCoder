#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H1: isize, M1: isize, H2: isize, M2: isize, K: isize
    };
    let tmp = M2 - M1;
    let tmp2 = H2 - H1;
    let ans = tmp2 * 60 + tmp - K;

    println!("{}", ans)
}
