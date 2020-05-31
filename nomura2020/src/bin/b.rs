#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        mut S: Chars,
    }

    let mut S: Vec<char> = S;
    let l = S.len();

    if S[l - 1] == '?' {
        S[l - 1] = 'D'
    }

    for i in (1..S.len()).rev() {
        if S[i - 1] == '?' && S[i] == 'P' {
            S[i - 1] = 'D'
        } else if S[i - 1] == '?' {
            S[i - 1] = 'D'
        }
    }

    let s: String = S.into_iter().collect();

    println!("{}", s)
}
