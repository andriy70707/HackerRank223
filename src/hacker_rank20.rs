use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

fn pageCount(n: i32, p: i32) -> i32 {
    // Calculate turns from the front
    let front_turns = p / 2;

    // Calculate turns from the back
    let back_turns = if n % 2 == 0 {
        (n - p + 1) / 2 // if n is even
    } else {
        (n - p) / 2 // if n is odd
    };

    // Return the minimum of the two
    front_turns.min(back_turns)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}