#[macro_use]
extern crate scan_fmt;
use std::{
    cmp::min,
    io::{self, BufRead},
};

fn f(xa: i32, ya: i32, xb: i32, yb: i32, xp: i32, yp: i32) -> usize {
    let mut res = 999999999;
    let mut dp = vec![vec![(0, 0); 101]; 101];
    for i in 1..=100 {
        dp[0][i] = (dp[0][i - 1].0 + xa, dp[0][i - 1].1 + ya);
    }
    for i in 0..=100 {
        for j in 1..=100 {
            dp[j][i] = (dp[j - 1][i].0 + xb, dp[j - 1][i].1 + yb);
            if dp[j][i] == (xp, yp) {
                res = min(res, 3 * i + j);
            }
        }
    }
    if res == 999999999 {
        res = 0;
    }
    res
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
        if let Ok((t, x, y)) = scan_fmt!(&l, "Button {}: X+{}, Y+{}", String, i32, i32) {
            if t == "A" {
                xa = x;
                ya = y;
            } else {
                xb = x;
                yb = y;
            }
        } else if let Ok((_, x, y)) = scan_fmt!(&l, "{}: X={}, Y={}", String, i32, i32) {
            xp = x;
            yp = y;
            res += f(xa, ya, xb, yb, xp, yp);
        }
    }

    println!("{res}");
}
