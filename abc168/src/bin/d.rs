#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize, M: usize,
        AB: [(Usize1, Usize1); M],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; N];
    for &(a, b) in &AB {
        g[a].push(b);
        g[b].push(a);
    }

    let mut visited: Vec<bool> = vec![false; N];
    let mut light_to: Vec<usize> = vec![0; N];

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;

    while !queue.is_empty() {
        if let Some(u) = queue.pop_front() {
            for &v in &g[u] {
                if !visited[v] {
                    visited[v] = true;
                    light_to[v] = u + 1;
                    queue.push_back(v);
                }
            }
        }
    }

    if visited.iter().all(|&judge| judge) {
        println!("Yes");
        for i in &light_to[1..] {
            println!("{}", i);
        }
    } else {
        println!("Yes");
    }
}
