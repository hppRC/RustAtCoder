use competitive_hpp::prelude::*;

fn digit_num(num: u64) -> u32 {
    (num as f64).log10().trunc() as u32
}

fn leftmost_digit(num: u64) -> u8 {
    let digit = digit_num(num);
    (num / 10u64.pow(digit)) as u8
}

fn rightmost_digit(num: u64) -> u8 {
    (num % 10) as u8
}

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut counts = vec![vec![0; 10]; 10];

    for i in 1..=n {
        let a = leftmost_digit(i) as usize;
        let b = rightmost_digit(i) as usize;

        counts[a][b] += 1;
    }

    let mut ans = 0;

    for (i, j) in iproduct!(0..10, 0..10) {
        ans += counts[i][j] * counts[j][i];
    }

    println!("{}", ans);
}
