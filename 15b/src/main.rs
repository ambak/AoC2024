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
                    for j in 0..v[i].len() {
                        if v[i][j] == b'@' {
                            (x, y) = (i, j);
                        }
                    }
                }
                is_map = false;
                continue;
            }
            let mut w = vec![];
            for c in l {
                if c == b'#' || c == b'.' {
                    w.push(c);
                    w.push(c);
                } else if c == b'O' {
                    w.push(b'[');
                    w.push(b']');
                } else {
                    w.push(c);
                    w.push(b'.');
                }
            }
            v.push(w);
        } else {
            for c in l {
                let m = d[c as usize];
                let (mut i, mut j) = (x, y);
                let mut possible_to_move = false;
                if c == b'<' || c == b'>' {
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
                } else {
                    let mut w = vec![(x, y)];
                    let mut q = vec![];
                    let mut to_move = vec![];
                    to_move.push(w.clone());
                    'outer: loop {
                        for (i, j) in &w {
                            let a = (*i as i32 + m.0) as usize;
                            let b = (*j as i32 + m.1) as usize;
                            if v[a][b] == b'#' {
                                possible_to_move = false;
                                break 'outer;
                            }
                            if v[a][b] == b'[' {
                                q.push((a, b));
                                q.push((a, b + 1));
                            }
                            if v[a][b] == b']' {
                                if let Some(z) = q.last() {
                                    if z.0 == a && z.1 == b {
                                        continue;
                                    }
                                }
                                q.push((a, b - 1));
                                q.push((a, b));
                            }
                        }
                        if q.is_empty() {
                            possible_to_move = true;
                            break;
                        }
                        w.clear();
                        w = q.clone();
                        q.clear();
                        to_move.push(w.clone());
                    }
                    if possible_to_move {
                        to_move.reverse();
                        x = (x as i32 + m.0) as usize;
                        y = (y as i32 + m.1) as usize;
                        for w in to_move {
                            for (i, j) in w {
                                let a = (i as i32 + m.0) as usize;
                                let b = (j as i32 + m.1) as usize;
                                (v[i][j], v[a][b]) = (v[a][b], v[i][j]);
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == b'[' {
                res += 100 * i + j;
            }
        }
    }
    println!("{res}");
}
