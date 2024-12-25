#[macro_use]
extern crate scan_fmt;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
struct Gate {
    name1: String,
    op: String,
    name2: String,
    resname: String,
    cnt: i32,
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut gates: HashMap<String, Vec<usize>> = HashMap::new();
    let mut hm = HashMap::new();
    let mut ready = vec![];
    let mut gates_list: Vec<Gate> = vec![];
    loop {
        let l = lines.next().unwrap().unwrap();
        if l == "" {
            break;
        }
        if let Ok((name, val)) = scan_fmt!(&l, "{}: {}", String, i64) {
            hm.insert(name, val);
        }
    }
    for line in lines {
        let l = line.unwrap();
        if let Ok((name1, op, name2, resname)) =
            scan_fmt!(&l, "{} {} {} -> {}", String, String, String, String)
        {
            let n1 = hm.contains_key(&name1);
            let n2 = hm.contains_key(&name2);
            let mut cnt = 0;
            if n1 && n2 {
                cnt = 2;
            } else if n1 || n2 {
                cnt = 1;
            }
            if cnt == 2 {
                ready.push(Gate {
                    name1,
                    op,
                    name2,
                    resname,
                    cnt,
                });
            } else {
                let newgate = Gate {
                    name1: name1.clone(),
                    op: op.clone(),
                    name2: name2.clone(),
                    resname: resname.clone(),
                    cnt,
                };
                gates_list.push(newgate);
                let g1 = gates.get_mut(&name1);
                if g1.is_some() {
                    let g = g1.unwrap();
                    g.push(gates_list.len() - 1);
                } else {
                    gates.insert(name1.clone(), vec![gates_list.len() - 1]);
                }
                let g2 = gates.get_mut(&name2);
                if g2.is_some() {
                    let g = g2.unwrap();
                    g.push(gates_list.len() - 1);
                } else {
                    gates.insert(name2.clone(), vec![gates_list.len() - 1]);
                }
            }
        }
    }

    while !ready.is_empty() {
        let g = ready.pop().unwrap();
        let val1 = hm.get(&g.name1).unwrap();
        let val2 = hm.get(&g.name2).unwrap();
        let val;
        if g.op == "AND" {
            val = val1 & val2;
        } else if g.op == "OR" {
            val = val1 | val2;
        } else {
            val = val1 ^ val2;
        }
        hm.insert(g.resname.clone(), val);
        let g1 = gates.get_mut(&g.resname);
        if g1.is_none() {
            continue;
        }
        let g1 = g1.unwrap();
        for x in g1 {
            gates_list[*x].cnt += 1;
            if gates_list[*x].cnt == 2 {
                ready.push(gates_list[*x].clone());
            }
        }
    }

    let mut z = vec![];
    for x in hm {
        if x.0.starts_with('z') {
            z.push(x);
        }
    }
    z.sort();
    let mut res = 0i64;
    for (i, (_, val)) in z.into_iter().enumerate() {
        res |= val << i;
    }
    println!("{res}");
}
