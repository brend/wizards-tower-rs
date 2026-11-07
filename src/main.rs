use macroquad::prelude::*;

const GAME_WIDTH: f32 = 320.0;
const GAME_HEIGHT: f32 = 180.0;
const SCALE: i32 = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "Wizard's Tower".to_owned(),
        window_width: GAME_WIDTH as i32 * SCALE,
        window_height: GAME_HEIGHT as i32 * SCALE,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    // Create the low-resolution drawing surface once.
    let target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
    target.texture.set_filter(FilterMode::Nearest);

    // Game coordinates run from (0, 0) at the top-left to (320, 180).
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT))
    };

    loop {
        game.update(get_frame_time());

        // 1. Draw the game at 320×180.
        set_camera(&camera);
        clear_background(BLACK);
        game.render();

        // 2. Draw the resulting texture across the window.
        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

struct Entity {
    texture: Texture2D,
    position: Vec2,
}

impl Entity {
    async fn new(texture_filename: &str, position: Vec2) -> Entity {
        let texture = load_texture(texture_filename)
            .await
            .expect("Failed to load asset");

        // Useful for crisp pixel art when scaled.
        texture.set_filter(FilterMode::Nearest);

        Entity { texture, position }
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }
}

#[derive(Default)]
struct Game {
    enemies: Vec<Entity>,
    projectiles: Vec<Entity>,
    background: Vec<Entity>,
}

impl Game {
    async fn new() -> Self {
        Self {
            enemies: vec![Entity::new("assets/Ork.png", vec2(100.0, 100.0)).await],
            background: vec![Entity::new("assets/Tower.png", vec2(100.0, 100.0)).await],
            ..Self::default()
        }
    }

    fn update(&mut self, dt: f32) {
        for e in self.background.iter_mut() {
            e.update(dt);
        }
        for e in self.enemies.iter_mut() {
            e.update(dt);
        }
        for e in self.projectiles.iter_mut() {
            e.update(dt);
        }
    }

    fn render(&self) {
        for e in &self.background {
            e.render();
        }
        for e in &self.enemies {
            e.render();
        }
        for e in &self.projectiles {
            e.render();
        }
    }
}
