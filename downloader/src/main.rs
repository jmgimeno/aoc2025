use clap::Parser;
use reqwest::blocking::Client;
use std::fs;
use std::io::Write;
use std::env;

/// A program to download input for a day in AOC
#[derive(Debug, Parser)]
struct Args {
    /// Day to download
    #[clap(short, long)]
    day: String,
}

fn main() {
    dotenv::dotenv().ok();
    let cookie = env::var("AOC_COOKIE").expect("AOC_COOKIE undefined");

    let args = Args::parse();
    let day = parse_day(&args.day).expect(&format!("could not parse day in {}", args.day));

    let client = Client::builder()
        .user_agent("github.com/jmgimeno/aoc2016 downloader")
        .build()
        .unwrap();

    fs::create_dir_all("data").expect("could not create output directory");

    download_day(day, &client, &cookie);
}

fn parse_day(day: &str) -> Option<u32> {
    day.strip_prefix("day").and_then(|rest| {
        if rest.len() == 2 && rest.chars().all(|c| c.is_ascii_digit()) {
            rest.parse::<u32>().ok()
        } else {
            None
        }
    })
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
