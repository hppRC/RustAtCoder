use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    for permu in (0..4).permutations(4) {
        println!("{:?}", permu)
    }
}
