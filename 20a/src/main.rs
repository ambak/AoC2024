use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn bfs(v: &Vec<Vec<u8>>) -> i64 {
    let n = v.len();
    let m = v[0].len();
    let inf = 9999999;
    let mut dist: Vec<Vec<i32>> = vec![vec![inf; m]; n];
    let d = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if v[i][j] == b'S' {
                (x, y) = (i, j);
                break;
            }
        }
    }
    dist[x][y] = 0;
    let mut q = VecDeque::new();
    q.push_back((x, y));
    while !q.is_empty() {
        (x, y) = q.pop_front().unwrap();
        if v[x][y] == b'E' {
            break;
        }
        for (a, b) in &d {
            let a = (x as i32 + *a) as usize;
            let b = (y as i32 + *b) as usize;
            if dist[a][b] > dist[x][y] && v[a][b] != b'#' {
                q.push_back((a, b));
                dist[a][b] = dist[x][y] + 1;
            }
        }
    }
    let mut res = 0i64;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            let mut cheat = vec![];
            for (a, b) in &d {
                let a = (i as i32 + *a) as usize;
                let b = (j as i32 + *b) as usize;
                if dist[a][b] != inf && v[a][b] != b'#' {
                    cheat.push(dist[a][b]);
                }
            }
            for k in 0..cheat.len() {
                for l in k + 1..cheat.len() {
                    if (cheat[k] - cheat[l]).abs() >= 102 {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut v: Vec<Vec<u8>> = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    let res = bfs(&v);
    println!("{res}");
}
