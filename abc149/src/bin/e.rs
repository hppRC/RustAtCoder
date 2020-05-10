#![allow(non_snake_case)]
#![allow(unused_variables)]
use competitive_hpp::prelude::*;

fn main() {
    input! {
        N: usize, M: usize,
        mut A: [usize; N]
    }

    let mut A: Vec<usize> = A;
    A.sort();

    let mut ng = 0;
    let mut ok = A.last().unwrap() * 2;

    while ok - ng > 1 {
        let x = (ng + ok) / 2;
        let s = A.iter().fold(0, |acc, &ai| {
            if x <= ai {
                acc + N
            } else {
                acc + (N - A.binary_search(&(x - ai)).unwrap_or_else(|i| i))
            }
        });

        if s <= M {
            ok = x;
        } else {
            ng = x;
        }
    }

    println!("{} {}", ok, ng);

    let cumsum: Vec<usize> = A.iter().cumsum().collect();

    let mut ans = 0;

    for i in 0..N {
        let pos = if ok > A[i] {
            A.binary_search(&(ok - A[i])).unwrap_or_else(|j| j)
        } else {
            0
        };
        let count = N - pos;
        ans += A[i] * count + (cumsum.last().unwrap() - cumsum[pos - 1]);
        println!("{} {} {} {}", A[i], pos, count, ans);
    }
    echo!(ans);
}
