#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        N: i64,
    }

    let N: i64 = N;

    let mut ans = 0;

    let mut factors = N.factors_map();

    let mut chosen: HashMap<i64, i64> = HashMap::new();
    factors.remove(&1);

    loop {
        let mut flag = true;
        for (&k, &v) in factors.clone().iter() {
            let t = chosen.entry(k).or_default();
            if v > *t {
                *t += 1;
                factors.entry(k).and_modify(|e| *e -= *t);
                ans += 1;
                flag = false;
            }
        }
        if flag {
            break;
        }
    }
    println!("{}", ans)
}
