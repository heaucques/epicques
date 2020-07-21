use ggez::*;
use graphics::{MeshBuilder, DrawMode};
use ggez::nalgebra::{Point2, Vector2};
use std::time::{Duration, Instant};
use std::cmp::Ordering;

enum Position {
    coordinate{x : f64, y : f64},
}
struct State {
    loaded_tiles: Vec<Position>,
    // movement: Movement,
    the_simulator: Simulator<Movement>,
    x: f64,
    y: f64,
}
struct Simulator<S: Simulatable> {
    simulator: S,
    last_time: Instant,
    simulated_time: Duration,
    total_time: Duration,
    ms_per_update: f64,
    lag: f64,
    i: u32,
    fps: u32,
}
struct Movement {
    // ctx: &'a Context,
    y_acceleration: f64,
    x_acceleration: f64,
    y_velocity: f64,
    x_velocity: f64,
    gravity: f64,
    on_ground: bool,
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
    is_jumping: bool,
    x_pos: f64,
    y_pos: f64,
}
trait Simulatable {
    fn simulate(&mut self, time : f64);
}

impl<S: Simulatable> Simulator<S> {
    fn new(simulator: S) -> Self {
        Self {
            simulator,
            last_time: Instant::now(),
            simulated_time: Duration::default(),
            total_time: Duration::default(),
            ms_per_update: 1.0 / 30.0 * 1_000_000_000.0,
            lag: 0.0,
            i: 0,
            fps: 30,
        }
    }
    fn update(&mut self, ctx : &Context) {
        self.lag += timer::delta(ctx).subsec_nanos() as f64;
        
        while self.lag as f64 >= self.ms_per_update {
            self.simulator.simulate(self.lag);
            self.lag -= self.ms_per_update;
            self.i += 1;
        }
        
        println!("{}", self.i);
        // let dt = timer::delta(ctx).as_secs_f64();
        // self.simulator.simulate(dt);
    }
}
impl Simulatable for Movement {
    fn simulate(&mut self, time : f64) {
        if self.w_pressed {
            self.y_acceleration += 1.0;
        }
        if self.s_pressed {
            self.y_acceleration -= 1.0;
        }
        self.y_velocity += self.y_acceleration;
        self.y_acceleration *= 0.5;
        self.y_velocity *= 0.5;
        self.y_pos *= 0.5;

        self.y_pos += self.y_velocity;

        // println!("test {}", self.y_acceleration);
    }
}
impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        self.the_simulator.simulator.w_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W);
        self.the_simulator.simulator.s_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S);
        self.the_simulator.simulator.a_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A);
        self.the_simulator.simulator.d_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D);
        self.the_simulator.update(ctx);
        

        let mut mesh_builder = MeshBuilder::new();
        let mut i = 0;

        let dt = timer::delta(ctx).subsec_nanos() as f64 / 1_000_000_000.0;
        if !timer::check_update_time(ctx, self.the_simulator.fps) {
            return Ok(())
        }
        for position in &mut self.loaded_tiles.iter_mut() {
            match position {
                // x and y are tile x and tile y respectively oops
                Position::coordinate{ref mut x, ref mut y} => {
                    let (canvas_x, canvas_y) = graphics::drawable_size(ctx);
                    // match *x {
                    //     x if x > (canvas_x / 2.0 - 5.0) as f64 || x + (10.0 as f64) < (canvas_x / 2.0 + 5.0) as f64 => {
                    //         // println!("matching {}, {}", *x, canvas_x);
                    //         match *y {
                    //             y if y < (canvas_y / 2.0 + 1.0) as f64 && y - (10.0 as f64) > (canvas_y / 2.0) as f64 => {
                    //                 self.dy = 0 as f64;
                    //             },
                    //             _ => {}
                    //         }
                    //     },
                    //     _ => {}
                    // }
                    
                    // let dy = match *y {
                    //     y if y < canvas_y as f64 / 2.0 => {
                    //         self.the_simulator.simulator.on_ground = true;
                    //         // println!("{}", y);
                    //         canvas_y as f64 / 2.0 - y
                    //     },
                    //     _ => {0.0},
                    // };
                        *x += self.the_simulator.simulator.x_pos;
                        *y += self.the_simulator.simulator.y_pos;


                    // scalable sizing of the tiles (width and height) per resolution would be nice
                    // x = &mut (*x + self.dx);
                    // y = &mut (*y + self.dy);
                    mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(*x as f32, *y as f32, 10.0, 10.0), graphics::WHITE);
                    // println!("x: {}, y: {}, i: {}", self.x, self.y, i);
                    // println!("{}, {}", x, y);
                }
            }
            i+=1;
        }
        self.x = 0.0;
        self.y = 0.0;

        mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(graphics::drawable_size(ctx).0 / 2.0, graphics::drawable_size(ctx).1 / 2.0, 10.0, 10.0), graphics::Color::new(1.0, 0.0, 0.0, 1.0));

        let mesh = mesh_builder.build(ctx)?;
        let param = graphics::DrawParam::new()
        .dest(mint::Point2 {
            x: 0.0,
            y: 0.0,
        });
        graphics::draw(ctx, &mesh, param,)?;
        graphics::present(ctx)?;
        
        // println!("fps: {}", timer::fps(ctx));
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
    
    let (mut ctx, mut event_loop) = ContextBuilder::new("scary", "author?").conf(c).build().unwrap();

    let mut tiles = Vec::new();
    for i in 0..100 {
        for j in 50..51 {
            tiles.push(Position::coordinate {
                x: (i * 10) as f64,
                y: (j * 10) as f64,
            });
        }
    }
    let mut state = State {
        the_simulator: Simulator::new(Movement {
            // ctx: &mut ctx,
            y_velocity: 0.0,
            x_velocity: 0.0,
            y_acceleration: 0.0,
            x_acceleration: 0.0,
            gravity: -0.025,
            on_ground: true,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            is_jumping: false,
            x_pos: 0.0,
            y_pos: 0.0,
        }),
        loaded_tiles: tiles,
        x: 0.0,
        y: 0.0,
    };

    event::run(&mut ctx, &mut event_loop, &mut state).unwrap();
}