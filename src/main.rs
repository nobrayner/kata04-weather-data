use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = env::args();

    if let Some(filepath) = args.nth(1) {
        let file = File::open(filepath).expect("Unable to read that file!");
        let reader = BufReader::new(file);

        let mut lines_iter = reader.lines().enumerate().skip(2).peekable();

        let mut day_with_lowest_spread: Option<(u8, f32)> = None;

        while let Some((_, line)) = lines_iter.next() {
            if let None = lines_iter.peek() {
                break;
            }

            let data: Vec<_> = (line.unwrap())
                .split_ascii_whitespace()
                .enumerate()
                .take(3)
                .map(|(_, item)| item.to_owned())
                .collect();

            let day_and_spread = (
                data[0].parse::<u8>().unwrap(),
                (data[1].trim_end_matches("*").parse::<f32>().unwrap()
                    - data[2].trim_end_matches("*").parse::<f32>().unwrap())
                .abs(),
            );

            if let Some((_, spread)) = day_with_lowest_spread {
                if day_and_spread.1 < spread {
                    day_with_lowest_spread = Some(day_and_spread);
                }
            } else {
                day_with_lowest_spread = Some(day_and_spread);
            }
        }

        if let Some((day, spread)) = day_with_lowest_spread {
            println!("Day with lowest spread:\nDay: {}\nSpread: {}", day, spread);
        } else {
            println!("No days to compare");
        }
    } else {
        println!("Please provide a filepath");
    }
}
