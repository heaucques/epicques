use ggez::*;
use graphics::{MeshBuilder, DrawMode};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Position {
    Coordinate{x : i64, y : i64},
}

trait Simulatable {
    fn simulate(&mut self, tiles : &HashMap<Position, i64>);
    fn detect(&mut self, tiles : &HashMap<Position, i64>);
}

struct State {
    tiles: HashMap<Position, i64>,
    the_simulator: Simulator<Movement>,
    size: f64,
    x: f64,
    y: f64,
}

struct Simulator<S: Simulatable> {
    simulator: S,
    fps: u32,
}

struct Movement {
    y_velocity: f64,
    x_velocity: f64,
    dx: f64,
    dy: f64,
    gravity: f64,
    on_ground: bool,
    is_jumping: bool,

    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,

    player_x: f64,
    player_y: f64,
}

impl<S: Simulatable> Simulator<S> {
    fn new(simulator: S) -> Self {
        Self {
            simulator,
            fps: 60,
        }
    }
    fn update(&mut self, tiles : &HashMap<Position, i64>) {
        self.simulator.simulate(tiles);
    }
}

impl Simulatable for Movement {
    fn simulate(&mut self, tiles : &HashMap<Position, i64>) {
        // dont think this is optimal either since xvel and dx could be combined into one but whatever
        self.dx = 0.0;
        self.dy = 0.0;
        // self.y_velocity = 0.0;
        // self.x_velocity = 0.0;
        if self.w_pressed {
            self.y_velocity -= 0.3;
        }
        if self.s_pressed {
            self.y_velocity += 0.3;
        }
        if !self.on_ground {
        }
        if self.a_pressed {
            self.x_velocity -= 0.3;
            self.on_ground = false;
        }
        if self.d_pressed {
            self.x_velocity += 0.3;
            self.on_ground = false;
        }

        // put platform detection function here (before the y velocity calculations things) so that it doesnt have a flickering effect (hopefully) and the y_velocity value isnt wrong
        
        self.detect(tiles);
        self.dy += self.y_velocity;
        self.y_velocity *= 0.5;
        self.dx += self.x_velocity;
        self.x_velocity *= 0.5;
        if !self.on_ground {
        }
        // println!("{}, {}", self.player_x, self.player_y);
        // these stuffs aren't reversed to accommodate for the stationary player persons thats done later
        self.player_x += self.dx;
        self.player_y += self.dy;
    }
    fn detect(&mut self, tiles : &HashMap<Position, i64>) {
        // todo: make this not bad like
        
        // this is the purest form of sphaghetti ive ever seen
        
        // detect floor based of the blue sheep guy's idea that's similar to how minecraft works apparently?
        
        // relative to the player cube thingy
        let bottom_pos_x = self.player_x.floor() as i64;
        let top_pos_x = self.player_x.ceil() as i64;
        let bottom_pos_y = self.player_y.floor() as i64;
        let top_pos_y = self.player_y.ceil() as i64;


        // i think sometimes on the initial thwack into the wall it still jumps and idk how to fix that
        // OK ITS A FEATURE NOW
        
        // apparently i spent way too much time realizing that the bottom/top y check thingies are offset due to the player being 1 by 1
        let bottom_y : i64;
        let top_y : i64;
        let y_shift : i64;
        // welcome to the gateway to hell
        if self.y_velocity > 0.0 {
            bottom_y = (self.player_y - self.y_velocity).ceil() as i64;
            top_y = self.player_y.floor() as i64;
            y_shift = 1;

            // this is a TERRIBLE way of doing this
            // this has a lot of wasted lines since it could be condensed much more compactly (plus its then repeated another 3 times)
            for i in bottom_y..=top_y {
                if self.player_x.fract() == 0.0 {
                    let possible_position = Position::Coordinate {
                        x: self.player_x.round() as i64,
                        y: i + y_shift,
                    };
        
                    if tiles.contains_key(&possible_position) {
                        self.y_velocity = 0.0;
                        self.player_y = i as f64;
                        self.dy = self.y_velocity;
                        self.on_ground = true;
                        break;
                    }
                }

                // i dont believe these are needed here??
                if self.player_x == bottom_pos_x as f64 || self.player_x == top_pos_x as f64{
                    break;
                }
                
                let possible_position = Position::Coordinate {
                    x: bottom_pos_x,
                    y: i + y_shift,
                };
    
                // println!("test {}", i);
                if tiles.contains_key(&possible_position) {
                    self.y_velocity = 0.0;
                    // oh
                    self.player_y = i as f64;
                    // self.player_y = bottom_y as f64 - self.player_y;
                    self.dy = self.y_velocity;
                    self.on_ground = true;
                    break;
                }
                let possible_position = Position::Coordinate {
                    x: top_pos_x,
                    y: i + y_shift,
                };
    
                // println!("test {}", i);
                if tiles.contains_key(&possible_position) {
                    self.y_velocity = 0.0;
                    // oh
                    self.player_y = i as f64;
                    // self.player_y = bottom_y as f64 - self.player_y;
                    self.dy = self.y_velocity;
                    self.on_ground = true;
                    break;
                }
            }
            
        } else if self.y_velocity < 0.0 {
            bottom_y = self.player_y.ceil() as i64;
            top_y = (self.player_y - self.y_velocity).floor() as i64;
            y_shift = -1;

            for i in bottom_y..=top_y {
                if self.player_x.fract() == 0.0 {
                    let possible_position = Position::Coordinate {
                        x: self.player_x.round() as i64,
                        y: i + y_shift,
                    };
        
                    if tiles.contains_key(&possible_position) {
                        self.y_velocity = 0.0;
                        self.player_y = i as f64;
                        self.dy = self.y_velocity;
                        self.on_ground = true;
                        break;
                    }
                }

                if self.player_x == bottom_pos_x as f64 || self.player_x == top_pos_x as f64{
                    break;
                }

                let possible_position = Position::Coordinate {
                    x: bottom_pos_x,
                    y: i + y_shift,
                };
    
                if tiles.contains_key(&possible_position) {
                    self.y_velocity = 0.0;
                    // oh
                    self.player_y = i as f64;
                    self.dy = self.y_velocity;
                    self.on_ground = true;
                    break;
                }
                let possible_position = Position::Coordinate {
                    x: top_pos_x,
                    y: i + y_shift,
                };
    
                // println!("test {}", i);
                if tiles.contains_key(&possible_position) {
                    self.y_velocity = 0.0;
                    // oh
                    self.player_y = i as f64;
                    self.dy = self.y_velocity;
                    self.on_ground = true;
                    break;
                }
            }
        }

        let right_x : i64;
        let left_x : i64;
        let x_shift : i64;
        if self.x_velocity > 0.0 {
            right_x = (self.player_x + self.x_velocity).floor() as i64;
            left_x = self.player_x.floor() as i64;
            x_shift = 1;

            // println!("{}", self.x_velocity);
            for i in left_x..=right_x {
                if self.player_y.fract() == 0.0 {
                    let possible_position = Position::Coordinate {
                        x: i + x_shift,
                        y: self.player_y.round() as i64,
                    };
                    if tiles.contains_key(&possible_position) {
                        self.x_velocity = 0.0;
                        self.player_x = i as f64;
                        self.dx = self.x_velocity;
                        break;
                    }
                }
                
                if self.player_y == bottom_pos_y as f64 || self.player_y == top_pos_y as f64{
                    break;
                }

                let possible_position = Position::Coordinate {
                    x: i + x_shift,
                    y: top_pos_y,
                };

                if tiles.contains_key(&possible_position) {
                    self.x_velocity = 0.0;
                    self.player_x = i as f64;
                    self.dx = self.x_velocity;
                    break;
                }

                let possible_position = Position::Coordinate {
                    x: i + x_shift,
                    y: bottom_pos_y,
                };
                
                if tiles.contains_key(&possible_position) {
                    self.x_velocity = 0.0;
                    self.player_x = i as f64;
                    self.dx = self.x_velocity;
                    break;
                }
            }
        } else if self.x_velocity < 0.0 {
            right_x = self.player_x.ceil() as i64;
            left_x = (self.player_x + self.x_velocity).ceil() as i64;
            x_shift = -1;

            for i in left_x..=right_x {
                if self.player_y.fract() == 0.0 {
                    let possible_position = Position::Coordinate {
                        x: i + x_shift,
                        y: self.player_y.round() as i64,
                    };
                    if tiles.contains_key(&possible_position) {
                        self.x_velocity = 0.0;
                        self.player_x = i as f64;
                        self.dx = self.x_velocity;
                        break;
                    }
                }

                if self.player_y == bottom_pos_y as f64 || self.player_y == top_pos_y as f64{
                    break;
                }

                let possible_position = Position::Coordinate {
                    x: i + x_shift,
                    y: top_pos_y,
                };
                if tiles.contains_key(&possible_position) {
                    self.x_velocity = 0.0;
                    self.player_x = i as f64;
                    self.dx = self.x_velocity;
                    break;
                }
                let possible_position = Position::Coordinate {
                    x: i + x_shift,
                    y: bottom_pos_y,
                };
                if tiles.contains_key(&possible_position) {
                    self.x_velocity = 0.0;
                    self.player_x = i as f64;
                    self.dx = self.x_velocity;
                    break;
                }
            }
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, self.the_simulator.fps) {
            // definitely should make this more compact
            self.the_simulator.simulator.w_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W);
            self.the_simulator.simulator.s_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S);
            self.the_simulator.simulator.a_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A);
            self.the_simulator.simulator.d_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D);
            self.the_simulator.update(&self.tiles);
        }
                println!("fps: {}", timer::fps(ctx));

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        let mut mesh_builder = MeshBuilder::new();
        
        let (canvas_x, canvas_y) = graphics::drawable_size(ctx);

        for (position, _thing) in &self.tiles {
            match position {
                Position::Coordinate {x, y} => {
                    let rect = graphics::Rect::new(
                        (*x as f64 * self.size) as f32 + canvas_x / 2.0 - (self.the_simulator.simulator.player_x * self.size) as f32,
                        (*y as f64 * self.size) as f32 + canvas_y / 2.0 - (self.the_simulator.simulator.player_y * self.size) as f32,
                        self.size as f32,
                        self.size as f32,
                    );
                    mesh_builder.rectangle(DrawMode::fill(), rect, graphics::WHITE);
                }
            }
        }


        


        // mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new((self.the_simulator.simulator.player_x * self.size) as f32 + canvas_x / 2.0, (self.the_simulator.simulator.player_y * self.size) as f32 + canvas_y / 2.0, self.size as f32, self.size as f32), graphics::Color::new(1.0, 0.0, 0.0, 1.0));
        mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(canvas_x / 2.0, canvas_y / 2.0, self.size as f32, self.size as f32), graphics::Color::new(1.0, 0.0, 0.0, 1.0));

        let mesh = mesh_builder.build(ctx)?;
        // lol what if this was used for a screen shake effect
        let param = graphics::DrawParam::new()
        .dest(mint::Point2 {
            x: 0.0,
            y: 0.0,
        });
        graphics::draw(ctx, &mesh, param,)?;
        graphics::present(ctx)?;
        
        Ok(())
    }
    
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width as f32, height as f32)).unwrap();
        self.size = height as f64 / 36.0;
        graphics::present(ctx).expect("oof");
    }
}


