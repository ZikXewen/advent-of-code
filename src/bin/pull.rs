use std::{path::Path, process::Command};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    year: usize,
    day: usize,
    session: String,
    create: Option<bool>,
    open: Option<bool>,
}

fn main() {
    // Read config
    let Config {
        year,
        day,
        session,
        create,
        open,
    } = serde_json::from_str(std::fs::read_to_string("config.json").unwrap().as_str()).unwrap();

    // Download input
    let url = format!("https://adventofcode.com/20{}/day/{}/input", year, day);
    let res = ureq::get(&url)
        .set("Cookie", format!("session={}", session).as_str())
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    std::fs::write("./input.txt", res).unwrap();

    // Create file if it doesn't exist
    if !create.unwrap_or(false) {
        return;
    }
    let boilerplate = r#"
use aoc::make_main;

fn compute(input: String) -> 

make_main! {}
"#
    .trim();
    let path = format!("./src/bin/y{}d{}p1.rs", year, day);
    if !Path::new(&path).exists() {
        std::fs::write(&path, boilerplate).unwrap();
    }

    // Open in VSCode
    if open.unwrap_or(false) {
        Command::new("code.cmd")
            .arg("-g")
            .arg(path + ":3:30")
            .spawn()
            .unwrap();
    }
}
