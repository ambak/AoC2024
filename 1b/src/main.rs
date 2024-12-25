use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut va = vec![];
    let mut mb: BTreeMap<i64, i64> = BTreeMap::new();
    for line in lines {
        let l = line.unwrap();
        let mut ll = l.split_whitespace();
        let a = ll.next().unwrap().parse::<i64>().unwrap();
        let b = ll.next().unwrap().parse::<i64>().unwrap();
        va.push(a);
        mb.entry(b)
            .and_modify(|val| {
                *val += 1;
            })
            .or_insert(1);
    }
    for i in 0..va.len() {
        let x = mb.entry(va[i]).or_default();
        res += va[i] * *x;
    }
    println!("{res}");
}
