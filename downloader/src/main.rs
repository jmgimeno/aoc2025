use std::{env, io};
use std::fs;
use std::io::Write;
use reqwest::blocking::Client;

fn main() {
    dotenv::dotenv().ok();
    let cookie = env::var("AOC_COOKIE").expect("AOC_COOKIE undefined");

    let client = Client::builder()
        .user_agent("github.com/jmgimeno/aoc2016 downloader")
        .build()
        .unwrap();

    fs::create_dir_all("data").unwrap();

    let day = loop {
        match ask_for_day() {
            Some(d) => break d,
            None => {
                eprintln!("Invalid input, please try again.");
                continue;
            }
        }
    };

    download_day(day, &client, &cookie);
}

fn ask_for_day() -> Option<u32> {
    print!("Enter day to download (1-12): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input.");
        return None;
    }

    let day: u32 = match input.trim().parse() {
        Ok(n) if (1..=12).contains(&n) => n,
        _ => {
            eprintln!("Invalid day. Please enter a number between 1 and 12.");
            return None;
        }
    };
    Some(day)
}

#[allow(unused)]
fn download_all(client: &Client, cookie: &String) {
    for day in 1..=12 {
        download_day(day, client, cookie);
    }
}

fn download_day(day: u32, client: &Client, cookie: &String) {
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .unwrap();

    if resp.status().is_success() {
        let content = resp.text().unwrap();
        let filename = format!("data/day{:02}.txt", day);
        let mut file = fs::File::create(&filename).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        println!("Day {} downloaded.", day);
    } else {
        println!("Error downloading day {}: {}", day, resp.status());
    }
}
