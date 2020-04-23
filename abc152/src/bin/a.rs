use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        n: i64,m: i64,
    }

    println!("{}", if n == m { "Yes" } else { "No" })
}
