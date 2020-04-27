#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

const P: u64 = 2019;

#[fastout]
fn main() {
    input! {
        S: Chars,
    }
    let S: Vec<char> = S;
    let mut ans = 0;
    let mut power_of_ten = 1u64;

    let mut map: HashMap<u64, u64> = HashMap::new();
    let mut prev: u64 = 0;

    for c in S.iter().rev() {
        let c: u64 = format!("{}", c).parse().unwrap_or(0);
        prev = (prev + c * power_of_ten) % P;
        map.entry(prev).and_modify(|key| *key += 1).or_insert(1);

        power_of_ten = (10 * power_of_ten) % P;
    }

    for (_, &v) in &map {
        ans += v * (v - 1) / 2;
    }
    ans += *map.get(&0).unwrap_or(&0);

    println!("{}", ans);
}
