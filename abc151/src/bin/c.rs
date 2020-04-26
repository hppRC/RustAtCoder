#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, M:usize,
        ps: [(Usize1, String); M]
    }
    let ps: Vec<(usize, String)> = ps;

    let mut wa = vec![0; N];
    let mut solved = vec![false; N];
    let mut ac = 0;

    for (pi, result) in ps {
        if solved[pi] {
            continue;
        }
        if result == "AC".to_string() {
            solved[pi] = true;
            ac += 1;
        } else {
            wa[pi] += 1;
        }
    }

    println!(
        "{} {}",
        ac,
        (0..N).fold(0, |sum, i| if solved[i] { sum + wa[i] } else { sum })
    );
}
