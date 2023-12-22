use crate::error::Error;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use inquire::Confirm;
use notch::servers::runner::Runner;
use notch::servers::server::Server;

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    #[default]
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Reset,
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
    let bytes = bytes
        .bytes()
        .expect("😧 Failed to get bytes (Check your internet connection)");
    let max = bytes.len();
    let chunks = bytes.chunks(4096);
    let mut current = 0;
    for chunk in chunks {
        current += chunk.len();
        let percent = (current as f32 / max as f32) * 100.0;
        print!("\r🗂️ Downloading... {percent:.2}%");
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

pub fn canonize(path: &Path) -> Result<PathBuf, Error> {
    let full_path = fs::canonicalize(path)?;
    let full_path = full_path.to_str().unwrap().trim_start_matches("\\\\?\\");
    let full_path = PathBuf::from(full_path);
    Ok(full_path)
}

pub fn start_server(server: &Server) -> Result<(), Error> {
    let mut runner = Runner::new(server, vec![], vec![]);
    let mut child = runner.start()?;

    // Reader
    if let Some(ref mut stdout) = child.stdout {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            let text = line.unwrap();
            let text = text
                .escape_default()
                .to_string();

            // Remove [HH:MM:SS] from text
            let text = text.split(' ').collect::<Vec<&str>>();
            let time = text[0..2].join(" ");
            let text = text[2..].join(" ");
            if text.starts_with("You need to agree to the EULA in order to run the server.") {
                println!("🚨 You need to agree to the EULA in order to run the server.");

                // Prompt for Y/N (default: N)
                let answer = Confirm::new("🎚️ Do you want to agree to the EULA? (Y/N)").prompt()?;
                if answer {
                    server.accept_eula()?;
                    println!("🎚️ EULA accepted! You can now start the server.");
                } else {
                    println!("🎚️ EULA not accepted!");
                }
                break;
            }

            println!("{time} {text}");
        }
    }

    let status = child.wait()?;
    Ok(())
}