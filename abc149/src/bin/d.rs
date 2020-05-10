#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

fn to_win(&ch: &char) -> char {
    if ch == 'r' {
        'p'
    } else if ch == 's' {
        'r'
    } else {
        's'
    }
}

#[fastout]
fn main() {
    input! {
        N: usize, K:usize,
        R: usize, S: usize, P: usize,
        T: Chars
    }

    let mut origin: Vec<char> = T.iter().map(to_win).collect();

    for (i, _) in origin.clone().iter().enumerate() {
        if i + K < N {
            if origin[i + K] == origin[i] {
                origin[i + K] = ' '
            }
        }
    }

    let get_score = |&ch: &char, other: &char| {
        if to_win(&other) == ch {
            match ch {
                'r' => R,
                's' => S,
                'p' => P,
                _ => 0,
            }
        } else {
            0
        }
    };

    let ans = origin
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &ch)| acc + get_score(&ch, &T[i]));

    echo!(ans);
}
