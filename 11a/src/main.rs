use std::io::{self, BufRead};

fn f(nr: i32, v: &mut Vec<i128>) {
    let mut nr = nr;
    while nr > 0 {
        let mut w = vec![];
        for x in v.into_iter() {
            if *x == 0 {
                w.push(1i128);
            } else {
                let y = x.to_string().as_bytes().to_vec();
                if y.len() % 2 == 0 {
                    let a = String::from_utf8(y[..y.len() / 2].to_vec())
                        .unwrap()
                        .parse::<i128>()
                        .unwrap();
                    let b = String::from_utf8(y[y.len() / 2..].to_vec())
                        .unwrap()
                        .parse::<i128>()
                        .unwrap();
                    w.push(a);
                    w.push(b);
                } else {
                    w.push(*x * 2024);
                }
            }
        }
        v.clear();
        let _ = v.extend(w);
        nr -= 1;
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let l = line.unwrap();
        for x in l.split_whitespace() {
            let mut v = vec![];
            let x = x.parse::<i128>().unwrap();
            v.push(x);
            f(25, &mut v);
            res += v.len();
        }
    }
    println!("{res}");
}
