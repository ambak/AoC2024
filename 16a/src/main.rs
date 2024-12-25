use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, BufRead},
};

fn dijkstra(v: &Vec<Vec<u8>>) -> i64 {
    let mut res = vec![vec![99999999999999999; v.len()]; v.len()];
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
            if v[a][b] != b'#' && res[a][b] + 999 > val + cost {
                res[a][b] = min(val + cost, res[a][b]);
                q.insert((val + cost, a, b, i));
            }
        }
    }

    res[1][v.len() - 2]
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
