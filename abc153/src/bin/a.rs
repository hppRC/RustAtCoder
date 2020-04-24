use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H: usize, A:usize
    }

    println!("{}", (H + A - 1) / A)
}
