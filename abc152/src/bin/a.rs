use proconio::input;

fn main() {
    input! {
        n: i32, m: i32,
    }
    println!("{}", if n == m { "Yes" } else { "No" })
}
