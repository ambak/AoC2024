#[macro_use]
extern crate scan_fmt;
use std::io::{self, BufRead};

fn prepare_grid(
    v: &Vec<(i64, i64, i64, i64)>,
    grid: &mut Vec<Vec<char>>,
    t: i64,
    n: i64,
    m: i64,
) -> bool {
    for i in 0..m as usize {
        for j in 0..n as usize {
            grid[i][j] = '.';
        }
    }
    for (xp, yp, xv, yv) in v {
        let x = (((xp + t * xv) % n + n) % n) as usize;
        let y = (((yp + t * yv) % m + m) % m) as usize;
        grid[y][x] = '*';
    }
    let mut display = false;
    for i in 0..(m - 5) as usize {
        for j in 0..(n - 5) as usize {
            let mut stars = 0;
            for x in 0..5 {
                for y in 0..5 {
                    if grid[i + y][j + x] == '*' {
                        stars += 1;
                    }
                }
            }
            if stars > 20 {
                display = true;
            }
        }
    }
    if display {
        println!("answer = {}", t);
        for i in 0..m as usize {
            for j in 0..n as usize {
                print!("{}", grid[i][j]);
            }
            println!();
        }
        return true;
    }
    false
}

fn main() {
    let lines = io::stdin().lock().lines();
    let n = 101;
    let m = 103;
    let mut v = vec![];
    for line in lines {
        let l = line.unwrap();
        if let Ok((xp, yp, xv, yv)) = scan_fmt!(&l, "p={},{} v={},{}", i64, i64, i64, i64) {
            v.push((xp, yp, xv, yv));
        }
    }
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; n as usize]; m as usize];
    for t in 0.. {
        if prepare_grid(&v, &mut grid, t, n, m) {
            break;
        }
    }
}
