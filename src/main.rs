mod ui;

use macroquad::prelude::*;
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
    let mut ui = Ui::default();
    loop {
        clear_background(BLACK);

        ui.update();
        ui.draw();

        next_frame().await;
    }
}
