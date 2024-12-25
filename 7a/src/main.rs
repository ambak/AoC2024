use std::io::{self, BufRead};

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
    f(a, res + v[pos], &v, pos + 1)
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
