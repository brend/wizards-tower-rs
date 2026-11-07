use macroquad::prelude::*;

#[macroquad::main("Wizard's Tower")]
async fn main() {
    let mut game = Game::default();
    
    loop {
        let dt = macroquad::time::get_frame_time();

        clear_background(BLACK);

        game.update(dt);
        game.render();

        next_frame().await
    }
}

struct Game {
    elapsed: f32,
}

impl Game {
    fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    fn render(&self) {
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed: 0.0,
        }
    }
}