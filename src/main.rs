mod game;

use macroquad::prelude::*;
use game::*;

const PATH_FONT: &str = "./assets/Pixeled.ttf";

#[macroquad::main("Pong")]
async fn main() {
    let font = load_ttf_font(PATH_FONT).await.unwrap();
    let mut state = State::new();

    set_camera(&Camera2D {
        zoom: vec2(1. / SCR_W * 2., -1. / SCR_H * 2.),
        target: screen_center(),
        ..Default::default()
    });

    loop {
        state.update();
        draw_frame(&state, Some(font));
        next_frame().await;
    }
}
