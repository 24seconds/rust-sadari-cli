use std::fmt;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::line,
    widgets::{Block, Borders, Widget},
};

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

pub enum LineDirection {
    Left,
    Right,
    Down,
}

pub enum BorderKind {
    Selected,
    NotSelected,
}

struct Line {
    x_i: u32,
    y_i: u32,
    x_f: u32,
    y_f: u32,
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

impl BorderKind {
    pub fn color(self) -> Color {
        match self {
            BorderKind::Selected => Color::Red,
            BorderKind::NotSelected => Color::White,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LineWidget<'a> {
    line_type: &'a str,
    border_style: Style,
}

impl<'a> Default for LineWidget<'a> {
    fn default() -> LineWidget<'a> {
        LineWidget {
            line_type: line::VERTICAL,
            border_style: Style::default(),
        }
    }
}

impl<'a> LineWidget<'a> {
    pub fn border_style(mut self, style: Style) -> LineWidget<'a> {
        self.border_style = style;
        self
    }

    pub fn line_type(mut self, line_type: &'a str) -> LineWidget<'a> {
        self.line_type = line_type;
        self
    }
}

impl<'a> Widget for LineWidget<'a> {
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
