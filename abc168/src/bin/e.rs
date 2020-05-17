#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(f64, f64); N]
    }

    let mut map = HashMap::new();

    for &(A, B) in &AB {
        let key: String = format!("{}", A / B);
        map.entry(key).and_modify(|entry| *entry += 1).or_insert(1);
    }

    println!("{:?}", map);
}
