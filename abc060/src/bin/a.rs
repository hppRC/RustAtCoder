#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        A: Chars,
        B: Chars,
        C: Chars,
    }
    let A: Vec<char> = A;

    println!("{}", YESNO(A.last() == B.first() && B.last() == C.first()));
}
