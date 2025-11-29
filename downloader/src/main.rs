use std::env;
use std::fs;
use std::io::Write;
use reqwest::blocking::Client;

fn main() {
    dotenv::dotenv().ok(); // Opcional, si usas .env
    let cookie = env::var("AOC_COOKIE").expect("AOC_COOKIE no está definida");

    let client = Client::builder()
        .user_agent("github.com/jmgimeno/aoc2016 downloader")
        .build()
        .unwrap();

    fs::create_dir_all("data").unwrap();

    for day in 1..=25 {
        let url = format!("https://adventofcode.com/2016/day/{}/input", day);
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
            println!("Día {} descargado correctamente.", day);
        } else {
            println!("Error al descargar el día {}: {}", day, resp.status());
        }
    }
}
