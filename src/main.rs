mod color;
mod ui;

use macroquad::prelude::*;
use macroquad::ui::widgets::Window;

use ui::Ui;

fn window_conf() -> Conf {
    Conf {
        window_width: ui::WINDOW_WIDTH as i32,
        window_height: ui::WINDOW_HEIGHT as i32,
        window_resizable: false,
        window_title: "Color Flipper".into(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut myui = Ui::default();
    loop {
        clear_background(BLACK);

        myui.update();
        myui.draw();

        Window::new(0, Vec2::new(100.0, 100.0), Vec2::new(100.0, 100.0))
            .label("Label")
            .titlebar(true)
            .ui(&mut *macroquad::ui::root_ui(), |_| {});

        next_frame().await;
    }
}
