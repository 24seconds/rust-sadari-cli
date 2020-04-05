use argh;
mod helper;
use helper::{read_file, Cli, Event, Events};
use rand::Rng;
use std::{collections::HashMap, error::Error, io, thread, time};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Widget,
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // let cli: Cli = argh::from_env();
    // mockup to focus on other things
    let cli: Cli = Cli::get_mockup();
    println!("cli is {:?}", &cli);
    read_file(&cli.get_path())
        .expect(format!("can not read file in path {:?}", &cli.get_path()).as_str());
    // read_args(env::args().collect());

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    let number_of_blocks: u8 = 3;
    let number_of_max_bridge = 6;

    let mut bridge_hashmap: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut rng = rand::thread_rng();
    for i in 0..(number_of_blocks - 1) {
        let random_bridge = rng.gen_range(2, number_of_max_bridge);
        let vec = helper::calc_bridge_layout(random_bridge);
        bridge_hashmap.insert(i.into(), vec);
    }

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10), // guide to user
                        Constraint::Percentage(80), // main render
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10), // guide to user
                        Constraint::Percentage(80), // main render
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .horizontal_margin(10)
                .split(chunks[1]);

            let mut block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Green));
            f.render(&mut block, main_chunks[0]);
            let mut block = Block::default()
                .borders(Borders::ALL)
                // .style(Style::default().bg(Color::Yellow));
                .style(Style::default());
            f.render(&mut block, main_chunks[1]);
            let mut block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White));
            f.render(&mut block, main_chunks[2]);

            let name_chunk = main_chunks[0];
            let vec = helper::calc_names_layout(number_of_blocks, 20, 10);

            // render name_chunks
            let name_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    vec.iter()
                        .map(|x| Constraint::Percentage(*x))
                        .collect::<Vec<Constraint>>(),
                )
                .split(name_chunk);

            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green));
            for i in 0..number_of_blocks {
                f.render(&mut block, name_chunks[i as usize * 2 + 1]);
            }

            // render result_chunks
            let result_chunk = main_chunks[2];
            let result_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    vec.iter()
                        .map(|x| Constraint::Percentage(*x))
                        .collect::<Vec<Constraint>>(),
                )
                .split(result_chunk);

            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue));
            for i in 0..number_of_blocks {
                f.render(&mut block, result_chunks[i as usize * 2 + 1]);
            }

            // render bridge vertical
            let bridge_chunks: Vec<Rect> = name_chunks
                .iter()
                .zip(result_chunks.iter())
                .map(|z| {
                    let n = z.0;
                    let r = z.1;
                    Rect::new(
                        n.x + n.width / 2,
                        n.y + n.height,
                        n.width / 2,
                        r.y - (n.y + n.height),
                    )
                })
                .collect();
            let mut line = Block::default()
                .borders(Borders::LEFT)
                .border_style(Style::default().fg(Color::LightBlue));

            for i in 0..number_of_blocks {
                f.render(&mut line, bridge_chunks[i as usize * 2 + 1]);
            }

            // render horizontal
            for i in 0..(number_of_blocks - 1) {
                let chunk_i = i as usize * 2 + 1;
                let bridge_chunk = Rect::new(
                    bridge_chunks[chunk_i].x + 1,
                    bridge_chunks[chunk_i].y,
                    bridge_chunks[chunk_i + 2].x - bridge_chunks[chunk_i].x - 1,
                    bridge_chunks[chunk_i].height,
                );
                let vec: &Vec<u16> = bridge_hashmap.get(&(i as u16)).unwrap();
                let ratio_length = vec.iter().fold(0, |acc, x| acc + x);

                let bridge_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        vec.iter()
                            .map(|x| Constraint::Ratio((*x).into(), ratio_length.into()))
                            .collect::<Vec<Constraint>>(),
                    )
                    .split(bridge_chunk);

                let mut bridge_horizontal = Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::Yellow));

                for i in 0..vec.len() {
                    f.render(
                        &mut bridge_horizontal,
                        Block::default().inner(bridge_chunks[i as usize]),
                    );
                }
            }
        })?;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
