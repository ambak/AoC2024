use std::io::{self, BufRead};

fn check_pattern(i: usize, l: &[u8], p: &[u8]) -> bool {
    if i + p.len() > l.len() {
        return false;
    }
    for j in i..i + p.len() {
        if l[j] != p[j - i] {
            return false;
        }
    }
    true
}

fn check_number(i: usize, l: &[u8]) -> Option<(i32, usize)> {
    let mut res = 0;
    let mut ok = false;
    let mut j = 0;
    while j < 3 {
        if l[j + i] >= b'0' && l[j + i] <= b'9' {
            res *= 10;
            res += (l[j + i] - b'0') as i32;
            ok = true;
        } else {
            break;
        }
        j += 1;
    }
    if ok {
        return Some((res, j));
    }
    return None;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let smul = "mul(".to_string();
    let pmul = smul.as_bytes();
    let scomma = ",".to_string();
    let pcomma = scomma.as_bytes();
    let sparenthesis = ")".to_string();
    let pparenthesis = sparenthesis.as_bytes();
    for line in lines {
        let l = line.unwrap();
        let l = l.as_bytes();
        let mut i = 0;
        while i < l.len() {
            if check_pattern(i, l, pmul) {
                i += pmul.len();
                let a;
                let b;
                let val = check_number(i, l);
                if let Some((x, j)) = val {
                    a = x;
                    i += j;
                } else {
                    continue;
                }
                if check_pattern(i, l, pcomma) {
                    i += pcomma.len();
                } else {
                    continue;
                }
                let val = check_number(i, l);
                if let Some((x, j)) = val {
                    b = x;
                    i += j;
                } else {
                    continue;
                }
                if check_pattern(i, l, pparenthesis) {
                    i += pparenthesis.len();
                } else {
                    continue;
                }
                res += a * b;
            } else {
                i += 1;
            }
        }
    }
    println!("{res}");
}
