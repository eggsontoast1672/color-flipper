use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use macroquad::prelude as mq;

#[derive(Debug)]
enum ColorErrorKind {
    InvalidHexit,
    InvalidFormat,
}

#[derive(Debug)]
pub struct ParseColorError {
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
pub struct Color {
    data: [u8; 4],
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            data: [r, g, b, 0xff],
        }
    }

    pub fn inverted(&self) -> Self {
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
