use ggez::*;
use graphics::{MeshBuilder, DrawMode};
use std::collections::HashMap;
use std::time::Instant;
// use std::cmp::Ordering;

mod movement;
mod simulate;
mod blocks;
mod files;

type H = HashMap<Position, u64>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Position {
    Coordinate{x : i64, y : i64},
}

struct State {
    tiles: H,
    all_tiles: H,
    
    the_simulator: simulate::Simulator<movement::Movement>,
    size: f64,
    canvas_x: f32,
    canvas_y: f32,

    mesh_builder: MeshBuilder,
}


impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, self.the_simulator.fps) {
            // definitely should make this more compact
            self.update_keypress(ctx);
            self.the_simulator.update(&self.tiles);
        }

        println!("fps: {}", timer::fps(ctx));

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        self.draw_and_check_tiles();
        self.draw_debug_tiles();

        self.draw_player();

        let mesh = self.mesh_builder.build(ctx)?;
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
        self.canvas_x = graphics::drawable_size(ctx).0;
        self.canvas_y = graphics::drawable_size(ctx).1;
        graphics::present(ctx).expect("oof");
    }
}

impl State {
    fn update_keypress(&mut self, ctx: &mut Context) {
        let simulator = &mut self.the_simulator.simulator;
        // jump
        simulator.jump_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::C) || input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Space);

        // up
        simulator.up_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Up) || input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W);

        // down
        simulator.down_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Down) || input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S);

        // left
        simulator.left_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Left) ||
        input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A);

        // right
        simulator.right_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Right) ||
        input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D);

        // grab/climb
        simulator.grab_pressed = input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::V)
        || input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Z)
        || input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::LShift);
        
    }

    fn draw_and_check_tiles(&mut self) {
        self.mesh_builder = MeshBuilder::new();

        let (player_x, player_y, x_shift, y_shift) = (self.the_simulator.simulator.player_x, self.the_simulator.simulator.player_y, self.canvas_x as f64 / self.size / 2.0, self.canvas_y as f64 / self.size / 2.0);
        let (left_x_border, right_x_border) = ((player_x - x_shift).round() as i64, (player_x + x_shift).round() as i64);
        let (top_y_border, bottom_y_border) = ((player_y - y_shift).round() as i64, (player_y + y_shift).round() as i64);

        let mut tiles_to_remove = Vec::new();
        for (position, _thing) in &self.tiles {
            match position {
                Position::Coordinate {x, y} => {
                    let rect = graphics::Rect::new(
                        (*x as f64 * self.size) as f32 + self.canvas_x / 2.0 - (self.the_simulator.simulator.player_x * self.size) as f32 - self.size as f32 / 2.0,
                        (*y as f64 * self.size) as f32 + self.canvas_y / 2.0 - (self.the_simulator.simulator.player_y * self.size) as f32 - self.size as f32 / 2.0,
                        self.size as f32,
                        self.size as f32
                    );
                    self.mesh_builder.rectangle(DrawMode::fill(), rect, graphics::WHITE);

                    match (x, y) {
                        (x, _) if x < &left_x_border => {
                            tiles_to_remove.push(Position::Coordinate {
                                x: *x,
                                y: *y,
                            });
                        },
                        (x, _) if x > &right_x_border => {
                            tiles_to_remove.push(Position::Coordinate {
                                x: *x,
                                y: *y,
                            });
                        },
                        (_, y) if y < &top_y_border => {
                            tiles_to_remove.push(Position::Coordinate {
                                x: *x,
                                y: *y,
                            });
                        },
                        (_, y) if y > &bottom_y_border => {
                            tiles_to_remove.push(Position::Coordinate {
                                x: *x,
                                y: *y,
                            });
                        },
                        (_, _) => (),
                    }
                    if *x < left_x_border {
                    }
                }
            }
        }
        self.remove_offscreen_tiles(tiles_to_remove);
        self.load_onscreen_tiles();
    }

    fn draw_debug_tiles(&mut self) {
        for position in &self.the_simulator.simulator.temp_debug {
            match position {
                Position::Coordinate {x, y} => {
                    let uau_x = x;
                    let uau_y = y;
                    self.mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(
                        (*uau_x as f64 * self.size) as f32 + self.canvas_x / 2.0 as f32 - (self.the_simulator.simulator.player_x * self.size) as f32 - self.size as f32 / 2.0,
                        (*uau_y as f64 * self.size) as f32 + self.canvas_y / 2.0 as f32 - (self.the_simulator.simulator.player_y * self.size) as f32 - self.size as f32 / 2.0,
                        self.size as f32,
                        self.size as f32),
                        graphics::Color::new(0.0, 1.0, 0.0, 0.5)
                    );
                }
            }
        }
    }

    fn draw_player(&mut self) {
        self.mesh_builder.rectangle(DrawMode::fill(), graphics::Rect::new(
            self.canvas_x / 2.0 - self.size as f32 / 2.0, self.canvas_y / 2.0 - self.size as f32 / 2.0,
            self.size as f32, self.size as f32),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0)
        );
    }

    
    fn remove_offscreen_tiles(&mut self, tiles_to_remove : Vec<Position>) {
        for position in &tiles_to_remove {
            self.tiles.remove(position);
        }
    }
    
    fn load_onscreen_tiles(&mut self) {
        // lazier method, extremely inefficient and cuts fps by more than 50%
        // excess cycles from useless things, lags because it has to iterate through a couple thousand items, most useless

        // ok hopefully this is faster now
        let (player_x, player_y, x_shift, y_shift) = (self.the_simulator.simulator.player_x, self.the_simulator.simulator.player_y, self.canvas_x as f64 / self.size / 2.0, self.canvas_y as f64 / self.size / 2.0);
        let (left_x_border, right_x_border) = ((player_x - x_shift).round() as i64, (player_x + x_shift).round() as i64);
        // top and bottom y border values are shifted here since y axis is flipped
        let (top_y_border, bottom_y_border) = ((player_y - y_shift).round() as i64, (player_y + y_shift).round() as i64);

        // let (mut left_x_border, mut right_x_border, mut top_y_border, mut bottom_y_border) = (0, 0, 0, 0);
        let x_velocity = self.the_simulator.simulator.x_velocity;
        let y_velocity = self.the_simulator.simulator.y_velocity;
        
        let (mut x_start, mut y_start, mut x_end, mut y_end) = (0, 0, 0, 0);

        // self.the_simulator.simulator.temp_debug = Vec::new();

        match x_velocity {
            // _ if x_velocity == 0.0 => (),
            _ if x_velocity > 0.0 => {
                let x_shift = x_velocity.ceil() as i64;
                x_start = right_x_border;
                x_end = x_start + x_shift;
            },
            _ if x_velocity < 0.0 => {
                let x_shift = x_velocity.floor() as i64;
                x_start = left_x_border;
                x_end = x_start - x_shift;
            },
            _ => (),
        }

        match y_velocity {
            // _ if y_velocity == 0.0 => (),
            _ if y_velocity > 0.0 => {
                let y_shift = y_velocity.floor() as i64;
                y_start = bottom_y_border;
                y_end = y_start - y_shift;
            },
            _ if y_velocity < 0.0 => {
                let y_shift = y_velocity.ceil() as i64;
                y_start = top_y_border;
                y_end = y_start + y_shift;
            },
            _ => (),
        }
        
        if x_velocity != 0.0 {
            self.load_tiles_vertical(x_start, x_end);
        }
        if y_velocity != 0.0 {
            self.load_tiles_horizontal(y_start, y_end);
        }
    }

    fn load_tile(&mut self, possible_position : Position) {
        if self.all_tiles.contains_key(&possible_position) && !self.tiles.contains_key(&possible_position) {
            if let Some(r) = self.all_tiles.get_key_value(&possible_position) {
                match r {
                    (_pos, tile_type) => {
                        // println!("{}", tile_type);
                        match tile_type {
                            0 => (),
                            1 => {
                                let tile = *tile_type;
                                self.add(possible_position, tile);
                            },
                            _ => println!("this should not have gotten here oof, unsure what went wrong"),
                        }
                    }
                }
            }
        }
    }

    fn load_tiles_vertical(&mut self, x_start : i64, x_end : i64) {
        let (player_y, y_shift) = (self.the_simulator.simulator.player_y, self.canvas_y as f64 / self.size / 2.0);
        let (top_y_border, bottom_y_border) = ((player_y - y_shift).round() as i64, (player_y + y_shift).round() as i64);
        for i in x_start..=x_end {
            for j in top_y_border..=bottom_y_border {
                let possible_position = Position::new(i, j);
                // self.the_simulator.simulator.temp_debug.push(possible_position.to_owned());
                self.load_tile(possible_position);
            }
        }
    }

    fn load_tiles_horizontal(&mut self, y_start : i64, y_end : i64) {
        let (player_x, x_shift) = (self.the_simulator.simulator.player_x, self.canvas_x as f64 / self.size / 2.0);
        let (left_x_border, right_x_border) = ((player_x - x_shift).round() as i64, (player_x + x_shift).round() as i64);
        for i in left_x_border..=right_x_border {
            for j in y_start..=y_end {
                let possible_position = Position::new(i, j);
                // self.the_simulator.simulator.temp_debug.push(possible_position.to_owned());
                self.load_tile(possible_position);
            }
        }
    }

    fn add(&mut self, position : Position, tile_type : u64) {
        self.tiles.insert(position, tile_type);
    }

    fn add_to_master(&mut self, position : Position, tile_type : u64) {
        self.all_tiles.insert(position, tile_type);
    }

    fn load(&mut self) -> std::io::Result<()> {
        let mut loader = files::Info::new();
        loader.load()?;

        let (player_x, player_y, x_shift, y_shift) = (self.the_simulator.simulator.player_x, self.the_simulator.simulator.player_y, self.canvas_x as f64 / self.size / 2.0, self.canvas_y as f64 / self.size / 2.0);

        // increased borders by 1 to accommodate for the possibility that some wont initially show since i think it's because they dont take in account velocity like the others
        let (left_x_border, right_x_border) = ((player_x - x_shift).round() as i64 - 1, (player_x + x_shift).round() as i64 + 1);
        let (top_y_border, bottom_y_border) = ((player_y - y_shift).round() as i64 - 1, (player_y + y_shift).round() as i64 + 1);

        for (position, tile_type) in loader.tiles {
            match tile_type {
                0 => (),
                1 => {
                    let pos = Position::new(position.get_x(), position.get_y());
                    let pos_x = pos.get_x();
                    let pos_y = pos.get_y();
                    let within_bounds = |x : i64, y : i64| -> bool {if x > left_x_border && x < right_x_border && y > top_y_border && y < bottom_y_border {true} else {false}};
                    if within_bounds(pos_x, pos_y) {
                        self.add(pos.to_owned(), tile_type);
                    }
                    self.add_to_master(pos, tile_type);
                }
                _ => panic!("somethign went wrong oof lol"),
            }
        }
        Ok(())
    }
}

