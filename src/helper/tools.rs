use argh::FromArgs;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::File,
    io,
    io::prelude::*,
};

/// cli test
#[derive(Debug, FromArgs)]
pub struct Cli {
    #[argh(option, default = "250")]
    /// tick_rate
    pub tick_rate: u64,

    /// file path to read
    #[argh(option)]
    pub file_path: String,
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

pub fn read_file(filename: &String) -> Result<Vec<Vec<String>>, io::Error> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(&file);

    let mut vec: Vec<Vec<String>> = Vec::new();
    let mut line_iter = reader.lines();

    (0..2).into_iter().for_each(|_| {
        let line = line_iter.next();

        match line {
            Some(l) => {
                let s: String = l.unwrap();
                let v: Vec<String> = s.split(",").map(move |x| String::from(x.trim())).collect();
                vec.push(v);
            }
            None => {}
        };
    });

    Ok(vec)
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
