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
const NUMBER_OF_LINES_TO_READ: i32 = 2;

#[derive(Debug)]
pub struct SadariEnvironment {
    pub number_of_blocks: u8,
    pub number_of_max_bridges: u8,
    pub y_coordinate: u16,
    rng: ThreadRng,
    pub name_vec: Vec<String>,
    pub result_vec: Vec<String>,
    pub tick_rate: u64,
}

impl SadariEnvironment {
    fn default() -> SadariEnvironment {
        SadariEnvironment {
            number_of_blocks: 0,
            number_of_max_bridges: 6,
            y_coordinate: 10,
            rng: rand::thread_rng(),
            name_vec: Vec::new(),
            result_vec: Vec::new(),
            tick_rate: 250,
        }
    }

    fn number_of_blocks(mut self, number_of_blocks: u8) -> Self {
        self.number_of_blocks = number_of_blocks;

        self
    }

    fn name_vec(mut self, name_vec: Vec<String>) -> Self {
        self.name_vec = name_vec;

        self
    }

    fn result_vec(mut self, result_vec: Vec<String>) -> Self {
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

mod interaction {
    use super::{SadariEnvironment, MAX_NUMBER_OF_BLOCKS, MIN_NUMBER_OF_BLOCKS};
    use std::io;
    use std::io::prelude::*;

    pub enum State {
        Idle,
        NameInput,
        ResultInput,
        BeforeDone,
        Done,
        Quit,
    }

    impl State {
        fn next_state(self) -> State {
            match self {
                State::Idle => State::NameInput,
                State::NameInput => State::ResultInput,
                State::ResultInput => State::BeforeDone,
                State::BeforeDone => State::Done,
                _ => self,
            }
        }

        fn previous_state(self) -> State {
            match self {
                State::NameInput => State::Idle,
                State::ResultInput => State::NameInput,
                State::BeforeDone => State::ResultInput,
                _ => self,
            }
        }
    }

    pub fn help_guide() {
        let text = r#"
        There are TWO modes to run sadari application.
        1. Using file path as input
        2. Interacting with user by asking sevearl Questions.

        1 -> For file as input mode example : cargo run ./text.txt
        2 -> For interaction mode example : cargo run

        Enjoy!
        "#;

        println!("{}", text);
    }

    fn idle_guide() {
        println!("\tType list of names separated by comma! ex) name1, name2, name3 ...\n");
        println!("\tQ,q) Quit\n");
        print!("type: ");

        io::stdout().flush().unwrap();
    }

    fn name_input_guide(v: &Vec<String>) {
        println!("\tIs that right? \n\tnames: {:?}, len: {}\n", v, v.len());
        println!("\tY, y) yes");
        println!("\tN, n) no");
        println!("\tQ,q) Quit\n");
        print!("type: ");
        io::stdout().flush().unwrap();
    }

    fn result_input_guide() {
        println!("\tType list of results separated by comma! ex) result1, result2, result3 ...");
        println!("\tR, r) If you want auto generated results");
        println!("\tQ,q) Quit\n");
        print!("type: ");
        io::stdout().flush().unwrap();
    }

    fn before_done_guide(sadari_env: &SadariEnvironment) {
        println!(
            "\tIs that right? \n\tname: {:?}, len: {}",
            sadari_env.name_vec,
            sadari_env.name_vec.len()
        );
        println!(
            "\tresult: {:?}, len: {}\n",
            sadari_env.result_vec,
            sadari_env.result_vec.len()
        );
        println!("\tY, y) yes");
        println!("\tN, n) no");
        println!("\tQ,q) Quit\n");
        print!("type: ");
        io::stdout().flush().unwrap();
    }

    pub fn handle_state_guide(state: &State, sadari_env: &SadariEnvironment) {
        match state {
            State::Idle => idle_guide(),
            State::NameInput => name_input_guide(&sadari_env.name_vec),
            State::ResultInput => result_input_guide(),
            State::BeforeDone => before_done_guide(&sadari_env),
            _ => {}
        };
    }

    pub fn handle_user_input(
        action: String,
        state: State,
        sadari_env: SadariEnvironment,
    ) -> (SadariEnvironment, State) {
        let action = action.trim();

        let mut next_sadari_env = sadari_env;

        match action {
            "Q" | "q" => (next_sadari_env, State::Quit),
            "Y" | "y" => {
                let (is_valid, message) = validate_input(&state, &next_sadari_env);

                match message {
                    Some(m) => println!("{}", m),
                    None => {}
                };

                let next_state = match state {
                    State::NameInput => {
                        if is_valid {
                            state.next_state()
                        } else {
                            state
                        }
                    }
                    State::BeforeDone => {
                        if is_valid {
                            state.next_state()
                        } else {
                            state.previous_state()
                        }
                    }
                    _ => state,
                };

                (next_sadari_env, next_state)
            }
            "N" | "n" => {
                let next_state = match state {
                    State::NameInput => {
                        next_sadari_env = next_sadari_env.name_vec(Vec::new()).number_of_blocks(0);

                        state.previous_state()
                    }
                    State::BeforeDone => {
                        next_sadari_env = next_sadari_env.result_vec(Vec::new());

                        state.previous_state()
                    }
                    _ => state,
                };

                (next_sadari_env, next_state)
            }
            "R" | "r" => {
                let next_state = match state {
                    State::ResultInput => {
                        let vec: Vec<String> = (0..next_sadari_env.number_of_blocks as u8)
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect();

                        next_sadari_env = next_sadari_env.result_vec(vec);

                        state.next_state()
                    }
                    _ => state,
                };

                (next_sadari_env, next_state)
            }
            _ => {
                let vec: Vec<String> = action
                    .split(",")
                    .map(move |x| String::from(x.trim()))
                    .collect();

                let next_state = match state {
                    State::Idle => {
                        next_sadari_env = next_sadari_env
                            .number_of_blocks(vec.len() as u8)
                            .name_vec(vec);

                        state.next_state()
                    }
                    State::ResultInput => {
                        next_sadari_env = next_sadari_env.result_vec(vec);

                        let (is_valid, message) = validate_input(&state, &next_sadari_env);

                        match message {
                            Some(m) => println!("{}", m),
                            None => {}
                        };

                        if is_valid {
                            state.next_state()
                        } else {
                            next_sadari_env = next_sadari_env.result_vec(Vec::new());

                            state
                        }
                    }
                    _ => state,
                };

                (next_sadari_env, next_state)
            }
        }
    }

    fn validate_input(state: &State, sadari_env: &SadariEnvironment) -> (bool, Option<String>) {
        match state {
            State::NameInput => {
                let len = sadari_env.name_vec.len();

                if len >= MIN_NUMBER_OF_BLOCKS as usize && len <= MAX_NUMBER_OF_BLOCKS as usize {
                    (true, None)
                } else {
                    (
                        false,
                        Some(format!(
                            "\n\tInput length should be {} <= input_length <= {}\n",
                            MIN_NUMBER_OF_BLOCKS, MAX_NUMBER_OF_BLOCKS
                        )),
                    )
                }
            }
            State::BeforeDone | State::ResultInput => {
                let name_vec_len = sadari_env.name_vec.len();
                let result_vec_len = sadari_env.result_vec.len();

                if name_vec_len != result_vec_len {
                    return (
                        false,
                        Some(format!(
                            "\n\tLengths are different! name: {}, result: {}\n",
                            name_vec_len, result_vec_len
                        )),
                    );
                }

                if result_vec_len >= MIN_NUMBER_OF_BLOCKS as usize
                    && result_vec_len <= MAX_NUMBER_OF_BLOCKS as usize
                {
                    (true, None)
                } else {
                    (
                        false,
                        Some(format!(
                            "\n\tInput length should be {} <= input_length <= {}\n",
                            MIN_NUMBER_OF_BLOCKS, MAX_NUMBER_OF_BLOCKS
                        )),
                    )
                }
            }
            _ => (false, None),
        }
    }
}

fn read_args_from_stdin() -> SadariEnvironment {
    let mut sadari_env = SadariEnvironment::default();
    let mut state = interaction::State::Idle;

    loop {
        interaction::handle_state_guide(&state, &sadari_env);

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("read error");

        // println!("\naction is {}\n", action);

        let (next_sadari_env, next_state) =
            interaction::handle_user_input(action, state, sadari_env);
        state = next_state;
        sadari_env = next_sadari_env;

        match state {
            interaction::State::Quit => {
                process::exit(0);
            }
            interaction::State::Done => {
                break;
            }
            _ => {}
        }
    }

    sadari_env
}

fn get_input_from_file(filename: &String) -> Result<Vec<Vec<String>>, io::Error> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(&file);

    let mut vec: Vec<Vec<String>> = Vec::new();
    let mut line_iter = reader.lines();

    (0..NUMBER_OF_LINES_TO_READ).into_iter().for_each(|_| {
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

fn read_args_from_file(args: Vec<String>) -> SadariEnvironment {
    let filename = &args[1];
    let vec_read_file = get_input_from_file(filename).unwrap_or_else(|err| {
        panic!("\n\tget_input_from_file error : {}", err);
    });

    if vec_read_file.len() < 1 {
        panic!("\n\ttest input file has few lines, provide 2 lines!");
    }

    let name_vec: &Vec<String> = vec_read_file
        .get(0)
        .ok_or_else(|| "no input for names")
        .unwrap_or_else(|err| {
            panic!("\n\tname_vec, test error : {}", err);
        });

    let number_of_bloks = name_vec.len();
    if number_of_bloks > MAX_NUMBER_OF_BLOCKS as usize {
        panic!(
            "\n\tname_vec length is larger than limit, length: {}, limit {}",
            number_of_bloks, MAX_NUMBER_OF_BLOCKS
        );
    }
    if number_of_bloks < MIN_NUMBER_OF_BLOCKS as usize {
        panic!(
            "\n\tname_vec length is smaller than limit, length: {}, limit {}",
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
            "\n\tname and result length are different name: {}, result: {}",
            name_vec.len(),
            result_vec.len()
        );
    }

    SadariEnvironment::default()
        .number_of_blocks(number_of_bloks as u8)
        .name_vec(name_vec)
        .result_vec(result_vec)
}

pub fn read_args<T>(args: T) -> SadariEnvironment
where
    T: Iterator<Item = String>,
{
    let args: Vec<String> = args.collect();

    if args.len() >= 2 && ["help", "--help"].contains(&args[1].as_str()) {
        interaction::help_guide();
        process::exit(0);
    }

    if args.len() < 2 {
        read_args_from_stdin()
    } else {
        read_args_from_file(args)
    }
}

fn _print_hashmap<K, V>(name: String, hashmap: &HashMap<K, V>)
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
