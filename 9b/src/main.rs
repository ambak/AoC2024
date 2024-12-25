use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut res = 0i64;
    let mut v: Vec<i64> = vec![];
    let s = lines.next().unwrap().unwrap();
    let mut empty = false;
    let mut id = 0i64;
    let mut counter = vec![BTreeSet::new(); 10];
    for c in s.bytes() {
        let cnt = (c - b'0') as usize;
        for _ in 0..cnt {
            if empty {
                v.push(-1);
            } else {
                v.push(id);
            }
        }
        if !empty {
            id += 1;
        } else {
            counter[cnt].insert(v.len() - cnt);
        }
        empty ^= true;
    }
    let mut j = v.len() - 1;
    'outer: loop {
        let mut cnt = 0;
        while v[j] == -1 {
            j -= 1;
        }
        let x = v[j];
        while v[j] == x {
            if j == 0 {
                break 'outer;
            }
            j -= 1;
            cnt += 1;
        }
        let mut id = 0;
        let mut pos = v.len();
        for i in cnt..=9 {
            if let Some(z) = counter[i].first() {
                if *z < pos {
                    pos = *z;
                    id = i;
                }
            }
        }
        counter[id].pop_first();
        if pos > j {
            continue;
        }
        for i in pos..pos + cnt {
            v[i] = x;
        }
        counter[id - cnt].insert(pos + cnt);
        for i in j + 1..j + 1 + cnt {
            v[i] = -1;
        }
    }
    for i in 0..v.len() {
        if v[i] == -1 {
            continue;
        }
        res += v[i] * i as i64;
    }
    println!("{res}");
}
