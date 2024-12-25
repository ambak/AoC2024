use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut va = vec![];
    let mut vb = vec![];
    for line in lines {
        let l = line.unwrap();
        let mut ll = l.split_whitespace();
        let a = ll.next().unwrap().parse::<i64>().unwrap();
        let b = ll.next().unwrap().parse::<i64>().unwrap();
        va.push(a);
        vb.push(b);
    }
    va.sort();
    vb.sort();
    for i in 0..va.len() {
        res += (va[i] - vb[i]).abs();
    }
    println!("{res}");
}
