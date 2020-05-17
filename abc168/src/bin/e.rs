#![allow(non_snake_case)]
#![allow(unused_variables)]

use competitive_hpp::prelude::*;

fn normalize(x: i64, y: i64) -> (i64, i64) {
    if x > 0 && y >= 0 {
        let gcd = x.gcd(&y);
        (x / gcd, y / gcd)
    } else {
        normalize(-y, x)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(i64, i64); N]
    }
    let mut map: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut zeros = 0;

    for &(a, b) in &AB {
        if a == 0 && b == 0 {
            zeros += 1;
            continue;
        }
        map.entry(normalize(a, b)).or_default().push((a, b));
    }

    let mut ans = ModInt::new(1i64);
    for (_, v) in map {
        let (mut plus, mut minus) = (0, 0);

        for (x, y) in v {
            if x > 0 && y >= 0 {
                plus += 1;
            } else if x <= 0 && y > 0 {
                minus += 1;
            } else if x < 0 && y <= 0 {
                plus += 1;
            } else {
                minus += 1;
            }
        }

        let current = ModInt::new(2i64).pow(plus) + ModInt::new(2i64).pow(minus) - 1;
        ans = ans * current;
    }

    println!("{}", ans - 1 + zeros);
}
