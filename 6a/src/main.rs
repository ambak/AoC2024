use std::io::{self, BufRead};

fn bfs(v: &mut Vec<Vec<u8>>) -> i32 {
    let d = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut res = 1;
    let mut q = (0, 0);
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == b'^' {
                q = (i as i32, j as i32);
                break;
            }
        }
    }
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
            if v[x as usize][y as usize] == b'.' {
                res += 1;
                v[x as usize][y as usize] = b'!';
            }
            q.0 = x;
            q.1 = y;
        } else {
            break;
        }
    }
    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let res;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap();
        v.push(l.as_bytes().to_vec());
    }
    res = bfs(&mut v);
    println!("{res}");
}
