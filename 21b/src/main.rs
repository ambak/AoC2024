use std::{
    cmp::min,
    collections::BTreeSet,
    io::{self, BufRead},
};

fn find_pos(k: &Vec<Vec<u8>>, c: &u8) -> (usize, usize) {
    for i in 0..k.len() {
        for j in 0..k[i].len() {
            if k[i][j] == *c {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn human_press(to_press: &mut Vec<Vec<i64>>, a: &Vec<u8>, pos: &Vec<(usize, usize)>) {
    for c1 in a {
        for c2 in a {
            let (x1, y1) = pos[*c1 as usize];
            let (x2, y2) = pos[*c2 as usize];
            let dist = (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs() + 1;
            to_press[*c1 as usize][*c2 as usize] = dist;
        }
    }
}

fn keyboard_press(
    k2: &Vec<Vec<u8>>,
    a1: &Vec<u8>,
    a2: &Vec<u8>,
    to_press: &mut Vec<Vec<i64>>,
    cost: &Vec<Vec<i64>>,
    pos2: &Vec<(usize, usize)>,
) {
    let d = vec![
        (1, 0, b'v' as usize),
        (-1, 0, b'^' as usize),
        (0, 1, b'>' as usize),
        (0, -1, b'<' as usize),
    ];
    for c in a2 {
        let mut v = vec![vec![999999999999999i64; 120]; 120]; //cost to letter i using j as last direction
        let cc = *c as usize;
        v[cc][b'A' as usize] = 0;
        to_press[cc][cc] = 1;
        let mut q = BTreeSet::new();
        q.insert((0, *c as usize, b'A' as usize));
        while !q.is_empty() {
            let (val, lit, last) = q.pop_first().unwrap();
            let (x1, y1) = pos2[lit];
            for (a, b, direction) in &d {
                let x2 = (x1 as i64 + *a) as usize;
                let y2 = (y1 as i64 + *b) as usize;
                let letter = k2[x2][y2] as usize;
                if letter != b'#' as usize {
                    let price = val + cost[last][*direction];
                    if v[letter][*direction] > price {
                        v[letter][*direction] = price;
                        q.insert((price, letter, *direction));
                    }
                }
            }
        }
        for p in a2 {
            let pp = *p as usize;
            for o in a1 {
                let oo = *o as usize;
                to_press[cc][pp] = min(to_press[cc][pp], v[pp][oo] + cost[oo][b'A' as usize]);
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut res = 0;
    let keyboard1 = "##### #789# #456# #123# ##0A# #####";
    let keyboard2 = "##### ##^A# #<v># #####";
    let available1 = "7894561230A";
    let available2 = "^A<v>";
    let mut k1 = vec![];
    let mut k2 = vec![];
    for key in keyboard1.split_whitespace() {
        k1.push(key.as_bytes().to_vec());
    }
    for key in keyboard2.split_whitespace() {
        k2.push(key.as_bytes().to_vec());
    }
    let mut pos1: Vec<(usize, usize)> = vec![(0, 0); 120];
    let mut pos2: Vec<(usize, usize)> = vec![(0, 0); 120];
    let a1 = available1.as_bytes().to_vec();
    let a2 = available2.as_bytes().to_vec();
    for c in &a1 {
        pos1[*c as usize] = find_pos(&k1, c);
    }
    for c in &a2 {
        pos2[*c as usize] = find_pos(&k2, c);
    }
    let mut h_k2_press = vec![vec![999999999999999i64; 120]; 120];
    human_press(&mut h_k2_press, &a2, &pos2);
    for _ in 0..24 {
        let cost = h_k2_press.clone();
        let mut to_press = vec![vec![999999999999999i64; 120]; 120];
        keyboard_press(&k2, &a2, &a2, &mut to_press, &cost, &pos2);
        h_k2_press = to_press.clone();
    }
    let mut k2_k2_press2 = vec![vec![999999999999999i64; 120]; 120];
    keyboard_press(&k1, &a2, &a1, &mut k2_k2_press2, &h_k2_press, &pos1);

    for line in lines {
        let v = line.unwrap().as_bytes().to_vec();
        let mut last = b'A' as usize;
        let mut ans = 0;
        for c in &v {
            let cc = *c as usize;
            ans += k2_k2_press2[last][cc];
            last = cc;
        }
        let mut val = 0;
        for c in &v {
            val *= 10;
            let cc = *c as i64 - b'0' as i64;
            if cc <= 9 {
                val += cc;
            }
        }
        val /= 10;
        res += ans * val;
    }
    println!("{res}");
}
