#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N:usize,
        S:[String; N],
    }

    let mut set: HashSet<String> = HashSet::new();
    for s in S {
        set.insert(s);
    }

    println!("{}", set.len());
}