fn main() {
    // conf stuff are configurations
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
        // vsync here
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

    
    let mut tiles = HashMap::new();
    
    for i in -20..-10 {
        for j in 10..11 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0  // i dont know what else to put here lol haha
         );
        }
    }
    
    for i in -20..-10 {
        for j in -11..-10 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
         );
        }
    }
    
    for i in 10..20 {
        for j in 10..11 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
         );
        }
    }

    for i in 10..20 {
        for j in -11..-10 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
        );
        }
    }

    for i in 10..20 {
        for j in 0..1 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
        );
        }
    }

    for i in -20..-10 {
        for j in 0..1 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
        );
        }
    }
    
    for i in 0..1 {
        for j in -20..-10 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
        );
        }
    }

    for i in 0..1 {
        for j in 10..20 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            0
        );
        }
    }
    // basic test level here
    // utilizes tile.push(Position::Coordinate {x: this, y: that})
    
    // let mut tiles = Vec::new();

    // for i in 0..10 {
    //     for j in 0..1 {
    //         tiles.push(Position::Coordinate {
    //             x: i,
    //             y: j,
    //         })
    //     }
    // }

    // btw the thing above thats commented out is DEAD
    let mut state = State {
        the_simulator: Simulator::new(Movement {
            y_velocity: 0.0,
            x_velocity: 0.0,
            gravity: -0.5,
            on_ground: false,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            is_jumping: false,
            dx: 0.0,
            dy: 0.0,
            player_x: 0.0,
            player_y: 0.0,
        }),
        tiles: tiles,
        size: graphics::drawable_size(&ctx).1 as f64 / 36.0,
        x: 0.0,
        y: 0.0,
    };

    event::run(&mut ctx, &mut event_loop, &mut state).unwrap();
}