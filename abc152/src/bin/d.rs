use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut counts = vec![vec![0; 10]; 10];

    for i in 1..=n {
        let a = i.leftmost_digit() as usize;
        let b = i.rightmost_digit() as usize;
        counts[a][b] += 1;
    }

    let ans = iproduct!(0..10, 0..10).fold(0, |sum, (i, j)| sum + counts[i][j] * counts[j][i]);
    println!("{}", ans);
}
