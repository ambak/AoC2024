use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn dfs(st: (usize, usize), v: &Vec<Vec<u8>>, hs: &mut HashSet<(usize, usize)>, val: u8) -> i32 {
    let n = v.len();
    let mut res = 0;
    hs.insert(st);
    if val == b'9' {
        return 1;
    }
    if st.0 > 0 && !hs.contains(&(st.0 - 1, st.1)) && v[st.0 - 1][st.1] == val + 1 {
        res += dfs((st.0 - 1, st.1), v, hs, val + 1);
    }
    if st.1 > 0 && !hs.contains(&(st.0, st.1 - 1)) && v[st.0][st.1 - 1] == val + 1 {
        res += dfs((st.0, st.1 - 1), v, hs, val + 1);
    }
    if st.0 < n - 1 && !hs.contains(&(st.0 + 1, st.1)) && v[st.0 + 1][st.1] == val + 1 {
        res += dfs((st.0 + 1, st.1), v, hs, val + 1);
    }
    if st.1 < n - 1 && !hs.contains(&(st.0, st.1 + 1)) && v[st.0][st.1 + 1] == val + 1 {
        res += dfs((st.0, st.1 + 1), v, hs, val + 1);
    }
    return res;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    let n = v.len();
    for i in 0..n {
        for j in 0..n {
            if v[i][j] == b'0' {
                let mut hs = HashSet::new();
                res += dfs((i, j), &v, &mut hs, b'0');
            }
        }
    }
    println!("{res}");
}
