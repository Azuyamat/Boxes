use std::fs::File;
use std::io::{Write};
use reqwest::blocking::Response;
use crate::error::Error;

pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Reset
}

impl Color {
    fn to_str(&self) -> &str {
        // Color w/ ascii code
        match self {
            Color::Black => "\x1b[30m",
            Color::DarkBlue => "\x1b[34m",
            Color::DarkGreen => "\x1b[32m",
            Color::DarkAqua => "\x1b[36m",
            Color::DarkRed => "\x1b[31m",
            Color::DarkPurple => "\x1b[35m",
            Color::Gold => "\x1b[33m",
            Color::Gray => "\x1b[37m",
            Color::DarkGray => "\x1b[90m",
            Color::Blue => "\x1b[94m",
            Color::Green => "\x1b[92m",
            Color::Aqua => "\x1b[96m",
            Color::Red => "\x1b[91m",
            Color::LightPurple => "\x1b[95m",
            Color::Yellow => "\x1b[93m",
            Color::White => "\x1b[97m",
            Color::Reset => "\x1b[0m",
        }
    }
}

pub fn colorize(string: &str, color: Color) -> String {
    format!("{}{}{}", color.to_str(), string, Color::Reset.to_str())
}

pub fn download(bytes: Response, file: &mut File) {
    println!("🗂️ Preparing to download...");
    let bytes = bytes.bytes().expect("😧 Failed to get bytes (Check your internet connection)");
    let max = bytes.len();
    let chunks = bytes.chunks(4096);
    let mut current = 0;
    for chunk in chunks {
        current += chunk.len();
        let percent = (current as f32 / max as f32) * 100.0;
        print!("\r🗂️ Downloading... {:.2}%", percent);
        file.write_all(chunk).expect("😧 Failed to write to file");
    }
}

#[macro_export]
macro_rules! get_exec_time {
    ($func:expr) => {{
        let start = std::time::Instant::now();
        $func;
        let end = start.elapsed();
        format!("{:.2}s", end.as_secs_f32())
    }};
}


pub fn read_line(prompt: &str) -> Result<String, Error> {
    let text = inquire::Text::new(prompt).prompt()?;
    Ok(text)
}