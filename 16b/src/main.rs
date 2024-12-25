use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, BufRead},
};

fn dijkstra(v: &Vec<Vec<u8>>) -> i64 {
    let mut res = vec![vec![99999999; v.len()]; v.len()];
    let mut dres = vec![vec![vec![99999999; 4]; v.len()]; v.len()];
    let mut vis = vec![vec![false; v.len()]; v.len()];
    let (x, y) = (v.len() - 2, 1);
    res[x][y] = 0;
    let d = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut q: BTreeSet<(i64, usize, usize, usize)> = BTreeSet::new();
    q.insert((0, x, y, 1));

    while !q.is_empty() {
        let (val, x, y, id) = q.pop_first().unwrap();
        for i in 0..4 {
            let mut cost = 1001;
            if i == id {
                cost = 1;
            }
            let a = (x as i32 + d[i].0) as usize;
            let b = (y as i32 + d[i].1) as usize;
            if v[a][b] != b'#' && res[a][b] + 1000 >= val + cost {
                res[a][b] = min(val + cost, res[a][b]);
                q.insert((val + cost, a, b, i));
                dres[a][b][i] = min(val + cost, dres[a][b][i]);
            }
        }
    }
    let mut paths = 2;

    q.insert((res[1][v.len() - 2], 1, v.len() - 2, 1));
    q.insert((res[1][v.len() - 2], 1, v.len() - 2, 2));

    while !q.is_empty() {
        let (val, x, y, id) = q.pop_first().unwrap();
        let a = (x as i32 - d[id].0) as usize;
        let b = (y as i32 - d[id].1) as usize;
        for i in 0..4 {
            let mut cost = 1001;
            if id == i {
                cost = 1;
            }
            if dres[a][b][i] == val - cost {
                q.insert((val - cost, a, b, i));
                vis[a][b] = true;
            }
        }
    }
    for i in 0..vis.len() {
        for j in 0..vis.len() {
            if vis[i][j] {
                paths += 1;
            }
        }
    }
    paths
}

fn main() {
    let lines = io::stdin().lock().lines();
    let res;
    let mut v: Vec<Vec<u8>> = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    res = dijkstra(&v);
    println!("{res}");
}
