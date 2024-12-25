use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    let n = v.len();
    let m = v[0].len();
    let mut odw = vec![vec![0; m]; n];
    let mut map: BTreeMap<u8, Vec<(usize, usize)>> = BTreeMap::new();
    for i in 0..n {
        for j in 0..m {
            if v[i][j] != b'.' {
                let x = map.get_mut(&v[i][j]);
                if let Some(w) = x {
                    w.push((i, j));
                } else {
                    map.insert(v[i][j], vec![(i, j)]);
                }
            }
        }
    }
    for (_, w) in map {
        for i in 0..w.len() {
            for j in i + 1..w.len() {
                let x1 = (w[i].0 * 2) as i32 - w[j].0 as i32;
                let y1 = (w[i].1 * 2) as i32 - w[j].1 as i32;
                let x2 = (w[j].0 * 2) as i32 - w[i].0 as i32;
                let y2 = (w[j].1 * 2) as i32 - w[i].1 as i32;
                if x1 >= 0 && x1 < n as i32 && y1 >= 0 && y1 < m as i32 {
                    odw[x1 as usize][y1 as usize] = 1;
                }
                if x2 >= 0 && x2 < n as i32 && y2 >= 0 && y2 < m as i32 {
                    odw[x2 as usize][y2 as usize] = 1;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            res += odw[i][j];
        }
    }
    println!("{res}");
}
