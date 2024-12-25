use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, BufRead},
};

fn dijkstra(v: &Vec<Vec<i32>>) -> i64 {
    let mut res = vec![vec![99999999999999999; v.len()]; v.len()];
    let (x, y) = (0, 0);
    res[x][y] = 0;
    let d = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut q: BTreeSet<(i64, usize, usize)> = BTreeSet::new();
    q.insert((0, x, y));

    while !q.is_empty() {
        let (val, x, y) = q.pop_first().unwrap();
        for i in 0..4 {
            let cost = 1;
            if x as i32 + d[i].0 < 0 || y as i32 + d[i].1 < 0 {
                continue;
            }
            let a = (x as i32 + d[i].0) as usize;
            let b = (y as i32 + d[i].1) as usize;
            if a < v.len() && b < v.len() && v[a][b] != 1 && res[a][b] > val + cost {
                res[a][b] = min(val + cost, res[a][b]);
                q.insert((val + cost, a, b));
            }
        }
    }

    res[v.len() - 1][v.len() - 1]
}

fn main() {
    let lines = io::stdin().lock().lines();
    let res;
    let n = 70;
    let limit = 1024;
    let mut v: Vec<Vec<i32>> = vec![vec![0; n + 1]; n + 1];
    let mut cnt = 0;
    for line in lines {
        let l = line.unwrap();
        let mut l = l.split(',');
        let x = l.next().unwrap().parse::<usize>().unwrap();
        let y = l.next().unwrap().parse::<usize>().unwrap();
        v[x][y] = 1;
        cnt += 1;
        if cnt == limit {
            break;
        }
    }
    res = dijkstra(&v);
    println!("{res}");
}
