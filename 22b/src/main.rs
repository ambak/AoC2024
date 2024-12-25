use std::{
    cmp::max,
    io::{self, BufRead},
};

fn simulate(x: i64, step: usize, v: &mut Vec<Vec<Vec<Vec<i64>>>>) {
    let mut x = x;
    let mut prev = x;
    let (mut b, mut c, mut d) = (0, 0, 0);
    let mut s = vec![vec![vec![vec![false; 20]; 20]; 20]; 20];
    for i in 0..step {
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

        let diff = (prev % 10 - x % 10 + 10) as usize;
        let a = b;
        b = c;
        c = d;
        d = diff;
        if i >= 3 {
            if !s[a][b][c][d] {
                s[a][b][c][d] = true;
                v[a][b][c][d] += x % 10;
            }
        }
        prev = x;
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v: Vec<Vec<Vec<Vec<i64>>>> = vec![vec![vec![vec![0i64; 20]; 20]; 20]; 20];
    for line in lines {
        let x = line.unwrap().parse::<i64>().unwrap();
        simulate(x, 2000, &mut v);
    }
    for a in 0..20 {
        for b in 0..20 {
            for c in 0..20 {
                for d in 0..20 {
                    res = max(res, v[a][b][c][d]);
                }
            }
        }
    }
    println!("{res}");
}
