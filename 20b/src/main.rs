use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn bfs(v: &Vec<Vec<u8>>) -> i64 {
    let n = v.len();
    let m = v[0].len();
    let inf = 9999999;
    let mut dist: Vec<Vec<i32>> = vec![vec![inf; m]; n];
    let mut d = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
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
    d.clear();
    let max_cheat = 20;
    for i in 1..=max_cheat {
        if i + i <= max_cheat {
            d.push((i, i));
            d.push((-i, i));
            d.push((i, -i));
            d.push((-i, -i));
        }
        if i <= max_cheat {
            d.push((0, i));
            d.push((0, -i));
            d.push((i, 0));
            d.push((-i, 0));
        }
        for j in i + 1..=max_cheat {
            if i + j <= max_cheat {
                d.push((i, j));
                d.push((-i, j));
                d.push((i, -j));
                d.push((-i, -j));

                d.push((j, i));
                d.push((-j, i));
                d.push((j, -i));
                d.push((-j, -i));
            }
        }
    }
    let mut res = 0i64;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if dist[i][j] != inf {
                let mut cheat = vec![];
                for (a, b) in &d {
                    let aa = i as i32 + *a;
                    let bb = j as i32 + *b;
                    if aa > 0 && aa < n as i32 && bb > 0 && bb < m as i32 {
                        x = aa as usize;
                        y = bb as usize;
                        if dist[x][y] != inf && v[x][y] != b'#' {
                            let tmp = dist[x][y] as i32 - dist[i][j] as i32 - a.abs() - b.abs();
                            cheat.push(tmp);
                        }
                    }
                }
                for k in 0..cheat.len() {
                    if cheat[k] >= 100 {
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
