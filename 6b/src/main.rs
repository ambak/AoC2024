use std::io::{self, BufRead};

fn bfs(q: (i32, i32), v: &mut Vec<Vec<u8>>) -> bool {
    let d = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut q = q;
    let n = v.len() as i32;
    let m = v[0].len() as i32;
    let mut dd = 0;
    loop {
        let x = q.0 + d[dd].0;
        let y = q.1 + d[dd].1;
        if x >= 0 && x < n && y >= 0 && y < m {
            if v[x as usize][y as usize] == b'#' {
                dd = (dd + 1) % 4;
                continue;
            }
            if v[x as usize][y as usize] == dd as u8 {
                return true;
            }
            v[x as usize][y as usize] = dd as u8;
            q.0 = x;
            q.1 = y;
        } else {
            break;
        }
    }
    false
}

fn clean_input(v: &mut Vec<Vec<u8>>) {
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] != b'#' {
                v[i][j] = b'.';
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap();
        v.push(l.as_bytes().to_vec());
    }
    let mut q = (0, 0);
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == b'^' {
                v[i][j] = b'.';
                q = (i as i32, j as i32);
                break;
            }
        }
    }
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == b'.' {
                v[i][j] = b'#';
                if bfs(q.clone(), &mut v) {
                    res += 1;
                }
                clean_input(&mut v);
                v[i][j] = b'.';
            }
        }
    }
    println!("{res}");
}
