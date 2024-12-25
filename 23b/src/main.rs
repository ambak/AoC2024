#[macro_use]
extern crate scan_fmt;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn f(res: &mut Vec<Vec<usize>>, grid: &Vec<Vec<bool>>) {
    let mut res2 = vec![];
    for i in 0..res.len() {
        for j in i + 1..res.len() {
            let mut ok = true;
            for k in 0..res[i].len() - 1 {
                if res[i][k] != res[j][k] {
                    ok = false;
                }
            }
            let m = res[i].len() - 1;
            if ok {
                if grid[res[i][m]][res[j][m]] {
                    let mut tmp = res[i].clone();
                    tmp.push(res[j][m]);
                    tmp.sort();
                    res2.push(tmp);
                }
            } else {
                break;
            }
        }
    }
    res.clear();
    *res = res2;
    res.sort();
    res.dedup();
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v = vec![];
    let mut hm = HashMap::new();
    let mut hm2 = HashMap::new();
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
            hm2.insert(val1, u);
            hm2.insert(val2, w);
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
    let mut res: Vec<Vec<usize>> = vec![];
    for (_key, val) in hm {
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
    res.sort();
    res.dedup();
    while res.len() > 1 {
        f(&mut res, &grid);
    }
    let mut ans = vec![];
    for v in res {
        for x in &v {
            ans.push(hm2.get(x).unwrap().clone());
        }
    }
    ans.sort();
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i != ans.len() - 1 {
            print!(",");
        }
    }
    println!();
}
