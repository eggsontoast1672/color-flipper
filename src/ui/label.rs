use macroquad::{color, prelude::*};

use crate::ui;

pub struct Label {
    position: Vec2,
    dimensions: Vec2,
}

impl Label {
    const BORDER_COLOR: color::Color = BLACK;
    const BORDER_THICKNESS: f32 = 5.0;
    const CORNER_RADIUS: f32 = 15.0;
    const FIELD_COLOR: color::Color = WHITE;

    pub fn new(position: Vec2, dimensions: Vec2) -> Self {
        Self {
            position,
            dimensions,
        }
    }

    pub fn draw(&self) {
        ui::draw_rounded_rectangle(
            self.position.x,
            self.position.y,
            self.dimensions.x,
            self.dimensions.y,
            Self::CORNER_RADIUS,
            Self::BORDER_COLOR,
        );
        ui::draw_rounded_rectangle(
            self.position.x + Self::BORDER_THICKNESS,
            self.position.y + Self::BORDER_THICKNESS,
            self.dimensions.x - Self::BORDER_THICKNESS * 2.0,
            self.dimensions.y - Self::BORDER_THICKNESS * 2.0,
            Self::CORNER_RADIUS - Self::BORDER_THICKNESS,
            Self::FIELD_COLOR,
        );
    }
}

pub struct Text {
    pub data: String,
    pub font_size: u16,
}

pub struct TextLabel {
    position: Vec2,
    text: Text,
}

impl TextLabel {
    const BORDER_COLOR: color::Color = BLACK;
    const BORDER_THICKNESS: f32 = 5.0;
    const CORNER_RADIUS: f32 = 15.0;
    const FIELD_COLOR: color::Color = WHITE;
    const PADDING: f32 = 10.0;
    const TEXT_COLOR: color::Color = BLACK;
    const TEXT_SCALE: f32 = 1.0;

    pub fn new(position: Vec2, text: Text) -> Self {
        Self { position, text }
    }

    pub fn draw(&self) {
        let text_dimensions = measure_text(
            self.text.data.as_str(),
            None,
            self.text.font_size,
            Self::TEXT_SCALE,
        );

        ui::draw_rounded_rectangle(
            self.position.x,
            self.position.y,
            text_dimensions.width + Self::PADDING * 2.0 + Self::BORDER_THICKNESS * 2.0,
            text_dimensions.height + Self::PADDING * 2.0 + Self::BORDER_THICKNESS * 2.0,
            Self::CORNER_RADIUS,
            Self::BORDER_COLOR,
        );
        ui::draw_rounded_rectangle(
            self.position.x + Self::BORDER_THICKNESS,
            self.position.y + Self::BORDER_THICKNESS,
            text_dimensions.width + Self::PADDING * 2.0,
            text_dimensions.height + Self::PADDING * 2.0,
            Self::CORNER_RADIUS - Self::BORDER_THICKNESS,
            Self::FIELD_COLOR,
        );

        draw_text(
            self.text.data.as_str(),
            self.position.x + Self::BORDER_THICKNESS + Self::PADDING,
            self.position.y + text_dimensions.height + Self::BORDER_THICKNESS + Self::PADDING,
            self.text.font_size as f32,
            Self::TEXT_COLOR,
        );
    }
}
