#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        mut K: usize,
    }

    if K > A {
        K -= A;
        A = 0;

        if K > B {
            B = 0;
        } else {
            B -= K;
        }
    } else {
        A -= K;
    }

    echo!(A, B);
}
