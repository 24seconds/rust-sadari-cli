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
    let y_coordinate = 10;

    let mut bridge_hashmap: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut rng = rand::thread_rng();
    for i in 0..(number_of_blocks - 1) {
        let number_of_bridge = rng.gen_range(2, number_of_max_bridge);
        let vec = helper::calc_bridge_indexes(&mut rng, number_of_bridge, y_coordinate);
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

            // let bridge_chunk = Rect::new(bridge_chunks[1].x + 1, bridge_chunks[1].y, bridge_chunks[3].x - bridge_chunks[1].x - 1, bridge_chunks[1].height);
            // // let bridge_chunk = Rect::new( x: u16, y: u16, width: u16, height: u16);
            // // let random_bridge = rng.gen_range(1, number_of_max_bridge);
            // // let vec = helper::calc_bridge_layout(random_bridge);
            // let vec: &Vec<u16> = bridge_hashmap.get(&0).unwrap();
            // let ratio_length = vec.iter().fold(0, |acc, x| acc + x);
            // let bridge_chunks = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(
            //         vec.iter()
            //             .map(|x| Constraint::Ratio((*x).into(), ratio_length.into()))
            //             .collect::<Vec<Constraint>>(),

            //     )
            //     .split(bridge_chunk);
            // let mut bridge_horizontal = Block::default()
            //     .borders(Borders::BOTTOM)
            //     .border_style(Style::default().fg(Color::Yellow));

            // for i in 0..4 {
            //     f.render(&mut bridge_horizontal, Block::default().inner(bridge_chunks[i as usize]));
            // }

            // let another_chunks = Layout::default()
            //     .direction(Direction::Horizontal)
            //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            //     .split(chunks[0]);

            // let mut block = Block::default()
            //     .borders(Borders::LEFT | Borders::BOTTOM)
            //     // .title("Main block with white border")
            //     .border_style(
            //         Style::default().fg(Color::Green)
            //         .modifier(Modifier::DIM)
            //     );
            // f.render(&mut block, another_chunks[1]);
            // let size = f.size();

            // if flag == false {
            //     println!(
            //         "size is {}, {}, {}, {}",
            //         size.left(),
            //         size.right(),
            //         size.top(),
            //         size.bottom()
            //     );
            //     flag = true;
            // }

            // // const BOLD              = 0b0000_0000_0001;
            // // const DIM               = 0b0000_0000_0010;
            // // const ITALIC            = 0b0000_0000_0100;
            // // const UNDERLINED        = 0b0000_0000_1000;
            // // const SLOW_BLINK        = 0b0000_0001_0000;
            // // const RAPID_BLINK       = 0b0000_0010_0000;
            // // const REVERSED          = 0b0000_0100_0000;
            // // const HIDDEN            = 0b0000_1000_0000;
            // // const CROSSED_OUT       = 0b0001_0000_0000;

            // let mut block = Block::default()
            //     .borders(Borders::ALL)
            //     .title("Main block with white border")
            //     .border_style(Style::default().fg(Color::Green));
            // f.render(&mut block, chunks[1]);

            // let mut canvas = Canvas::default()
            //     // .block(block)
            //     .block(Block::default().borders(Borders::ALL).title("World").border_style(Style::default().bg(Color::Blue)))
            //     .paint(|ctx| {
            //         ctx.draw(&Line {
            //             x1: 0f64, y1: 0f64, x2: 0f64, y2: 100f64, color: Color::White,
            //         })
            //     })
            //     .x_bounds([-100.0, 100.0])
            //     .y_bounds([-100.0, 100.0]);
            // f.render(&mut canvas, block.inner(chunks[1]));
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
