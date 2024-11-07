use std::collections::HashMap;
use std::io::{self, BufRead};

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird_count = HashMap::new();

    for &bird in arr {
        *bird_count.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut min_bird_id = i32::MAX;

    for (&bird_id, &count) in &bird_count {
        if count > max_count || (count == max_count && bird_id < min_bird_id) {
            max_count = count;
            min_bird_id = bird_id;
        }
    }

    min_bird_id
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    // Print result to consol
    println!("{}", result);
}