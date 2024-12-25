#[macro_use]
extern crate scan_fmt;
use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 1i64;
    let n = 101;
    let m = 103;
    let t = 100;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap();
        if let Ok((xp, yp, xv, yv)) = scan_fmt!(&l, "p={},{} v={},{}", i64, i64, i64, i64) {
            let x = ((xp + t * xv) % n + n) % n;
            let y = ((yp + t * yv) % m + m) % m;
            v.push((x, y));
        }
    }
    let mut square = vec![0; 4];
    for (x, y) in v {
        if x < n / 2 && y < m / 2 {
            square[0] += 1;
        } else if x < n / 2 && y > m / 2 {
            square[1] += 1;
        } else if x > n / 2 && y < m / 2 {
            square[2] += 1;
        } else if x > n / 2 && y > m / 2 {
            square[3] += 1;
        }
    }
    for k in square {
        res *= k;
    }
    println!("{res}");
}
