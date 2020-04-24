use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        n: u64,
        p: [u64; n]
    }
    let p: Vec<u64> = p;

    let (_, ans) = p.iter().fold((p[0], 0), |(min_of_left, ans), &pi| {
        if min_of_left >= pi {
            (pi, ans + 1)
        } else {
            (min_of_left, ans)
        }
    });

    println!("{:?}", ans);
}
