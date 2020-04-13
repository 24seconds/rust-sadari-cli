use argh::FromArgs;
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    fs::File,
    io,
    io::prelude::*,
};
use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

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

pub enum BorderKind {
    Selected,
    NotSelected,
}

impl BorderKind {
    pub fn color(self) -> Color {
        match self {
            BorderKind::Selected => Color::Red,
            BorderKind::NotSelected => Color::Green,
        }
    }
}

pub enum LineDirection {
    Left,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn default() -> Self {
        Point { x: 0, y: 0 }
    }
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub fn print_hashmap<K, V>(name: String, hashmap: &HashMap<K, V>)
where
    K: Debug + Display,
    V: Debug,
{
    eprintln!("\n{} --------------", &name);
    for (key, value) in hashmap {
        eprintln!("key : {}, value : {:?}", key, value);
    }
    eprintln!("{} --------------\n", &name);
}

pub fn create_simple_block<'a>(borders: Borders, color: Color) -> Block<'a> {
    Block::default()
        .borders(borders)
        .border_style(Style::default().fg(color))
}

#[derive(PartialEq)]
pub enum RenderingState {
    Idle,
    Drawing,
    Done,
}
