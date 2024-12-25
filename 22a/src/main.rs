use std::io::{self, BufRead};

fn simulate(x: i64, step: usize) -> i64 {
    let mut x = x;
    for _ in 0..step {
        let mut secret = x;
        x <<= 6;
        x ^= secret;
        x &= 16777215;
        secret = x;
        x >>= 5;
        x ^= secret;
        x &= 16777215;
        secret = x;
        x <<= 11;
        x ^= secret;
        x &= 16777215;
    }
    x
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let x = line.unwrap().parse::<i64>().unwrap();
        res += simulate(x, 2000);
    }
    println!("{res}");
}
