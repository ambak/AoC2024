#[macro_use]
extern crate scan_fmt;
use std::io::{self, BufRead};

fn f(xa: i64, ya: i64, xb: i64, yb: i64, xp: i64, yp: i64) -> i64 {
    let d;
    if (yp * xa - xp * ya) % (yb * xa - xb * ya) == 0 {
        d = (yp * xa - xp * ya) / (yb * xa - xb * ya);
    } else {
        return 0;
    }
    let c;
    if (xp - d * xb) % xa == 0 {
        c = (xp - d * xb) / xa;
    } else {
        return 0;
    }
    if d < 0 || c < 0 {
        return 0;
    }
    return 3 * c + d;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut xa = 0;
    let mut ya = 0;
    let mut xb = 0;
    let mut yb = 0;
    let mut xp;
    let mut yp;
    for line in lines {
        let l = line.unwrap();
        if let Ok((t, x, y)) = scan_fmt!(&l, "Button {}: X+{}, Y+{}", String, i64, i64) {
            if t == "A" {
                xa = x;
                ya = y;
            } else {
                xb = x;
                yb = y;
            }
        } else if let Ok((_, x, y)) = scan_fmt!(&l, "{}: X={}, Y={}", String, i64, i64) {
            xp = x;
            yp = y;
            res += f(xa, ya, xb, yb, xp + 10000000000000, yp + 10000000000000);
        }
    }
    println!("{res}");
}
