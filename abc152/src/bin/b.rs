use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
    }

    let n = format!("{}", a).repeat(b);
    let m = format!("{}", b).repeat(a);

    println!("{}", String::min(n, m))
}
