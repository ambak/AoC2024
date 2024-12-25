use std::io::{self, BufRead};

fn count_word(v: &Vec<Vec<u8>>) -> i32 {
    let mut res = 0;
    for i in 1..v.len() - 1 {
        for j in 1..v.len() - 1 {
            let mut cnt = 0;
            if v[i][j] == b'A' {
                if (v[i + 1][j + 1] == b'M' && v[i - 1][j - 1] == b'S')
                    || (v[i + 1][j + 1] == b'S' && v[i - 1][j - 1] == b'M')
                {
                    cnt += 1;
                }
                if (v[i - 1][j + 1] == b'M' && v[i + 1][j - 1] == b'S')
                    || (v[i - 1][j + 1] == b'S' && v[i + 1][j - 1] == b'M')
                {
                    cnt += 1;
                }
                if cnt == 2 {
                    res += 1;
                }
            }
        }
    }
    res
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap().as_bytes().to_vec();
        v.push(l);
    }
    res += count_word(&v);
    println!("{res}");
}
