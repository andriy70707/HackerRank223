use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // Get hour, minutes and AM/PM
    let period = &s[8..];
    let hour: i32 = s[..2].parse().unwrap();

    let converted_hour = if period == "AM" {
        if hour == 12 {
            "00".to_string() // 12 AM becomes 00
        } else {
            s[..2].to_string() // Other AM hours remain the same
        }
    } else {
        if hour == 12 {
            s[..2].to_string() // 12 PM remains 12
        } else {
            (hour + 12).to_string() // PM hours are converted to 24-hour format
        }
    };

    // Form the result in HH:MM:SS format
    format!("{}{}", converted_hour, &s[2..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}