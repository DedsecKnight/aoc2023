#[macro_use] extern crate prettytable;
use macros::today;
use prettytable::Table;
use chrono::prelude::*;
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::{header, Method};
use solution::Solution;
use std::error::Error;
use std::{env, fs};

mod solution;

fn get_current_day() -> u32 {
    let t = Utc::now();
    let tz_offset = FixedOffset::west_opt(5 * 3600).unwrap();
    tz_offset.from_utc_datetime(&t.naive_utc()).day()
}

fn run_sample_cases() -> Result<(u32, u32), Box<dyn Error>> {
    let sample_input_data = fs::read_to_string("sample.txt")?;
    Ok((
        <today!()>::part_one(sample_input_data.lines()),
        <today!()>::part_two(sample_input_data.lines())
    ))
}

fn run_main_case() -> Result<(u32, u32), Box<dyn Error>> {
    let today = get_current_day();
    let client = Client::new();
    let login_session =
        env::var("LOGIN_SESSION").unwrap_or_else(|_| panic!("Cannot find LOGIN_SESSION"));
    match client
        .request(
            Method::GET,
            format!("https://adventofcode.com/2023/day/{}/input", today),
        )
        .header(header::COOKIE, format!("session={}", login_session))
        .send()
    {
        Ok(res) => {
            let input_data = res.text()?;
            Ok((
                <today!()>::part_one(input_data.lines()),
                <today!()>::part_two(input_data.lines())
            ))
        }
        Err(err) => {
            eprintln!("Error getting input for today: {:#?}", err);
            Err(Box::new(err))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let mut table = Table::new();
    let (sample_p1, sample_p2) = run_sample_cases()?;
    let (main_p1, main_p2) = run_main_case()?;
    println!("");
    table.add_row(row!["", "Part 1", "Part 2"]);
    table.add_row(row!["Sample input", sample_p1, sample_p2]);
    table.add_row(row!["Main input", main_p1, main_p2]);
    table.printstd();
    println!("");
    Ok(())
}
