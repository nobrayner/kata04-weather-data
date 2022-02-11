use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct DayAndTemp {
    day: u8,
    min_temp: f32,
    max_temp: f32,
}

#[derive(Debug)]
struct DayAndSpread {
    day: u8,
    spread: f32,
}

fn main() {
    let mut args = env::args();

    if let Some(filepath) = args.nth(1) {
        let file = File::open(filepath).expect("Unable to read that file!");
        let reader = BufReader::new(file);

        let all_days = get_weather_parsed_data(
            reader
                .lines()
                .enumerate()
                // We don't need the first two lines of metadata
                .skip(2)
                .map(|(_, line)| line.unwrap())
                .collect(),
        );

        let all_days: Vec<_> = all_days
            .iter()
            .map(|i| DayAndSpread {
                day: i.day,
                spread: (i.max_temp - i.min_temp),
            })
            .collect();

        // Don't bother handling edge cases as we have guaranteed data
        let mut day_with_lowest_spread = &all_days[0];

        for day in &all_days {
            if day.spread < day_with_lowest_spread.spread {
                day_with_lowest_spread = day;
            }
        }

        println!(
            "Day with lowest spread:\nDay: {}\nSpread: {}",
            day_with_lowest_spread.day, day_with_lowest_spread.spread
        );
    } else {
        println!("Please provide a filepath");
    }
}

fn get_weather_parsed_data(lines: Vec<String>) -> Vec<DayAndTemp> {
    let mut all_days = vec![];

    // Reverse so as to drop the last line - which is averages that we don't want
    for line in lines.iter().rev().skip(1) {
        let sanitized_data_line: Vec<_> =
            line.split_ascii_whitespace().enumerate().take(3).collect();
        let sanitized_data: Vec<_> = sanitized_data_line
            .iter()
            .map(|(_, item)| item.trim_end_matches("*").to_owned())
            .collect();

        all_days.push(DayAndTemp {
            day: sanitized_data[0].parse::<u8>().unwrap(),
            min_temp: sanitized_data[2].parse::<f32>().unwrap(),
            max_temp: sanitized_data[1].parse::<f32>().unwrap(),
        })
    }

    all_days
}
