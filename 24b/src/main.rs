#[macro_use]
extern crate scan_fmt;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Gate {
    name1: String,
    op: String,
    name2: String,
    resname: String,
}

fn find_gate_out(gates_list: &Vec<Gate>, resname: &String) -> usize {
    for i in 0..gates_list.len() {
        if gates_list[i].resname == *resname {
            return i;
        }
    }
    999
}

fn find_gate_in(gates_list: &Vec<Gate>, name1: &String, name2: &String, op: &String) -> usize {
    for i in 0..gates_list.len() {
        if gates_list[i].op == *op
            && ((gates_list[i].name1 == *name1 && gates_list[i].name2 == *name2)
                || (gates_list[i].name1 == *name2 && gates_list[i].name2 == *name1))
        {
            return i;
        }
    }
    999
}

fn swap_gates_out(gates_list: &mut Vec<Gate>, a: usize, b: usize, ans: &mut Vec<String>) {
    let tmp = gates_list[a].resname.clone();
    gates_list[a].resname = gates_list[b].resname.clone();
    gates_list[b].resname = tmp;
    ans.push(gates_list[a].resname.clone());
    ans.push(gates_list[b].resname.clone());
}

fn prepare_names(a: usize) -> (String, String, String) {
    let mut z = a.to_string();
    let zero = "0".to_string().to_owned();
    if z.len() == 1 {
        z = zero + &z;
    }
    let x = "x".to_string().to_owned() + &z;
    let y = "y".to_string().to_owned() + &z;
    let z = "z".to_string().to_owned() + &z;
    (x, y, z)
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut gates_list: Vec<Gate> = vec![];
    loop {
        let l = lines.next().unwrap().unwrap();
        if l == "" {
            break;
        }
    }
    for line in lines {
        let l = line.unwrap();
        if let Ok((name1, op, name2, resname)) =
            scan_fmt!(&l, "{} {} {} -> {}", String, String, String, String)
        {
            let newgate = Gate {
                name1: name1.clone(),
                op: op.clone(),
                name2: name2.clone(),
                resname: resname.clone(),
            };
            gates_list.push(newgate);
        }
    }

    let xor = "XOR".to_string();
    let or = "OR".to_string();
    let and = "AND".to_string();
    let mut ans = vec![];
    let a = find_gate_in(&gates_list, &"x00".to_string(), &"y00".to_string(), &xor);
    let b = find_gate_out(&gates_list, &"z00".to_string());
    if a != b {
        swap_gates_out(&mut gates_list, a, b, &mut ans);
    }

    let mut prev_or = find_gate_in(&gates_list, &"x00".to_string(), &"y00".to_string(), &and);
    for i in 1..=44 {
        let (name1, name2, resname) = prepare_names(i);
        let a = find_gate_in(&gates_list, &name1, &name2, &xor);
        let b = find_gate_in(&gates_list, &name1, &name2, &and);
        let mut c = find_gate_out(&gates_list, &resname);
        let mut d = find_gate_in(
            &gates_list,
            &gates_list[prev_or].resname,
            &gates_list[a].resname,
            &xor,
        );
        if d == 999 {
            let n1 = gates_list[c].name1.clone();
            let n2 = gates_list[c].name2.clone();
            let pr = gates_list[prev_or].resname.clone();
            let ar = gates_list[a].resname.clone();
            if n1 == pr || n2 == pr {
                let mut nn = n1;
                if nn == pr {
                    nn = n2;
                }
                let tmp = find_gate_out(&gates_list, &nn);
                swap_gates_out(&mut gates_list, a, tmp, &mut ans);
            } else {
                let mut nn = n1;
                if nn == ar {
                    nn = n2;
                }
                let tmp = find_gate_out(&gates_list, &nn);
                swap_gates_out(&mut gates_list, prev_or, tmp, &mut ans);
            }
            c = find_gate_out(&gates_list, &resname);
            d = find_gate_in(
                &gates_list,
                &gates_list[prev_or].resname,
                &gates_list[a].resname,
                &xor,
            );
        }
        if c != d {
            swap_gates_out(&mut gates_list, c, d, &mut ans);
        }

        let e = find_gate_in(
            &gates_list,
            &gates_list[prev_or].resname,
            &gates_list[a].resname,
            &and,
        );
        if e == 999 {
            panic!("e {e}");
        }
        let f = find_gate_in(
            &gates_list,
            &gates_list[e].resname,
            &gates_list[b].resname,
            &or,
        );
        if f == 999 {
            panic!("f {f}");
        }
        prev_or = f;
    }

    ans.sort();
    for i in 0..ans.len() - 1 {
        print!("{},", ans[i]);
    }
    println!("{}", ans[ans.len() - 1]);
}
