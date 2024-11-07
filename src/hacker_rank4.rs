use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let row = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        arr.push(row);
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}