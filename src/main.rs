use argh;
mod helper;
use helper::{
    calc_next_index, calc_prev_index, create_simple_block, read_file, BorderKind, Cli, Event,
    Events, LineDirection, Point, RenderingState,
};
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io,
    iter::FromIterator,
    thread, time,
};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph, Text, Widget,
    },
    Terminal,
};
// use Extend::extend;

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
    let number_of_max_bridges = 6;
    let y_coordinate = 10;

    let mut rng = rand::thread_rng();
    let bridge_hashmap = helper::calc_bridge_hashmap(
        number_of_blocks,
        number_of_max_bridges,
        y_coordinate,
        &mut rng,
    );

    let mut selected_chunk = 0u8;
    let mut path_hashmap = HashMap::new();
    for index in 0..number_of_blocks {
        let path = helper::calc_path(index, &bridge_hashmap, y_coordinate as u8);
        path_hashmap.insert(index, path);
    }

    // helper::print_hashmap(String::from("bridge_hashmap"), &bridge_hashmap);
    // helper::print_hashmap(String::from("path_hashmap"), &path_hashmap);

    let mut tick = 0;
    let tic_speed = 1;

    // prevent key event input while doing animation
    let mut rendering_state = RenderingState::Idle;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(15), // guide to user
                        Constraint::Percentage(80), // main render
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // draw guide text
            let guide_chunk = chunks[0];
            let guide_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .horizontal_margin(10)
                .vertical_margin(1)
                .split(guide_chunk);

            let text = [Text::raw(
                r#"
←, → or h,l : Left, Right     s, enter : Start path animation
q           : Quit            r        : Go to result
                "#,
            )];

            let block = Block::default()
                .borders(Borders::NONE)
                .title_style(Style::default().modifier(Modifier::BOLD).fg(Color::Blue))
                .title("Rust-Sadari-Cli!");
            let mut paragraph = Paragraph::new(text.iter())
                .block(block)
                .alignment(Alignment::Left);
            f.render(&mut paragraph, guide_chunk[1]);

            // draw footer
            let footer_chunk = chunks[2];
            let footer_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(100),
                        // Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .horizontal_margin(10)
                .split(footer_chunk);

            let text = [Text::styled(
                "\n\n🍺 Github: 24seconds/rust-sadari-cli, powered by 24seconds",
                Style::default().fg(Color::Yellow),
            )];
            let mut paragraph = Paragraph::new(text.iter()).alignment(Alignment::Center);
            f.render(&mut paragraph, footer_chunk[0]);

            // main chunk part
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

            for i in 0..number_of_blocks {
                let mut block = create_simple_block(
                    Borders::ALL,
                    match i {
                        _ if i == selected_chunk => BorderKind::Selected.color(),
                        _ => BorderKind::NotSelected.color(),
                    },
                );
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

            let mut block = create_simple_block(Borders::ALL, Color::White);
            for i in 0..number_of_blocks {
                f.render(&mut block, result_chunks[i as usize * 2 + 1]);
            }

            let mut bridge_point_hashmap: HashMap<Point, Point> = HashMap::new();

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

            let mut line = create_simple_block(Borders::LEFT, Color::LightBlue);
            for i in 0..number_of_blocks {
                f.render(&mut line, bridge_chunks[i as usize * 2 + 1]);

                // collect bridge vertical points
                let Rect {
                    x,
                    y,
                    width: _,
                    height,
                } = bridge_chunks[i as usize * 2 + 1];

                bridge_point_hashmap
                    .insert(Point::new(i as i32, -1), Point::new(x as i32, y as i32 - 1));
                bridge_point_hashmap.insert(
                    Point::new(i as i32, y_coordinate as i32),
                    Point::new(x as i32, (y + height) as i32),
                );
            }

            // render bridge horizontal
            for i in 0..(number_of_blocks - 1) {
                let chunk_i = i as usize * 2 + 1;
                let bridge_chunk = Rect::new(
                    bridge_chunks[chunk_i].x + 1,
                    bridge_chunks[chunk_i].y + 1,
                    bridge_chunks[chunk_i + 2].x - bridge_chunks[chunk_i].x - 1,
                    bridge_chunks[chunk_i].height - 2,
                );

                let vec_indexes: &Vec<u16> = bridge_hashmap.get(&(i as u16)).unwrap();
                let bridge_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        helper::calc_distributed_height(y_coordinate + 1, bridge_chunk.height)
                            .iter()
                            .map(|x| Constraint::Length(*x))
                            .collect::<Vec<Constraint>>(),
                    )
                    .split(bridge_chunk);

                let mut bridge_horizontal = create_simple_block(Borders::BOTTOM, Color::Yellow);
                vec_indexes.iter().for_each(|vec_index| {
                    f.render(&mut bridge_horizontal, bridge_chunks[*vec_index as usize]);

                    // collect bridge horizontal points
                    let Rect {
                        x,
                        y,
                        width,
                        height,
                    } = bridge_chunks[*vec_index as usize];

                    bridge_point_hashmap.insert(
                        Point::new(i as i32, *vec_index as i32),
                        Point::new(x as i32 - 1, (y + height - 1) as i32),
                    );
                    bridge_point_hashmap.insert(
                        Point::new(i as i32 + 1, *vec_index as i32),
                        Point::new((x - 1 + width + 1) as i32, (y + height - 1) as i32),
                    );
                });
            }

            // draw animation
            let path = path_hashmap.get(&selected_chunk).unwrap();
            // helper::print_hashmap(String::from("bridge_point_hashmap"), &bridge_point_hashmap);

            let mut current_path_index = 0;
            let mut left_tick = tick;
            while left_tick > 0 && current_path_index < path.len() as usize {
                let (tick, area, direction, next_path_index) = helper::calc_partial_line(
                    &bridge_point_hashmap,
                    &path,
                    left_tick,
                    current_path_index as i32,
                    selected_chunk,
                );

                left_tick = tick;
                current_path_index = next_path_index as usize;

                let mut line = create_simple_block(
                    match direction {
                        LineDirection::Down => Borders::LEFT,
                        LineDirection::Right => Borders::TOP,
                        LineDirection::Left => Borders::TOP,
                    },
                    Color::Green,
                );

                f.render(&mut line, area);
            }

            if current_path_index == path.len() {
                // result chunk border should be red
                let (result_index, _) = path.last().unwrap();

                let mut block = create_simple_block(Borders::ALL, Color::Red);
                f.render(&mut block, result_chunks[*result_index as usize * 2 + 1]);

                rendering_state = RenderingState::Done;
            }

            match rendering_state {
                RenderingState::Idle | RenderingState::Done => {}
                RenderingState::Drawing => {
                    tick += tic_speed;
                }
            };

            // render all points for debug
            // for (_, value) in bridge_point_hashmap {
            //     let mut point = Block::default()
            //         .borders(Borders::TOP)
            //         .border_style(Style::default().fg(Color::Red));

            //     f.render(&mut point, Rect::new(value.x as u16, value.y as u16, 2, 2));
            // }
        })?;

        if rendering_state == RenderingState::Drawing {
            continue;
        }

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                val if [Key::Left, Key::Right, Key::Char('h'), Key::Char('l')].contains(&val) => {
                    match rendering_state {
                        RenderingState::Idle | RenderingState::Done => {
                            rendering_state = RenderingState::Idle;
                            tick = 0;

                            match key {
                                Key::Left | Key::Char('h') => {
                                    selected_chunk =
                                        calc_prev_index(selected_chunk, number_of_blocks);
                                }
                                Key::Right | Key::Char('l') => {
                                    selected_chunk =
                                        calc_next_index(selected_chunk, number_of_blocks);
                                }
                                _ => {}
                            };
                        }
                        RenderingState::Drawing => {}
                    };
                }
                Key::Char('\u{000A}') | Key::Char('s') => {
                    rendering_state = RenderingState::Drawing;
                }
                _ => {
                    eprintln!("key : {:?}", key);
                }
            },
            _ => {}
        }
    }

    Ok(())
}
