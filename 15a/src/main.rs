use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v: Vec<Vec<u8>> = vec![];
    let mut is_map = true;
    let mut d = vec![(0, 0); 200];
    d[b'^' as usize] = (-1, 0);
    d[b'v' as usize] = (1, 0);
    d[b'>' as usize] = (0, 1);
    d[b'<' as usize] = (0, -1);
    let (mut x, mut y) = (0, 0);
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        if is_map {
            if l.is_empty() {
                for i in 0..v.len() {
                    for j in 0..v.len() {
                        if v[i][j] == b'@' {
                            (x, y) = (i, j);
                        }
                    }
                }
                is_map = false;
                continue;
            }
            v.push(l);
        } else {
            for c in l {
                let m = d[c as usize];
                let (mut i, mut j) = (x, y);
                let mut possible_to_move = false;
                loop {
                    if v[i][j] == b'#' {
                        break;
                    }
                    if v[i][j] == b'.' {
                        possible_to_move = true;
                        break;
                    }
                    i = (i as i32 + m.0) as usize;
                    j = (j as i32 + m.1) as usize;
                }
                if possible_to_move {
                    loop {
                        let a = (i as i32 - m.0) as usize;
                        let b = (j as i32 - m.1) as usize;
                        (v[i][j], v[a][b]) = (v[a][b], v[i][j]);
                        if v[i][j] == b'@' {
                            (x, y) = (i, j);
                            break;
                        }
                        (i, j) = (a, b);
                    }
                }
            }
        }
    }
    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i][j] == b'O' {
                res += 100 * i + j;
            }
        }
    }
    println!("{res}");
}
