use macroquad::prelude::*;

#[macroquad::main("Wizard's Tower")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Welcome to the Wizard's Tower!", 20.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
