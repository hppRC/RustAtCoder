#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: usize,
    }
    let mut A: Vec<Vec<usize>> = vec![];
    let mut C: Vec<usize> = vec![];

    for _ in 0..N {
        input! {
            c: usize,
            a: [usize; M]
        }
        C.push(c);
        A.push(a);
    }

    let mut ans = 100000000000usize;

    for i in 0..1 << N {
        let pattern: Vec<bool> = (0..N).map(|j| (i >> j) & 1 == 1).collect();
        let mut tmp = vec![];
        let mut cost = 0;
        for j in 0..N {
            if pattern[j] {
                tmp.push(A[j].clone());
                cost += C[j];
            }
        }
        let mut tmp2 = vec![0; M];

        for ai in tmp.clone() {
            for i in 0..M {
                tmp2[i] += ai[i];
            }
        }
        if tmp2.iter().all(|&ai| ai >= X) {
            ans = std::cmp::min(ans, cost);
        }
    }
    echo!(if ans == 100000000000usize {
        -1
    } else {
        ans as isize
    });
}
