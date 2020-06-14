#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i128; N]
    }

    let A: Vec<i128> = A;

    let mut map = HashMap::new();
    for &ai in A.iter() {
        map.entry(ai).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut d: Vec<i128> = map
        .iter()
        .filter(|(&k, &v)| v <= 1)
        .map(|(&k, &v)| k)
        .collect();

    d.sort();

    let max_ai = match d.last() {
        Some(&v) => v,
        None => {
            println!("0");
            return;
        }
    };
    let mut era: Vec<bool> = (0..=1_000_000).map(|_| false).collect();

    for ci in map.iter().filter(|(&k, &v)| v > 1).map(|(&k, &v)| k) {
        for i in (1..).take_while(|&i| ci * i <= max_ai) {
            era[(ci * i) as usize] = true;
        }
    }

    let mut ans = 0;
    for di in d.into_iter() {
        if !era[di as usize] {
            ans += 1;

            for i in (1..).take_while(|&i| di * i <= max_ai) {
                era[(di * i) as usize] = true;
            }
        }
    }
    println!("{}", ans);
}
