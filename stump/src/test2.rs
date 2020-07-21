use ggez::*;
use graphics::{MeshBuilder, DrawMode};

enum Position {
    Coordinate{x : f64, y : f64},
}
struct State {
    loaded_tiles: Vec<Position>,
    // movement: Movement,
    the_simulator: Simulator<Movement>,
    x: f64,
    y: f64,
    scale: f64,
    shift: f64,
}
struct Simulator<S: Simulatable> {
    simulator: S,
    fps: u32,
}
struct Movement {
    // ctx: &'a Context,
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
    fn simulate(&mut self);
}

impl<S: Simulatable> Simulator<S> {
    fn new(simulator: S) -> Self {
        Self {
            simulator,
            fps: 60,
        }
    }
    fn update(&mut self, ctx : &Context) {
        self.simulator.simulate();
    }
}
impl Simulatable for Movement {
    fn simulate(&mut self) {
        let mut thing : f64 = 0.7;
        if self.on_ground {
            self.y_pos = 0.0;
            self.y_velocity = 0.0;
            if self.w_pressed {
                self.y_velocity = 10.0;
                // :(
                self.y_pos += 10.0;
                self.on_ground = false;
                self.is_jumping = true;
            }
            if self.a_pressed && self.x_velocity < 10.0 {
                self.x_velocity += 2.0;
            }
            if self.d_pressed && self.x_velocity > -10.0 {
                self.x_velocity -= 2.0;
            }

        } else {
            if self.w_pressed && self.is_jumping {
                thing = 0.9;
            }
            if self.y_velocity < 0.0 {
                self.is_jumping = false;
            }
            if self.a_pressed && self.x_velocity < 10.0 {
                self.x_velocity += 0.5;
            }
            if self.d_pressed && self.x_velocity > -10.0 {
                self.x_velocity -= 0.5;
            }
            if self.y_velocity > -10.0 {
                self.y_pos += thing * self.y_velocity;
                self.y_velocity += self.gravity;
                self.y_pos += thing * self.y_velocity;
            } else {
                self.y_pos += self.y_velocity;
            }
        }

        if self.on_ground {
            self.x_velocity *= 0.85;
        } else {
            self.x_velocity *= 0.95;
        }
        
        self.x_pos = self.x_velocity;

                // if self.a_pressed {
        //     self.x_acceleration += 10.0;
        // }
        // if self.d_pressed {
        //     self.x_acceleration -= 10.0;
        // }

        // self.x_acceleration *= 0.8;
        // self.x_velocity *= 0.5;

        // self.x_velocity += self.x_acceleration;
    }
}
impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, self.the_simulator.fps) {
            self.the_simulator.simulator.w_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W);
            self.the_simulator.simulator.s_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S);
            self.the_simulator.simulator.a_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A);
            self.the_simulator.simulator.d_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D);
            self.the_simulator.update(ctx);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        let mut mesh_builder = MeshBuilder::new();
        let mut position_thingies = Vec::new();
        self.shift = 0.0;
        let mut pls_move = 0.0;
        let mut is_there_a_thing_under_the_player_persons = false;
        for position in &mut self.loaded_tiles.iter_mut() {
            match position {
                Position::Coordinate{ref mut x, ref mut y} => {
                    let (canvas_x, canvas_y) = graphics::drawable_size(ctx);
                    *y += self.shift;
                    // lol
                    pls_move = match (*y, *x) {
                        (y, x) if y < canvas_y as f64 / 2.0 + 10.0 && y > canvas_y as f64 / 2.0 - 10.0 && x > canvas_x as f64 / 2.0 - 5.0 * self.scale && x < canvas_x as f64 / 2.0 + 5.0 * self.scale => {
                            self.the_simulator.simulator.on_ground = true;
                            canvas_y as f64 / 2.0 - y
                        },
                        _ => {
                            0.0
                        },
                    };
                    if *x > canvas_x as f64 / 2.0 - 5.0 * self.scale && *x < canvas_x as f64 / 2.0 + 5.0 * self.scale {
                        if *y < canvas_y as f64 / 2.0 + 10.0 && *y > canvas_y as f64 / 2.0 - 10.0 && !is_there_a_thing_under_the_player_persons {
                            is_there_a_thing_under_the_player_persons = true;
                        }
                    }
                    // println!("{}", pls_move);
                    // println!("{}", canvas_y);
                    *x += self.the_simulator.simulator.x_pos;
                    *y += self.the_simulator.simulator.y_pos;
                    // *y += self.the_simulator.simulator.y_pos + 0.5 * (self.scale - 1.0 ) * dy;
                    
                    // (*y - canvas_y as f64 / 2.0) * self.scale + canvas_y as f64 / 2.0;

                    
                    // mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(((*x - canvas_x as f64 / 2.0) * self.scale + canvas_x as f64 / 2.0) as f32, ((*y - canvas_y as f64 / 2.0) * self.scale + canvas_y as f64 / 2.0) as f32, (10.0 * self.scale) as f32, (10.0 * self.scale) as f32), graphics::WHITE);
                    
                    
                    // mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(((*x - 0.5 * (self.scale) * 10.0)) as f32, ((*y - 0.5 * (self.scale) * 10.0)) as f32, 10.0 * self.scale as f32, 10.0 * self.scale as f32), graphics::WHITE);

                    // mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(*x as f32, (*y + &self.shift) as f32, 10.0 as f32, 10.0 as f32), graphics::WHITE);

                    
                    position_thingies.push(Position::Coordinate {
                        x: *x,
                        y: *y,
                    });
                    // if *y < canvas_y as f64 / 2.0 {
                    //     *y += 0.1;
                    // }
                }
            }
        }
        for thing in position_thingies {
            match thing {
                Position::Coordinate{x, y} => {
                    mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(x as f32, (y + &pls_move) as f32, 10.0 as f32, 10.0 as f32), graphics::WHITE);
                }
            }
        }
        self.shift = pls_move;
        if !is_there_a_thing_under_the_player_persons {
            self.the_simulator.simulator.on_ground = false;
        }
        self.the_simulator.simulator.x_pos = 0.0;
        self.the_simulator.simulator.y_pos = 0.0;
        self.x = 0.0;
        self.y = 0.0;

        // mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(graphics::drawable_size(ctx).0 / 2.0, graphics::drawable_size(ctx).1 / 2.0 - 10.0, 10.0 * self.scale as f32, 10.0 * self.scale as f32), graphics::Color::new(1.0, 0.0, 0.0, 1.0));
        mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(graphics::drawable_size(ctx).0 / 2.0, graphics::drawable_size(ctx).1 / 2.0 - 10.0, 10.0 as f32, 10.0 as f32), graphics::Color::new(1.0, 0.0, 0.0, 1.0));

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
        self.scale = height as f64 / 720.0;
        // meh
        // self.the_simulator.simulator.y_pos = -10.0;
        // println!("{}", self.scale);
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
            tiles.push(Position::Coordinate {
                x: (i * 10) as f64,
                y: (j * 10) as f64,
            });
        }
    }
    for i in 100..130 {
        for j in 40..41 {
            tiles.push(Position::Coordinate {
                x: (i * 10) as f64,
                y: (j * 10) as f64,
            })
        }
    }
    tiles.push(Position::Coordinate {
        x: 0.0 as f64,
        y: 10.0 as f64,
    });
    let mut state = State {
        the_simulator: Simulator::new(Movement {
            // ctx: &mut ctx,
            y_velocity: 0.0,
            x_velocity: 0.0,
            gravity: -0.5,
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
        scale: 1.0,
        shift: 0.0,
    };

    event::run(&mut ctx, &mut event_loop, &mut state).unwrap();
}