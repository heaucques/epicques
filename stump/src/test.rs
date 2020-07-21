use ggez::*;
// use graphics::Mesh;

enum Position {
    coordinate(f32, f32),
}
struct State {
    loaded_tiles: Vec<Position>,
    spritebatch: graphics::spritebatch::SpriteBatch,
    dx: f32,
    dy: f32,
}
impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W) {
            self.dy -= 10.0;
        }
        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S) {
            self.dy += 10.0;
        }
        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A) {
            self.dx -= 10.0;
        }
        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D) {
            self.dx += 10.0;
        }

        for position in &self.loaded_tiles {
            let p = match position {
                Position::coordinate(x, y) => {
                    graphics::DrawParam::new()
                    .dest(mint::Point2 {
                        x: *x + self.dx,
                        y: *y + self.dy,
                    })

                }
            };
            self.spritebatch.add(p);
        }
        let param = graphics::DrawParam::new()
        .dest(mint::Point2 {
            x: 0.0,
            y: 0.0,
        });
        graphics::draw(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        Ok(())
    }
    
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width as f32, height as f32)).unwrap();
        graphics::present(ctx).expect("oof");
    }
}
fn main() {
    let window_mode = conf::WindowMode {
        width: 1280.0,
        height: 720.0,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    };

    let window_setup = conf::WindowSetup {
        title: "stump".to_owned(),
        samples: conf::NumSamples::Four,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    };
    let c = conf::Conf {
        window_mode: window_mode,
        window_setup: window_setup,
        backend: conf::Backend::default(),
        modules: conf::ModuleConf::default(),
    };
    
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("spritebatch", "author?").conf(c).build().unwrap();

    let mut tiles = Vec::new();
    for i in 0..50 {
        for j in 0..50 {
            tiles.push(Position::coordinate((i * 10) as f32, (j * 10) as f32));
        }
    }
    let state = &mut State {
        loaded_tiles: tiles,
        spritebatch: graphics::spritebatch::SpriteBatch::new(graphics::Image::solid(ctx, 10, graphics::WHITE).unwrap()),
        dx: 0.0,
        dy: 0.0,
    };
    
    event::run(ctx, event_loop, state).unwrap();
}