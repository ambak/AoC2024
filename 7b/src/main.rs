use std::io::{self, BufRead};

fn concatenate(a: i64, b: i64) -> Option<i64> {
    let aa = a.to_string();
    let bb = b.to_string();
    let x = format!("{aa}{bb}");
    let res = x.parse::<i64>();
    match res {
        Ok(val) => Some(val),
        _ => None,
    }
}

fn f(a: i64, res: i64, v: &Vec<i64>, pos: usize) -> bool {
    if res > a {
        return false;
    }
    if pos == v.len() {
        if a == res {
            return true;
        }
        return false;
    }
    if res.checked_mul(v[pos]).is_some() && f(a, res * v[pos], &v, pos + 1) {
        return true;
    }
    if f(a, res + v[pos], &v, pos + 1) {
        return true;
    }
    if let Some(x) = concatenate(res, v[pos]) {
        return f(a, x, &v, pos + 1);
    }
    false
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let mut v = vec![];
        let l = line.unwrap();
        let mut ll = l.split_whitespace();
        let a = ll
            .next()
            .unwrap()
            .split(':')
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        for l in ll {
            let b = l.parse::<i64>().unwrap();
            v.push(b);
        }
        if f(a, 0, &v, 0) {
            res += a;
        }
    }
    println!("{res}");
}
