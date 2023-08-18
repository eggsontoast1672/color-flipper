use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use macroquad::prelude::{self as mq, *};

const COLOR_WIDTH: f32 = WINDOW_WIDTH / 2.0 - COLOR_HORIZONTAL_PADDING * 2.0 - LINE_THICKNESS / 2.0;
const COLOR_HEIGHT: f32 = 50.0;
const COLOR_RADIUS: f32 = 15.0;
const COLOR_HORIZONTAL_PADDING: f32 = 100.0;
const COLOR_VERTICAL_PADDING: f32 = 20.0;

const ENTRY_X: f32 = WINDOW_WIDTH / 2.0 - ENTRY_WIDTH / 2.0;
const ENTRY_Y: f32 = WINDOW_HEIGHT / 2.0 - ENTRY_HEIGHT / 2.0;
const ENTRY_WIDTH: f32 = 400.0;
const ENTRY_HEIGHT: f32 = 100.0;
const ENTRY_RADIUS: f32 = 20.0;
const ENTRY_COLOR: mq::Color = WHITE;

const LINE_THICKNESS: f32 = 10.0;
const LINE_COLOR: mq::Color = BLACK;

const TEXT_COLOR: mq::Color = BLACK;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, r: f32, color: mq::Color) {
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color); // this
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color); // this

    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
}

#[derive(Debug)]
enum ColorErrorKind {
    InvalidHexit,
    InvalidFormat,
}

#[derive(Debug)]
struct ParseColorError {
    kind: ColorErrorKind,
}

impl Display for ParseColorError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.kind {
            ColorErrorKind::InvalidHexit => write!(f, "invalid hexit found in string"),
            ColorErrorKind::InvalidFormat => write!(f, "string does not conform to hex format"),
        }
    }
}

#[derive(Clone, Copy)]
struct Color {
    data: [u8; 4],
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            data: [r, g, b, 0xff],
        }
    }

    fn inverted(&self) -> Self {
        Color::new(!self.data[0], !self.data[1], !self.data[2])
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    // TODO: Optimize
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || &s[0..1] != "#" {
            return Err(ParseColorError {
                kind: ColorErrorKind::InvalidFormat,
            });
        }

        let Ok(r) = u8::from_str_radix(&s[1..3], 16) else {
            return Err(ParseColorError {
                kind: ColorErrorKind::InvalidHexit,
            });
        };
        let Ok(g) = u8::from_str_radix(&s[3..5], 16) else {
            return Err(ParseColorError {
                kind: ColorErrorKind::InvalidHexit,
            });
        };
        let Ok(b) = u8::from_str_radix(&s[5..7], 16) else {
            return Err(ParseColorError {
                kind: ColorErrorKind::InvalidHexit,
            });
        };

        Ok(Color::new(r, g, b))
    }
}

impl Into<mq::Color> for Color {
    fn into(self) -> mq::Color {
        self.data.into()
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            self.data[0], self.data[1], self.data[2]
        )
    }
}

pub struct Ui {
    color: Color,
    input: String,
}

impl Ui {
    pub fn draw(&self) {
        // Left colored area
        draw_rectangle(
            0.0,
            0.0,
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT,
            self.color.into(),
        );

        // Right colored area
        draw_rectangle(
            WINDOW_WIDTH / 2.0,
            0.0,
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT,
            self.color.inverted().into(),
        );

        // Vertical line
        draw_line(
            WINDOW_WIDTH / 2.0,
            0.0,
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT,
            LINE_THICKNESS,
            LINE_COLOR,
        );

        // Entry border
        draw_rounded_rectangle(
            ENTRY_X - LINE_THICKNESS,
            ENTRY_Y - LINE_THICKNESS,
            ENTRY_WIDTH + LINE_THICKNESS * 2.0,
            ENTRY_HEIGHT + LINE_THICKNESS * 2.0,
            ENTRY_RADIUS + LINE_THICKNESS,
            LINE_COLOR,
        );

        // Entry field
        draw_rounded_rectangle(
            ENTRY_X,
            ENTRY_Y,
            ENTRY_WIDTH,
            ENTRY_HEIGHT,
            ENTRY_RADIUS,
            ENTRY_COLOR,
        );

        // Entry field text
        draw_text(
            self.input.as_str(),
            ENTRY_X + ENTRY_RADIUS,
            ENTRY_Y + ENTRY_HEIGHT - ENTRY_RADIUS,
            ENTRY_HEIGHT,
            TEXT_COLOR,
        );

        // Left color field border
        draw_rounded_rectangle(
            COLOR_HORIZONTAL_PADDING,
            COLOR_VERTICAL_PADDING,
            COLOR_WIDTH,
            COLOR_HEIGHT,
            COLOR_RADIUS,
            LINE_COLOR,
        );

        // Left color field
        draw_rounded_rectangle(
            COLOR_HORIZONTAL_PADDING + LINE_THICKNESS,
            COLOR_VERTICAL_PADDING + LINE_THICKNESS,
            COLOR_WIDTH - LINE_THICKNESS * 2.0,
            COLOR_HEIGHT - LINE_THICKNESS * 2.0,
            COLOR_RADIUS - LINE_THICKNESS,
            WHITE,
        );

        // Right color field border
        draw_rounded_rectangle(
            WINDOW_WIDTH / 2.0 + LINE_THICKNESS / 2.0 + COLOR_HORIZONTAL_PADDING,
            COLOR_VERTICAL_PADDING,
            COLOR_WIDTH,
            COLOR_HEIGHT,
            COLOR_RADIUS,
            LINE_COLOR,
        );

        // Right color field
        draw_rounded_rectangle(
            WINDOW_WIDTH / 2.0 + LINE_THICKNESS / 2.0 + COLOR_HORIZONTAL_PADDING + LINE_THICKNESS,
            COLOR_VERTICAL_PADDING + LINE_THICKNESS,
            COLOR_WIDTH - LINE_THICKNESS * 2.0,
            COLOR_HEIGHT - LINE_THICKNESS * 2.0,
            COLOR_RADIUS - LINE_THICKNESS,
            WHITE,
        )
    }

    pub fn update(&mut self) {
        if let Some(code) = get_last_key_pressed() {
            match code {
                KeyCode::Key0 => self.add_character('0'),
                KeyCode::Key1 => self.add_character('1'),
                KeyCode::Key2 => self.add_character('2'),
                KeyCode::Key3 => {
                    if is_key_down(KeyCode::LeftShift) {
                        self.add_character('#');
                    } else {
                        self.add_character('3');
                    }
                }
                KeyCode::Key4 => self.add_character('4'),
                KeyCode::Key5 => self.add_character('5'),
                KeyCode::Key6 => self.add_character('6'),
                KeyCode::Key7 => self.add_character('7'),
                KeyCode::Key8 => self.add_character('8'),
                KeyCode::Key9 => self.add_character('9'),
                KeyCode::A => self.add_character('a'),
                KeyCode::B => self.add_character('b'),
                KeyCode::C => self.add_character('c'),
                KeyCode::D => self.add_character('d'),
                KeyCode::E => self.add_character('e'),
                KeyCode::F => self.add_character('f'),
                KeyCode::Enter => {
                    if let Ok(color) = self.input.parse::<Color>() {
                        self.color = color;
                    }
                    self.input.clear();
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => (),
            };
        }
    }

    fn add_character(&mut self, ch: char) {
        if self.input.len() < 7 {
            self.input.push(ch);
        }
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            color: Color::new(0xff, 0x00, 0x00),
            input: String::new(),
        }
    }
}
