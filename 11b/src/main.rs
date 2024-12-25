use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn f(nr: usize, x: i128, hm: &mut HashMap<i128, (i128, i128)>) {
    if hm.get_key_value(&x).is_some() {
        return;
    }
    if x == 0 {
        hm.insert(x, (1, -1));
        f(nr + 1, 1, hm);
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
            hm.insert(x, (a, b));
            f(nr + 1, a, hm);
            f(nr + 1, b, hm);
        } else {
            hm.insert(x, (x * 2024, -1));
            f(nr + 1, x * 2024, hm);
        }
    }
}

fn g(
    nr: usize,
    x: i128,
    hm: &HashMap<i128, (i128, i128)>,
    hv: &mut HashMap<(i128, usize), i64>,
) -> i64 {
    let mut res = 0;
    if nr == 0 {
        return 1;
    }
    let (a, b) = hm.get(&x).unwrap();
    if let Some(y) = hv.get(&(*a, nr - 1)) {
        res += *y;
    } else {
        let ans = g(nr - 1, *a, hm, hv);
        res += ans;
    }
    if *b != -1 {
        if let Some(y) = hv.get(&(*b, nr - 1)) {
            res += *y;
        } else {
            let ans = g(nr - 1, *b, hm, hv);
            res += ans;
        }
    }
    hv.insert((x, nr), res);
    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let l = line.unwrap();
        for x in l.split_whitespace() {
            let x = x.parse::<i128>().unwrap();
            let mut hm = HashMap::new();
            let mut hv = HashMap::new();
            let nr = 75;
            f(0, x, &mut hm);
            res += g(nr, x, &hm, &mut hv);
        }
    }
    println!("{res}");
}
