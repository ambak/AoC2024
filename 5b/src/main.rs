use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut va = vec![BTreeSet::new(); 100];
    let mut vb = vec![BTreeSet::new(); 100];
    let mut first_stage = true;
    for line in lines {
        let l = line.unwrap();
        if l == "" {
            first_stage = false;
            continue;
        }
        if first_stage {
            let mut ll = l.split('|');
            let a = ll.next().unwrap().parse::<usize>().unwrap();
            let b = ll.next().unwrap().parse::<usize>().unwrap();
            va[b].insert(a);
            vb[a].insert(b);
        } else {
            let mut v = vec![];
            let ll = l.split(',');
            for x in ll {
                v.push(x.parse::<usize>().unwrap());
            }
            let mut ok = true;
            'outer: for i in 1..v.len() {
                for j in 0..i {
                    let x = vb[v[i]].get(&v[j]);
                    if x.is_some() {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if !ok {
                while !ok {
                    ok = true;
                    'outer: for i in 1..v.len() {
                        for j in 0..i {
                            let x = vb[v[i]].get(&v[j]);
                            if x.is_some() {
                                ok = false;
                                (v[i], v[j]) = (v[j], v[i]);
                                break 'outer;
                            }
                        }
                    }
                }
                res += v[v.len() / 2];
            }
        }
    }
    println!("{res}");
}
