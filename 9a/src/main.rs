use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut res = 0i64;
    let mut v = vec![];
    let s = lines.next().unwrap().unwrap();
    let mut empty = false;
    let mut id = 0i64;
    for c in s.bytes() {
        for _ in 0..(c - b'0') {
            if empty {
                v.push(-1);
            } else {
                v.push(id);
            }
        }
        if !empty {
            id += 1;
        }
        empty ^= true;
    }
    let mut j = v.len() - 1;
    for i in 0..v.len() {
        if v[i] == -1 {
            while j > i {
                if v[j] == -1 {
                    j -= 1;
                } else {
                    (v[i], v[j]) = (v[j], v[i]);
                    break;
                }
            }
        }
    }
    for i in 0..v.len() {
        if v[i] == -1 {
            break;
        }
        res += v[i] * i as i64;
    }
    println!("{res}");
}
