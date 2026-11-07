use macroquad::prelude::*;

#[macroquad::main("Wizard's Tower")]
async fn main() {
    let mut game = Game::default();
    
    loop {
        clear_background(BLACK);

        game.update();
        game.render();

        next_frame().await
    }
}

struct Game {
    frame_count: u32,
    title: String,
}

impl Game {
    fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count % 300 == 0 {
            self.title.push('!');
        }
    }

    fn render(&self) {
        draw_text(&self.title, 20.0, 20.0, 20.0, WHITE);
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            title: "Welcome to the Wizard's Tower!".to_string(),
            frame_count: 0,
        }
    }
}