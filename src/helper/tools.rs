use argh::FromArgs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Line {
    x_i: u32,
    y_i: u32,
    x_f: u32,
    y_f: u32,
}

fn draw_line(_line: Line, _termion: Line) {
    // draw line with style?
}

/// cli test
#[derive(Debug, FromArgs)]
pub struct Cli {
    #[argh(option, default = "250")]
    /// tick_rate
    tick_rate: u64,

    /// file path to read
    #[argh(option)]
    file_path: String,
}

impl Cli {
    pub fn get_path(&self) -> &String {
        &self.file_path
    }

    pub fn get_mockup() -> Cli {
        Cli {
            tick_rate: 200,
            file_path: String::from("text.txt"),
        }
    }
}

pub fn read_file(filename: &String) -> Result<(), io::Error> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(&file);

    for line in reader.lines() {
        let t: String = line.unwrap();
        println!("{}", t);
    }

    Ok(())
}

pub fn read_args(args: Vec<String>) {
    println!("args is {:?}", args);

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    println!("trimmed id {}", trimmed);

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let trimmed = input_text.trim();
    println!("trimmed second line is {}", trimmed);
}
