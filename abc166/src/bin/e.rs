#![allow(non_snake_case)]
use competitive_hpp::prelude::*;
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        N:isize,
        A: [isize; N],
    }
    let A: Vec<isize> = A;

    let mut vp: Vec<(isize, usize)> = A
        .iter()
        .enumerate()
        .map(|(i, &j)| (i as isize + j, i))
        .collect();
    vp.sort_by_key(|&i| i.0);

    let mut vm: Vec<(isize, usize)> = A
        .iter()
        .enumerate()
        .map(|(i, &j)| (i as isize - j, i))
        .collect();
    vm.sort_by_key(|&i| i.0);

    let mut ans: HashSet<(usize, usize)> = HashSet::new();

    for (id, &length) in A.iter().enumerate() {
        let query = id as isize - length;

        if let Ok(v) = vp.binary_search_by_key(&query, |&k| k.0) {
            let large = max(id, vp[v].1);
            let small = min(id, vp[v].1);
            ans.insert((small, large));
        }
    }

    for (id, &length) in A.iter().enumerate() {
        let query = id as isize + length;

        if let Ok(v) = vm.binary_search_by_key(&query, |&k| k.0) {
            let large = max(id, vm[v].1);
            let small = min(id, vm[v].1);
            ans.insert((small, large));
        }
    }

    println!("{}", ans.len());
}
