#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, K:usize,
        mut A: [i64; N]
    }

    for _ in 0..K {
        let mut B: Vec<i64> = vec![0i64; N + 1];

        for i in 0usize..N {
            let l = std::cmp::max(i as i64 - A[i], 0) as usize;
            let r = std::cmp::min(i as i64 + A[i] + 1, N as i64) as usize;

            B[l] += 1;
            B[r] -= 1;
        }
        B = B.iter().cumsum().collect();
        B.pop();

        if A == B {
            break;
        } else {
            A = B
        }
    }
    echo!(A);
}