impl Position {
    pub fn get_xy(&self) -> (i64, i64) {
        #![allow(dead_code)]

        match self {
            Self::Coordinate {x, y} => {
                (*x, *y)
            }
        }
    }
    pub fn get_x(&self) -> i64 {
        #![allow(dead_code)]

        match self {
            Self::Coordinate {x, y: _} => {
                *x
            }
        }
    }
    pub fn get_y(&self) -> i64 {
        #![allow(dead_code)]

        match self {
            Self::Coordinate {x: _, y} => {
                *y
            }
        }
    }

    pub fn to_owned(&self) -> Self {
        match self {
            Self::Coordinate {x, y} => {
                Self::new(*x, *y)
            }
        }
    }
    pub fn new(x : i64, y: i64) -> Self {
        Self::Coordinate {
            x: x,
            y: y,
        }
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

    // q: why are these spaced out across so many lines?
    // a: ;-;

    let block_type = 1;
    for i in -20..=-15 {
        for j in 10..11 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type // i dont know what else to put here lol haha
         );
        }
    }
    
    for i in -10..=-5 {
        for j in 0..1 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
         );
        }
    }
    
    for i in 15..=20 {
        for j in 10..11 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
         );
        }
    }

    for i in 5..=10 {
        for j in 0..1 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in 10..=15 {
        for j in 5..6 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in -15..=-10 {
        for j in 5..6 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }
    
    for i in 0..1 {
        for j in -10..0 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in 0..1 {
        for j in 10..20 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in -20..20 {
        for j in 20..21 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in 10..=25 {
        for j in 15..16 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in -25..=-10 {
        for j in 15..16 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    for i in -5..=5 {
        for j in -5..-4 {
            tiles.insert(Position::Coordinate {
                x: i,
                y: j,
            },
            block_type
        );
        }
    }

    let mut all_tiles = HashMap::new();

    for key in tiles.keys() {
        match tiles.get_key_value(key).unwrap() {
            (position, i) => {
                match position {
                    Position::Coordinate {x, y} => {
                        all_tiles.insert(Position::Coordinate {
                            x: *x,
                            y: *y,
                        }, i.to_owned());
                    }
                }
            }
        }
    }

    let mut state = State {
        the_simulator: simulate::Simulator::new(movement::Movement {
            y_velocity: 0.0,
            x_velocity: 0.0,
            gravity: 0.025,

            on_ground: false,
            is_jumping: false,
            is_climbing: false,
            side_climbing: 0,

            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            jump_pressed: false,
            grab_pressed: false,

            can_jump: true,

            player_x: 0.0,
            player_y: 0.0,

            coyote_timer_instant: Instant::now(),
            coyote_timer_active: false,
            
            temp_debug: Vec::new(),

        }),
        all_tiles: all_tiles,
        tiles: tiles,
        size: graphics::drawable_size(&ctx).1 as f64 / 36.0,
        canvas_x: graphics::drawable_size(&ctx).0,
        canvas_y: graphics::drawable_size(&ctx).1,

        mesh_builder: MeshBuilder::new(),
    };

    state.load().expect("file loading failed");
    

    event::run(&mut ctx, &mut event_loop, &mut state).unwrap();
}