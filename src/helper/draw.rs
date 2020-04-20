use crate::helper;
use crate::helper::SadariEnvironment;
use std::{collections::HashMap, error::Error, fmt};
use tui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    symbols::line,
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Frame, Terminal,
};

fn create_simple_block<'a>(borders: Borders, color: Color) -> Block<'a> {
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

pub enum LineDirection {
    Left,
    Right,
    Down,
}

enum BorderKind {
    Selected,
    NotSelected,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl BorderKind {
    fn color(self) -> Color {
        match self {
            BorderKind::Selected => Color::Red,
            BorderKind::NotSelected => Color::White,
        }
    }
}

#[derive(Clone, Copy)]
struct LineWidget {
    border_style: Style,
    line_type: &'static str,
}

impl LineWidget {
    fn new(border_style: Style, line_type: &'static str) -> Self {
        Self {
            border_style,
            line_type,
        }
    }
}

impl Widget for LineWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.line_type, Style::default());

        match self.line_type {
            line::VERTICAL => {
                for y in area.top()..area.bottom() {
                    buf.get_mut(area.left(), y)
                        .set_symbol(self.line_type)
                        .set_style(self.border_style);
                }
            }
            line::HORIZONTAL => {
                for x in area.left()..area.right() {
                    buf.get_mut(x, area.top())
                        .set_symbol(line::HORIZONTAL)
                        .set_style(self.border_style);
                }
            }
            _ => {}
        }
    }
}

struct Label<'a> {
    text: &'a str,
    text_style: Style,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label {
            text: "",
            text_style: Style::default(),
        }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, self.text_style);
    }
}

impl<'a> Label<'a> {
    fn text(mut self, text: &'a str) -> Label<'a> {
        self.text = text;
        self
    }

    fn text_style(mut self, style: Style) -> Self {
        self.text_style = style;

        self
    }
}

#[allow(dead_code)]
pub fn draw_bridge_point<B>(point_hashmap: &HashMap<Point, Point>, f: &mut Frame<B>)
where
    B: Backend,
{
    for (_, value) in point_hashmap {
        let mut point = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Red));

        f.render(&mut point, Rect::new(value.x as u16, value.y as u16, 2, 2));
    }
}

pub fn render_sadari<B>(
    terminal: &mut Terminal<B>,
    sadari_env: &SadariEnvironment,
    selected_chunk: u8,
    tick: &mut i32,
    rendering_state: &mut RenderingState,
    bridge_hashmap: &HashMap<u16, Vec<u16>>,
    path_hashmap: &HashMap<u8, Vec<(u8, u8)>>,
) -> Result<(), Box<dyn Error>>
where
    B: Backend,
{
    let number_of_blocks: u8 = sadari_env.number_of_blocks;

    let y_coordinate = sadari_env.y_coordinate;
    let name_vec = &sadari_env.name_vec;
    let result_vec = &sadari_env.result_vec;
    let tic_speed = 1;

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
            .constraints([Constraint::Percentage(100)].as_ref())
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
            .title_style(Style::default().modifier(Modifier::BOLD).fg(Color::Green))
            .title("Rust-Sadari-Cli!");

        let mut paragraph = Paragraph::new(text.iter())
            .block(block)
            .alignment(Alignment::Center);
        f.render(&mut paragraph, guide_chunk[0]);

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

        let name_chunk = main_chunks[0];
        let vec = helper::calc_names_layout(number_of_blocks, 3, 1).unwrap();

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

            // draw name texts
            let text = [Text::raw(name_vec.get(i as usize).unwrap())];
            let mut paragraph = Paragraph::new(text.iter())
                .alignment(Alignment::Center)
                .wrap(true);
            f.render(&mut paragraph, block.inner(name_chunks[i as usize * 2 + 1]));
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

            // draw result texts
            let text = [Text::raw(result_vec.get(i as usize).unwrap())];
            let mut paragraph = Paragraph::new(text.iter())
                .alignment(Alignment::Center)
                .wrap(true);
            f.render(
                &mut paragraph,
                block.inner(result_chunks[i as usize * 2 + 1]),
            );
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
        let mut left_tick = *tick;
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

            let mut line = {
                let border_style = Style::default().fg(Color::Red);
                let line_type = match direction {
                    LineDirection::Down => symbols::line::VERTICAL,
                    LineDirection::Right | LineDirection::Left => symbols::line::HORIZONTAL,
                };

                LineWidget::new(border_style, line_type)
            };

            f.render(&mut line, area);
        }

        if current_path_index == path.len() {
            // result chunk border should be red
            let (result_index, _) = path.last().unwrap();

            let mut block = create_simple_block(Borders::ALL, Color::Red);
            f.render(&mut block, result_chunks[*result_index as usize * 2 + 1]);

            *rendering_state = RenderingState::Done;
        }

        match rendering_state {
            RenderingState::Idle | RenderingState::Done => {}
            RenderingState::Drawing => {
                *tick += tic_speed;
            }
        };
    })?;

    Ok(())
}

pub fn render_result<B>(
    terminal: &mut Terminal<B>,
    sadari_env: &SadariEnvironment,
    path_hashmap: &HashMap<u8, Vec<(u8, u8)>>,
) -> Result<(), Box<dyn Error>>
where
    B: Backend,
{
    terminal.draw(|mut f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let length = path_hashmap.len() as u8;

        let vec_text: Vec<(&String, &String)> = (0..length)
            .into_iter()
            .map(|i| {
                let path = path_hashmap.get(&i).unwrap();
                let start = path.first().unwrap();
                let end = path.last().unwrap();

                let start = start.0;
                let end = end.0;

                let start = sadari_env.name_vec.get(start as usize).unwrap();
                let end = sadari_env.result_vec.get(end as usize).unwrap();

                (start, end)
            })
            .collect();

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Length(20),
                    Constraint::Percentage(45),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let mut label = Label::default()
            .text("Sadari Result")
            .text_style(Style::default().modifier(Modifier::BOLD).fg(Color::Green));
        f.render(&mut label, chunks[0]);

        let vec_start_text: Vec<Text> = vec_text
            .iter()
            .map(|x| {
                let (start, _) = *x;

                Text::raw(format!("{}\n\n", start))
            })
            .collect();
        let mut paragraph = Paragraph::new(vec_start_text.iter()).alignment(Alignment::Right);
        f.render(&mut paragraph, main_chunks[0]);

        let vec_line: Vec<Text> = (0..length)
            .into_iter()
            .map(|_| Text::raw("<───────────>\n\n"))
            .collect();
        let mut paragraph = Paragraph::new(vec_line.iter()).alignment(Alignment::Center);
        f.render(&mut paragraph, main_chunks[1]);

        let vec_end_text: Vec<Text> = vec_text
            .iter()
            .map(|x| {
                let (_, end) = *x;

                Text::raw(format!("{}\n\n", end))
            })
            .collect();
        let mut paragraph = Paragraph::new(vec_end_text.iter()).alignment(Alignment::Left);
        f.render(&mut paragraph, main_chunks[2]);
    })?;

    Ok(())
}
