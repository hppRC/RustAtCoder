#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N:usize,M:usize,
        H: [usize; N],
        AB: [(Usize1, Usize1); M],
    }

    let N: usize = N;

    let mut g: Vec<Vec<usize>> = vec![vec![]; N];
    for &(A, B) in &AB {
        g[A].push(B);
        g[B].push(A);
    }

    let mut ans = 0;

    for i in 0..N {
        let mut flag = true;
        for &other in &g[i] {
            if H[other] >= H[i] {
                flag = false;
            }
        }
        if flag {
            ans += 1;
        }
    }

    println!("{:?}", ans);
}
