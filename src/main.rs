use argh;
mod helper;
use helper::{read_file, Cli, Event, Events};
use std::{error::Error, io, thread, time};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect, Margin},
    style::{Color, Modifier, Style},
    widgets::{canvas::{Canvas, Line}, Block, Borders, Widget},
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
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    let number_of_blocks = 3;

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
            
            let mut block = Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Green));
            f.render(&mut block, main_chunks[0]);
            let mut block = Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Yellow));
            f.render(&mut block, main_chunks[1]);
            let mut block = Block::default().borders(Borders::ALL).style(Style::default().bg(Color::White));
            f.render(&mut block, main_chunks[2]);
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
