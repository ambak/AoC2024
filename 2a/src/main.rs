use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.split_whitespace();
        let mut v = vec![];
        for x in line {
            let x = x.parse::<i64>().unwrap();
            v.push(x);
        }
        let mut prev = v[0];
        let mut ok = true;
        for a in v.iter().skip(1) {
            if !(prev > *a && prev < *a + 4) {
                ok = false;
                break;
            }
            prev = *a;
        }
        if ok {
            res += 1;
            continue;
        }
        prev = v[0];
        ok = true;
        for a in v.iter().skip(1) {
            if !(prev < *a && prev + 4 > *a) {
                ok = false;
                break;
            }
            prev = *a;
        }
        if ok {
            res += 1;
            continue;
        }
    }
    println!("{res}");
}
