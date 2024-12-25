use std::io::{self, BufRead};
use trie_rs::TrieBuilder;

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut res = 0;
    let line = lines.next().unwrap().unwrap();
    let l = line.split(", ");
    let mut builder = TrieBuilder::new();
    for x in l {
        builder.push(x);
    }
    let trie = builder.build();
    let _ = lines.next();
    for line in lines {
        let v = line.unwrap().as_bytes().to_vec();
        let mut dp: Vec<i64> = vec![0; v.len() + 1];
        dp[0] = 1;
        for j in 0..v.len() {
            if dp[j] != 0 {
                let mut inc_search = trie.inc_search();
                for i in 0.. {
                    if i + j >= v.len() {
                        break;
                    }
                    let is_match = inc_search.query(&v[i + j]);
                    if is_match.is_none() {
                        break;
                    }
                    if is_match.unwrap().is_match() {
                        dp[i + j + 1] += dp[j];
                    }
                }
            }
        }
        res += dp[v.len()];
    }
    println!("{res}");
}
