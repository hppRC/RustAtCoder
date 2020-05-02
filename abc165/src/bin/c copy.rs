#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() -> () {
    input! {
        N: usize, M:usize, Q:usize,
        abcd: [(Usize1, Usize1, usize, usize); Q]
    }
    let abcd: Vec<(usize, usize, usize, usize)> = abcd;

    let mut perms = vec![vec![0; 0]; 0];
    rec(M, N, vec![0; 0], &mut perms);

    let ans: usize = perms
        .iter()
        .map(|A| {
            abcd.iter()
                .filter_map(|&(a, b, c, d)| if A[b] == A[a] + c { Some(d) } else { None })
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}

fn rec(M: usize, n: usize, v: Vec<usize>, perms: &mut Vec<Vec<usize>>) -> () {
    if n == 0 {
        perms.push(v.clone());
    } else {
        let &tail = v.last().unwrap_or(&1usize);
        for i in tail..M + 1 {
            let mut tmp = v.clone();
            tmp.push(i);
            rec(M, n - 1, tmp.clone(), perms);
        }
    }
}

// #[fastout]
// fn main() {
//     input! {
//         N: usize, M:usize, Q:usize,
//         abcd: [(Usize1, Usize1, usize, usize); Q]
//     }
//     let abcd: Vec<(usize, usize, usize, usize)> = abcd;
//     let ans = (1..M + 1usize)
//         .combinations_with_replacement(N)
//         .map(|A| {
//             abcd.iter().fold(
//                 0,
//                 |acc, &(a, b, c, d)| if A[b] == A[a] + c { acc + d } else { acc },
//             )
//         })
//         .max()
//         .unwrap();
//     println!("{}", ans);
// }
