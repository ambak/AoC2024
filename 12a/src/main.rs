use std::io::{self, BufRead};

fn dfs(
    x: usize,
    y: usize,
    v: &Vec<Vec<u8>>,
    vis: &mut Vec<Vec<bool>>,
    d: &Vec<(i32, i32)>,
    n: i32,
) -> (usize, usize) {
    vis[x][y] = true;
    let mut res = (1, 0);
    for (a, b) in d {
        let xx = x as i32 + a;
        let yy = y as i32 + b;
        if xx >= 0
            && xx < n
            && yy >= 0
            && yy < n
            && !vis[xx as usize][yy as usize]
            && v[x][y] == v[xx as usize][yy as usize]
        {
            let ans = dfs(xx as usize, yy as usize, v, vis, d, n);
            res.0 += ans.0;
			res.1 += ans.1;
        } else if xx < 0 || xx >= n || yy < 0 || yy >= n || v[x][y] != v[xx as usize][yy as usize] {
            res.1 += 1;
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
    let d = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in 0..n {
        for y in 0..n {
            if !vis[x][y] {
                let ans = dfs(x, y, &v, &mut vis, &d, n as i32);
                res += ans.0 * ans.1;
            }
        }
    }
    println!("{res}");
}
