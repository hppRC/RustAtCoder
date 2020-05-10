#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        N: isize, M: isize, K: isize,
    }
    const MOD: isize = 998244353;

    let modfact = ModFact::new(N as usize, MOD as usize);

    let mut ans = 0;
    for ki in 0..=K {
        let M1_count: isize = N - 1 - ki;
        let mut tmp: isize = M * modfact.comb(N as usize - 1, ki as usize) as isize % MOD;
        tmp = tmp * mod_pow(M as isize - 1, M1_count, MOD) as isize % MOD;
        ans = (ans + tmp) % MOD;
    }

    println!("{}", ans % MOD);
}
