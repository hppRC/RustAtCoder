#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let mut out_of_lparen = 0;
    let mut out_of_rparen = 0;
    let mut over_left = 0;
    let mut over_right = 0;

    for i in 0..N {
        let mut left_start = 0;
        let mut right_start = S[i].len() - 1;
        for j in 0..S[i].len() {
            if S[i][j] == ')' {
                out_of_lparen += 1;
            } else {
                left_start = j;
                break;
            }
        }
        for j in (left_start..S[i].len()).rev() {
            if S[i][j] == '(' {
                out_of_rparen += 1;
            } else {
                right_start = j;
                break;
            }
        }

        let mut left = 0;
        let mut right = 0;

        for j in left_start..=right_start {
            match S[i][j] {
                '(' => left += 1,
                ')' => right += 1,
                _ => {}
            }
        }

        if left > right {
            over_left += left - right;
        } else if right > left {
            over_right += right - left;
        }
    }

    println!(
        "{}",
        YesNo(out_of_lparen == over_left && out_of_rparen == over_right)
    )
}
