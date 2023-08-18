use macroquad::prelude as mq;
use macroquad::prelude::*;

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
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);

    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
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
        draw_rectangle(
            0.0,
            0.0,
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT,
            self.color.into(),
        );
        draw_rectangle(
            WINDOW_WIDTH / 2.0,
            0.0,
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT,
            self.color.inverted().into(),
        );
        draw_line(
            screen_width() / 2.0 - LINE_THICKNESS / 2.0,
            0.0,
            screen_width() / 2.0 - LINE_THICKNESS / 2.0,
            screen_height(),
            LINE_THICKNESS,
            LINE_COLOR,
        );
        draw_rounded_rectangle(
            ENTRY_X - LINE_THICKNESS,
            ENTRY_Y - LINE_THICKNESS,
            ENTRY_WIDTH + LINE_THICKNESS * 2.0,
            ENTRY_HEIGHT + LINE_THICKNESS * 2.0,
            ENTRY_RADIUS + LINE_THICKNESS,
            LINE_COLOR,
        );
        draw_rounded_rectangle(
            ENTRY_X,
            ENTRY_Y,
            ENTRY_WIDTH,
            ENTRY_HEIGHT,
            ENTRY_RADIUS,
            ENTRY_COLOR,
        );
        draw_text(
            self.input.as_str(),
            ENTRY_X + ENTRY_RADIUS,
            ENTRY_Y + ENTRY_HEIGHT - ENTRY_RADIUS,
            ENTRY_HEIGHT,
            TEXT_COLOR,
        );
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
                KeyCode::Backspace => self.remove_character(),
                _ => (),
            };
        }
    }

    fn add_character(&mut self, ch: char) {
        if self.input.len() < 7 {
            self.input.push(ch);
        }
    }

    fn remove_character(&mut self) {
        self.input.pop();
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