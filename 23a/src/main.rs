#[macro_use]
extern crate scan_fmt;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    let mut hm = HashMap::new();
    for line in lines {
        let l = line.unwrap();
        if let Ok((u, w)) = scan_fmt!(&l, "{}-{}", String, String) {
            let (val1, val2);
            if !hm.contains_key(&u) {
                val1 = hm.len();
                hm.insert(u.clone(), hm.len());
            } else {
                val1 = *hm.get(&u).unwrap();
            }
            if !hm.contains_key(&w) {
                val2 = hm.len();
                hm.insert(w.clone(), hm.len());
            } else {
                val2 = *hm.get(&w).unwrap();
            }
            v.push((val1, val2));
        }
    }
    let n = hm.len();
    let mut grid = vec![vec![false; n]; n];
    let mut g = vec![vec![]; n];
    for (u, w) in v {
        grid[u][w] = true;
        grid[w][u] = true;
        g[u].push(w);
        g[w].push(u);
    }
    let mut res = vec![];
    for (key, val) in hm {
        if key.chars().nth(0).unwrap() == 't' {
            for i in 0..g[val].len() {
                for j in i + 1..g[val].len() {
                    if grid[g[val][i]][g[val][j]] {
                        let mut tmp = vec![val, g[val][i], g[val][j]];
                        tmp.sort();
                        res.push(tmp);
                    }
                }
            }
        }
    }
    res.sort();
    res.dedup();
    println!("{}", res.len());
}
