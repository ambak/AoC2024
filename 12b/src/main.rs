use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn dfs(
    x: usize,
    y: usize,
    v: &Vec<Vec<u8>>,
    vis: &mut Vec<Vec<bool>>,
    m: &Vec<(i32, i32, usize, usize)>,
    n: i32,
    hs: &mut HashSet<(i32, i32, i32, i32)>,
) -> (i64, i64) {
    vis[x][y] = true;
    let mut res = (1, 0);
    for (a, b, c, d) in m {
        let xx = x as i32 + a;
        let yy = y as i32 + b;
        if xx >= 0
            && xx < n
            && yy >= 0
            && yy < n
            && !vis[xx as usize][yy as usize]
            && v[x][y] == v[xx as usize][yy as usize]
        {
            let ans = dfs(xx as usize, yy as usize, v, vis, m, n, hs);
            res.0 += ans.0;
            res.1 += ans.1;
        } else if xx < 0 || xx >= n || yy < 0 || yy >= n || v[x][y] != v[xx as usize][yy as usize] {
            hs.insert((*a, *b, xx, yy));
            let q1 = (*a, *b, xx + m[*c].0, yy + m[*c].1);
            let q2 = (*a, *b, xx + m[*d].0, yy + m[*d].1);
            if hs.get(&q1).is_none() && hs.get(&q2).is_none() {
                res.1 += 1;
            }
            if hs.get(&q1).is_some() && hs.get(&q2).is_some() {
                res.1 -= 1;
            }
        }
    }
    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    let n = v.len();
    let mut vis = vec![vec![false; n]; n];
    let m = vec![(1, 0, 2, 3), (-1, 0, 2, 3), (0, 1, 0, 1), (0, -1, 0, 1)];
    for x in 0..n {
        for y in 0..n {
            if !vis[x][y] {
                let mut hs = HashSet::new();
                let ans = dfs(x, y, &v, &mut vis, &m, n as i32, &mut hs);
                res += ans.0 * ans.1;
            }
        }
    }
    println!("{res}");
}
