// ignore this this is useless all of it

use ggez::*;
use graphics::Mesh;
// use std::time::{Duration, Instant};

extern crate rand;
// use rand::{thread_rng, Rng};

enum Shape {
    // Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect, graphics::Color),
}

struct State {
    // dt: std::time::Duration,
    tiles: Vec<Shape>,
    dx: f32,
    dy: f32,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // self.dt = timer::delta(ctx);
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
        for i in 0..self.tiles.len() {
            match self.tiles.get_mut(i).unwrap() {
                Shape::Rectangle(rect, _color) => {
                    rect.translate(mint::Vector2 {
                        x: self.dx,
                        y: self.dy,
                    })
                }
            };

        }
        self.dx = 0.0;
        self.dy = 0.0;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // println!("fps: {}", 1 as f32 / (self.dt.subsec_nanos() as f32 / 1_000_000_000 as f32));
        // for shape in &self.shapes {
        //     let mesh = match shape {
        //         &Shape::Rectangle(rect) => {
        //             Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?
        //         }
        //         &Shape::Circle(origin, radius) => {
        //             Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::WHITE)?
        //         }
        //     };
            
        //     graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // }

        for tile in &self.tiles {
            let mesh = match tile {
                &Shape::Rectangle(rect, color) => {
                    Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?
                }
            };
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width as f32, height as f32)).unwrap();
        graphics::present(ctx).expect("oof");
    }

    // fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
    //     match keycode {
    //         input::keyboard::KeyCode::A => {
    //             self.dx -= 10.0;
    //         }
    //         input::keyboard::KeyCode::D => {
    //             self.dx += 10.0;
    //         }
    //         input::keyboard::KeyCode::W => {
    //             self.dy -= 10.0;
    //         }
    //         input::keyboard::KeyCode::S => {
    //             self.dy += 10.0;
    //         }
    //         _ => {}
    //     }
    // }


}


pub fn main() {
    
    // for _ in 0..8 {
    //     if thread_rng().gen_range(0, 2) % 2 == 0 {
    //         shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
    //             thread_rng().gen_range(0.0, 800.0),
    //             thread_rng().gen_range(0.0, 600.0),
    //             thread_rng().gen_range(0.0, 800.0),
    //             thread_rng().gen_range(0.0, 600.0),
    //         )));
    //     } else {
    //         shapes.push(Shape::Circle(
    //             mint::Point2{
    //                 x: thread_rng().gen_range(0.0, 800.0),
    //                 y: thread_rng().gen_range(0.0, 600.0),
    //             },
    //             thread_rng().gen_range(0.0, 300.0),
    //         ));
    //     }
    // }


    
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
    
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "author?").conf(c).build().unwrap();

    let mut tiles = Vec::new();
    // graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(0.0, 0.0, 100.0, 100.0), graphics::WHITE)
    for i in 0..50 {
        for j in 0..50 {
            let color = if i % 2 == 0 {graphics::WHITE} else {graphics::Color::new(255.0, 0.0, 0.0, 100.0)};
            tiles.push(Shape::Rectangle(graphics::Rect::new((i * 10) as f32, (j * 10) as f32, 50.0, 50.0), color))
        }
    }
    // tiles.push(Shape::Rectangle(graphics::Rect::new(10.0, 10.0, 10.0, 10.0)));
    
    let state = &mut State{
        // dt: std::time::Duration::new(0, 0),
        tiles: tiles,
        dx: 0.0,
        dy: 0.0,
    };

    event::run(ctx, event_loop, state).unwrap();
}