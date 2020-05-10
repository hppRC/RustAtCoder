#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, mut K: usize,
        A: [Usize1; N],
    }

    let mut visited = vec![false; N];
    let mut from = 0;
    let mut to_cycle = 0;
    let mut cycle_detected = false;

    for _ in 0..K {
        if visited[from] {
            cycle_detected = true;
            break;
        }
        visited[from] = true;
        to_cycle += 1;
        from = A[from];
    }

    if !cycle_detected {
        println!("{}", from + 1);
        return;
    }

    let mut cycle_size = 0;
    let cycle_start = from;

    for _ in 0..K {
        cycle_size += 1;
        from = A[from];
        if from == cycle_start {
            break;
        }
    }

    K -= to_cycle;
    K %= cycle_size;

    for _ in 0..K {
        from = A[from];
    }

    println!("{}", from + 1);
}
