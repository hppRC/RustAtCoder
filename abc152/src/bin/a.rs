use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        n: i64,m: i64,
    }

    println!("{}", YesNo(n == m))
}
