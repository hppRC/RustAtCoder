#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        S: Chars,
        mut T: Chars,
    }

    T.pop();

    println!("{}", YesNo(S == T));
}
