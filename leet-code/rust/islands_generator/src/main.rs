use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    let n = 20;
    let m = 20;

    let mut rng = StdRng::seed_from_u64(42);

    for _ in 0..n {
        for _ in 0..m {
            print!("{}", rng.random_range(0..=1));
        }
        println!();
    }
}
