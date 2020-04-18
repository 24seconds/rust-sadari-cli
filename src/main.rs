mod helper;
use helper::{
    calc_next_index, calc_prev_index, create_simple_block, BorderKind, Config, Event, Events,
    LineDirection, LineWidget, Point, RenderingState,
};
use std::{collections::HashMap, env, error::Error, io, time::Duration};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, Borders, Paragraph, Text},
    Terminal,
};
// use Extend::extend;

fn main() -> Result<(), Box<dyn Error>> {
    let sadari_env = helper::read_args(env::args());

    eprintln!("sadari_env is {:?}", sadari_env);
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(sadari_env.tick_rate),
        ..Config::default()
    });

    let number_of_blocks: u8 = sadari_env.number_of_blocks;
    let number_of_max_bridges = sadari_env.number_of_max_bridges;
    let y_coordinate = sadari_env.y_coordinate;
    let name_vec = &sadari_env.name_vec;
    let result_vec = &sadari_env.result_vec;

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

    let mut tick = 0;
    let mut sadari_render_flag = true;

    helper::print_hashmap("path_hashmap".to_string(), &path_hashmap);

    // prevent key event input while doing animation
    let mut rendering_state = RenderingState::Idle;

    loop {
        if !sadari_render_flag {
            // render result pages
            helper::render_result(&mut terminal, &sadari_env, &path_hashmap)?;
        } else {
            helper::render_sadari(
                &mut terminal,
                &sadari_env,
                selected_chunk,
                &mut tick,
                &mut rendering_state,
                &bridge_hashmap,
                &path_hashmap,
            )?;
        }

        if rendering_state == RenderingState::Drawing {
            continue;
        }

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Char('r') => {
                    sadari_render_flag = !sadari_render_flag;
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
