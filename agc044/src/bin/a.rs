#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

fn solve(N: i64, A: i64, B: i64, C: i64, D: i64) -> i64 {
    let mut cost_map = hashmap! { N => 0 };
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0i64), N));

    while let Some((Reverse(cost), n)) = heap.pop() {
        let mut candedates = vec![];
        let ABC = vec![0, 0, A, B, 0, C];

        for i in vec![2i64, 3i64, 5i64] {
            let ceil_i = n.div_ceil(&i) * i;
            let ceil_i_cost = (ceil_i - n).abs() * D + ABC[i as usize] + cost;
            candedates.push((ceil_i / i, ceil_i_cost));

            let floor_i = n.div_floor(&i) * i;
            let floor_i_cost = (floor_i - n).abs() * D + ABC[i as usize] + cost;
            candedates.push((floor_i / i, floor_i_cost));
        }

        for &(next_n, next_cost) in candedates.iter() {
            if next_n == 0 {
                continue;
            }

            let tmp = cost_map.entry(next_n).or_insert(std::i64::MAX);
            if *tmp > next_cost {
                *tmp = next_cost;
                heap.push((Reverse(next_cost), next_n));
            }
        }

        let ans = cost_map.entry(0).or_insert(std::i64::MAX);
        if n < std::i64::MAX / D {
            if *ans > n * D + cost {
                *ans = n * D + cost;
            }
        }
    }

    *cost_map.get(&0).unwrap()
}

#[fastout]
fn main() {
    input! {
        T: i64,
        NABCD: [(i64, i64, i64, i64, i64); T]
    }

    for (N, A, B, C, D) in NABCD {
        println!("{}", solve(N, A, B, C, D))
    }
}
