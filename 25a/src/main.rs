use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut start = true;
    let mut keys = vec![];
    let mut locks = vec![];
    let mut is_lock = false;
    let mut w = vec![0; 5];
    for line in lines {
        let l = line.unwrap();
        let v = l.as_bytes().to_vec();
        if l == "" {
            start = true;
            if is_lock {
                locks.push(w);
            } else {
                keys.push(w);
            }
            w = vec![0; 5];
            continue;
        }
        if start {
            start = false;
            if l == "#####" {
                is_lock = true;
            } else {
                is_lock = false;
                w = vec![-1; 5];
            }
            continue;
        }
        for i in 0..v.len() {
            if v[i] == b'#' {
                w[i] += 1;
            }
        }
    }
    if start == false {
        if is_lock {
            locks.push(w);
        } else {
            keys.push(w);
        }
    }

    for l in &locks {
        for k in &keys {
            let mut ok = true;
            for i in 0..k.len() {
                if l[i] + k[i] > 5 {
                    ok = false;
                }
            }
            if ok {
                res += 1;
            }
        }
    }
    println!("{res}");
}
