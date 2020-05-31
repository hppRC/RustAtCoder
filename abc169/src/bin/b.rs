#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i128; N],
    }

    let mut ans = 1;
    for i in 0..N {
        ans *= A[i];
        if ans > (10f64.powi(18) as i128) {
            ans = -1;
            break;
        }
    }

    for i in 0..N {
        if A[i] == 0 {
            ans = 0;
        }
    }

    println!("{}", ans)
}
