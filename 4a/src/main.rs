use std::io::{self, BufRead};

fn count_word(w: &Vec<u8>, v: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    for i in 0..=v.len() - w.len() {
        for j in 0..v.len() {
            let mut ok = true;
            for k in 0..w.len() {
                if w[k] != v[i + k][j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
            ok = true;
            for k in 0..w.len() {
                if w[w.len() - k - 1] != v[i + k][j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
            ok = true;
            for k in 0..w.len() {
                if w[k] != v[j][i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
            ok = true;
            for k in 0..w.len() {
                if w[w.len() - k - 1] != v[j][i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
        }
    }

    for i in 0..=v.len() - w.len() {
        for j in 0..=v.len() - w.len() {
            let mut ok = true;
            for k in 0..w.len() {
                if w[k] != v[j + k][i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
            ok = true;
            for k in 0..w.len() {
                if w[w.len() - k - 1] != v[j + k][i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
        }
    }

    for i in w.len() - 1..v.len() {
        for j in 0..=v.len() - w.len() {
            let mut ok = true;
            for k in 0..w.len() {
                if w[k] != v[j + k][i - k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
            ok = true;
            for k in 0..w.len() {
                if w[w.len() - k - 1] != v[j + k][i - k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    let xmas = "XMAS".as_bytes().to_vec();
    res += count_word(&xmas, &v);
    println!("{res}");
}
