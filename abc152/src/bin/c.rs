use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        n: u64,
        p: [u64; n]
    }

    let mut min_of_left = p[0];
    let mut ans = 0;

    for pj in p {
        if min_of_left >= pj {
            min_of_left = pj;
            ans += 1;
        }
    }

    println!("{}", ans)
}
