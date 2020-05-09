#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N:isize,
        A: [isize; N],
    }
    let A: Vec<isize> = A;
    let mut map: HashMap<isize, isize> = HashMap::new();

    let ans = A.iter().enumerate().fold(0, |acc, (i, &a)| {
        let &tmp = map.get(&(i as isize - a)).unwrap_or(&0);
        map.entry(i as isize + a)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
        acc + tmp
    });

    println!("{}", ans)
}
