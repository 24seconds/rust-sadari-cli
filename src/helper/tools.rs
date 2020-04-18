use argh::FromArgs;
use rand::rngs::ThreadRng;
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    fs::File,
    io,
    io::prelude::*,
    process,
};

const MAX_NUMBER_OF_BLOCKS: i32 = 12;
const MIN_NUMBER_OF_BLOCKS: i32 = 2;

/// cli test
/// TODO: Remove Cli struct. no longer needed
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

    pub fn new(file_path: String) -> Cli {
        Cli {
            tick_rate: 250,
            file_path,
        }
    }

    pub fn get_mockup() -> Cli {
        Cli {
            tick_rate: 200,
            file_path: String::from("text.txt"),
        }
    }
}

pub fn get_input_from_file(filename: &String) -> Result<Vec<Vec<String>>, io::Error> {
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

#[derive(Debug)]
pub struct SadariEnvironment {
    pub number_of_blocks: u8,
    pub number_of_max_bridges: u8,
    pub y_coordinate: u16,
    pub rng: ThreadRng,
    pub name_vec: Vec<String>,
    pub result_vec: Vec<String>,
}

impl SadariEnvironment {
    pub fn default() -> SadariEnvironment {
        SadariEnvironment {
            number_of_blocks: 0,
            number_of_max_bridges: 6,
            y_coordinate: 10,
            rng: rand::thread_rng(),
            name_vec: Vec::new(),
            result_vec: Vec::new(),
        }
    }

    pub fn number_of_blocks(mut self, number_of_blocks: u8) -> Self {
        self.number_of_blocks = number_of_blocks;

        self
    }

    pub fn name_vec(mut self, name_vec: Vec<String>) -> Self {
        self.name_vec = name_vec;

        self
    }

    pub fn result_vec(mut self, result_vec: Vec<String>) -> Self {
        self.result_vec = result_vec;

        self
    }
}

impl Display for SadariEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
        sadari env, block : {}, \
        max_bridges : {}, \
        y_coordinate : {}, \
        \nname_vec : {:?}, \
        \nresult_vec : {:?}",
            self.number_of_blocks,
            self.number_of_max_bridges,
            self.y_coordinate,
            self.name_vec,
            self.result_vec
        )
    }
}

struct SadariData {
    selected_chunk: u8,
    bridge_hashmap: HashMap<u16, Vec<u16>>,
    path_hashmap: HashMap<u8, Vec<(u8, u8)>>,
    tick: i32,
    tick_speed: i32,
}

pub fn read_args<T>(args: T) -> SadariEvnironment
where
    T: Iterator<Item = String>,
{
    let args: Vec<String> = args.collect();

    if args.len() < 2 {
        // direct input mode

        // function name would be `ask_action`

        // input scenario

        // 1. ask number of blocks first
        // depend on number of blocks, reject or proceed
        // 2. get name inputs in one line, separated by comma
        // 3. get result inputs or offer skip.
        // check result inputs len and name inputs len is same as number of blocks that user have typed before
        // of course, quit option is provided

        return SadariEnvironment::default();
    }

    let filename = &args[1];
    let vec_read_file = get_input_from_file(filename).unwrap_or_else(|err| {
        panic!("get_input_from_file error : {}", err);
    });

    if vec_read_file.len() < 1 {
        panic!("test input file has few lines, provide 2 lines!");
    }

    let name_vec: &Vec<String> = vec_read_file
        .get(0)
        .ok_or_else(|| "no input for names")
        .unwrap_or_else(|err| {
            panic!("name_vec, test error : {}", err);
        });

    let number_of_bloks = name_vec.len();
    if number_of_bloks > MAX_NUMBER_OF_BLOCKS as usize {
        panic!(
            "name_vec length is larger than limit, length: {}, limit {}",
            number_of_bloks, MAX_NUMBER_OF_BLOCKS
        );
    }
    if number_of_bloks < MIN_NUMBER_OF_BLOCKS as usize {
        panic!(
            "name_vec length is smaller than limit, length: {}, limit {}",
            number_of_bloks, MIN_NUMBER_OF_BLOCKS
        );
    }

    let name_vec = name_vec.clone();
    let result_vec: Vec<String> = if vec_read_file.len() == 1 {
        eprintln!("because i got one line, result will be automatically set as number, 0..n");
        let vec: Vec<String> = (0..number_of_bloks as u8)
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        vec
    } else {
        vec_read_file.get(1).unwrap().clone()
    };


    if name_vec.len() != result_vec.len() {
        panic!(
            "name and result length are different name: {}, result: {}",
            name_vec.len(),
            result_vec.len()
        );
    }

    SadariEnvironment::default()
        .number_of_blocks(number_of_bloks as u8)
        .name_vec(name_vec)
        .result_vec(result_vec)
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
