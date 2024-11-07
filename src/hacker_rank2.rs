use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(output_path).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = compare_triplets(&a, &b);

    writeln!(&mut fptr, "{} {}", result[0], result[1]).unwrap();
}

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut scores = vec![0, 0];
    for i in 0..a.len() {
        if a[i] > b[i] {
            scores[0] += 1;
        } else if a[i] < b[i] {
            scores[1] += 1;
        }
    }
    scores
}