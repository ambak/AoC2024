use std::io::{self, BufRead};

fn check(v: &Vec<i64>, desc: bool, remove_first: bool) -> bool {
    let mut prev = v[1];
    let mut prevprev = v[0];
    let mut ok = true;
    let mut used = false;
    if (desc && !(prevprev > prev && prevprev < prev + 4))
        || (!desc && !(prevprev < prev && prevprev + 4 > prev))
    {
        used = true;
        if !remove_first {
            prev = prevprev;
        }
    }
    for a in v.iter().skip(2) {
        if (desc && !(prev > *a && prev < *a + 4)) || (!desc && !(prev < *a && prev + 4 > *a)) {
            if !used {
                used = true;
                if !remove_first {
                    if (desc && !(prevprev > *a && prevprev < *a + 4))
                        || (!desc && !(prevprev < *a && prevprev + 4 > *a))
                    {
                        ok = false;
                        break;
                    }
                } else {
                    continue;
                }
            } else {
                ok = false;
                break;
            }
        }
        prevprev = prev;
        prev = *a;
    }
    ok
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.split_whitespace();
        let mut v = vec![];
        for x in line {
            let x = x.parse::<i64>().unwrap();
            v.push(x);
        }

        if check(&v, true, true)
            || check(&v, true, false)
            || check(&v, false, true)
            || check(&v, false, false)
        {
            res += 1;
        }
    }
    println!("{res}");
}
