use crate::movement;
use std::collections::HashMap;
use std::time::Instant;
// use std::time::Duration;

type H<'a> = &'a HashMap<super::Position, u64>;

pub trait Simulatable {
    fn simulate(&mut self, tiles : H);
    fn detect_ground(&mut self, tiles : H);
    fn detect_wall(&mut self, tiles : H);
    fn detect_falling(&mut self, tiles : H);
    fn detect_not_climbing(&mut self, tiles : H);
    fn detect_climbing(&mut self, tiles : H);
    fn update_movement(&mut self, _tiles : H);
    fn update_climbing(&mut self);
    fn update_motion(&mut self);
    fn update_can_jump(&mut self);
    fn wall_jump(&mut self);
    fn detect_wall_jumpable_walls(&mut self, tiles: H);
    fn vamonos(&mut self);
}

pub struct Simulator<S: Simulatable> {
    pub simulator: S,
    pub fps: u32,
}

impl<S: Simulatable> Simulator<S> {
    pub fn new(simulator: S) -> Self {
        Self {
            simulator,
            fps: 60,
        }
    }
    pub fn update(&mut self, tiles : H) {
        self.simulator.simulate(tiles);
    }
}

impl Simulatable for movement::Movement {
    fn simulate(&mut self, tiles : H) {
        // println!("{}, {}", self.player_x, self.player_y);
        self.update_movement(tiles);
        self.detect_ground(tiles);
        self.detect_wall(tiles);
        self.detect_falling(tiles);
        self.detect_climbing(tiles);
        self.detect_not_climbing(tiles);
        // self.detect_wall_jumpable_walls(tiles);
        self.vamonos();
    }
    fn detect_ground(&mut self, tiles : H) {        
        // self.temp_debug = Vec::new();
        
        // detect y axis

        let left_pos_x = self.player_x.round() as i64;

        // -1 = rounded down
        // 0 = rounded up
        // 1 = didnt round
        let round = if (left_pos_x as f64 - self.player_x).fract() == 0.0 {1} else {(left_pos_x as f64 - self.player_x).floor() as i64};

        let (mut x_start, mut x_end, mut y_start, mut y_end) = (0, 0, 0 ,0);

        if round == -1 {
            x_end += 1;
        } else if round == 0 {
            x_start -= 1;
        }
        
        if self.y_velocity.floor() > 0.0 {
            y_end += self.y_velocity.floor() as i64;
        } else if self.y_velocity < 0.0 {
            y_start += self.y_velocity.ceil() as i64;
        }
        
        for j in y_start..=y_end {
            let neg = self.y_velocity < 0.0;

            let future_position = if neg {
                (self.player_y + self.y_velocity).floor() + j as f64
            } else {
                (self.player_y + self.y_velocity).ceil() + j as f64
            };

            for i in x_start..=x_end {
                let possible_position = super::Position::Coordinate {
                    x: left_pos_x + i,
                    y: future_position as i64,
                };
                
                // self.temp_debug.push(super::Position::Coordinate {
                //     x: left_pos_x + i,
                //     y: future_position as i64,
                // });
                
                if tiles.contains_key(&possible_position) {
                    // im just gonna hope this works
                    self.on_ground = self.y_velocity > 0.0;

                    self.y_velocity = 0.0;
                    self.player_y = if neg {future_position + 1.0} else {future_position - 1.0};
                    break;
                }
            }
        }
    }
    fn detect_wall(&mut self, tiles : H) {
        // wont properly detect if falling downwards at high speeds
        
        // self.temp_debug = Vec::new();
        
        // detect x axis

        let (mut x_start, mut x_end, mut y_start, mut y_end) = (0, 0, 0, 0);
        
        let top_pos_y = self.player_y.round() as i64;
        
        let round = if (top_pos_y as f64 - self.player_y).fract() == 0.0 {1} else {(top_pos_y as f64 - self.player_y).floor() as i32};

        if round == -1 {
            y_end += 1;
        } else if round == 0 {
            y_start -= 1;
        }
        
        if self.x_velocity.floor() > 0.0 {
            x_end += self.x_velocity.floor() as i64;
        } else if self.x_velocity < 0.0 {
            x_start += self.x_velocity.ceil() as i64;
        }

        for j in x_start..=x_end {
            let neg = self.x_velocity < 0.0;

            let future_position = if neg {
                (self.player_x + self.x_velocity).floor() + j as f64
            } else {
                (self.player_x + self.x_velocity).ceil() + j as f64
            };

            for i in y_start..=y_end {
                let possible_position = super::Position::Coordinate {
                    x: future_position as i64,
                    y: top_pos_y + i,
                };
                
                // self.temp_debug.push(super::Position::Coordinate {
                //     x: future_position as i64,
                //     y: top_pos_y + i,
                // });
                
                if tiles.contains_key(&possible_position) {
                    self.x_velocity = 0.0;
                    self.player_x = if neg {future_position + 1.0} else {future_position - 1.0};
                    self.side_climbing = if neg {1} else {-1};
                    break;
                }
            }
        }
    }
    fn detect_falling(&mut self, tiles: H) {
        if !self.on_ground {
            return
        }
        let pos_x = self.player_x.round() as i64;
        
        // self.temp_debug = Vec::new();

        // -1 = rounded down
        // 0 = rounded up
        // 1 = didnt round
        let round = if (pos_x as f64 - self.player_x).fract() == 0.0 {1} else {(pos_x as f64 - self.player_x).floor() as i32};

        let (mut x_start, mut x_end) = (0, 0);

        if round == -1 {
            x_end += 1;
        } else if round == 0 {
            x_start -= 1;
        }

        // i keep on forgetting downwards is in the +y direction
        let pos_y = if self.player_y < 0.0 {self.player_y.floor() as i64 + 1} else {self.player_y.ceil() as i64 + 1};
        let mut is_there_ground = false;

        for i in x_start..=x_end {
            let possible_position = super::Position::Coordinate {
                x: pos_x + i,
                y: pos_y,
            };
            
            // self.temp_debug.push(super::Position::Coordinate {
            //     x: pos_x + i,
            //     y: pos_y,
            // });

            if tiles.contains_key(&possible_position) {
                is_there_ground = true;
            }
        }
        
        if self.on_ground {
            self.on_ground = is_there_ground;
        }
    }
    fn detect_not_climbing(&mut self, tiles: H) {
        if !self.is_climbing {
            return
        }
        
        // all based off of the assumption that climbing is only allowed when one hugs the wall completely, no gaps in between


        // separated in case needed to comment out later
        if !self.grab_pressed {
            self.is_climbing = false;
            self.on_ground = false;
            return
        }

        if self.x_velocity != 0.0 {
            self.is_climbing = false;
            self.on_ground = false;
            // self.side_climbing = 0;
            return
        }
        
        
        // i dont even think the rest of this is needed lolol
        
        // let y_pos = if self.player_y < 0.0 {self.player_y.floor() as i64} else {self.player_y.ceil() as i64};
        let y_pos = self.player_y.round() as i64;
        // let x_pos = if self.player_x < 0.0 {self.player_x.floor() as i64} else {self.player_x.ceil() as i64};
        let x_pos = self.player_x.round() as i64;
        
        
        let (mut x_start, mut x_end, mut y_start, mut y_end) = (0, 0, 0, 0);
        let round = if (x_pos as f64 - self.player_x).fract() == 0.0 {1} else {(x_pos as f64 - self.player_x).floor() as i32};
        
        
        if round == -1 {
            x_end += 1;
        } else if round == 0 {
            x_start -= 1;
        }
        
        let round = if (y_pos as f64 - self.player_y).fract() == 0.0 {1} else {(y_pos as f64 - self.player_y).floor() as i32};
        
        if round == -1 {
            y_end += 1;
        } else if round == 0 {
            y_start -= 1;
        }
        // self.temp_debug = Vec::new();

        let mut climbing = false;
        for j in y_start..=y_end {
            for i in x_start-1..=x_end+1 {
                let possible_position = super::Position::Coordinate {
                    x: x_pos + i,
                    y: y_pos + j,
                };
                // self.temp_debug.push(super::Position::Coordinate {
                    // x: x_pos + i,
                    // y: y_pos + j,
                // });
                if tiles.contains_key(&possible_position) {
                    climbing = true;
                }
            }
        }
        self.is_climbing = climbing;
        self.side_climbing = if !climbing {0} else {self.side_climbing};
    }

    fn detect_climbing(&mut self, tiles: H) {
        if self.is_climbing || !self.grab_pressed {
            return
        }
        let player_x = self.player_x;
        let player_y = self.player_y;

        let calc_shift = |y : f64| -> i64 {if y.fract() == 0.0 {0} else {1}};
        let y_shift = calc_shift(player_y);
        
        // self.temp_debug = Vec::new();

        if player_x.fract() == 0.0 {
            for i in (-1..=1).step_by(2) {
                // the y axis coverage isnt the most accurate compared to other implementations but it'll do
                for j in 0..=y_shift {
                    let pos = super::Position::new(player_x as i64 + i, player_y.floor() as i64 + j);
                    // self.temp_debug.push(pos.to_owned());
                    if tiles.contains_key(&pos) {
                        self.is_climbing = true;
                        return
                    }
                }
            }
        }
    }

    fn detect_wall_jumpable_walls(&mut self, tiles: H) {
        // essentially the same code as detect_climbing()
        // not technically ideal performance wise but it should only have a minimal impact

        if !self.can_jump {
            return
        }
        
        let player_x = self.player_x;
        let player_y = self.player_y;

        let calc_shift = |y : f64| -> i64 {if y.fract() == 0.0 {0} else {1}};
        let y_shift = calc_shift(player_y);
        
        // self.temp_debug = Vec::new();

        if player_x.fract() == 0.0 {
            for i in (-1..=1).step_by(2) {
                // the y axis coverage isnt the most accurate compared to other implementations but it'll do
                for j in 0..=y_shift {
                    let pos_x = player_x as i64 + i;
                    let pos_y = player_y.floor() as i64 + j;
                    let pos = super::Position::new(pos_x, pos_y);
                    // self.temp_debug.push(pos.to_owned());
                    if tiles.contains_key(&pos) {
                        self.coyote_timer_instant = Instant::now();
                        self.coyote_timer_active = true;
                        // what why is this reversed ??!?!?!
                        self.side_climbing = -i;
                        return
                    }
                }
            }
        }
    }

    fn wall_jump(&mut self) {
        if self.side_climbing == 0 {
            return
        }
        
        // climbing on the left side of a wall
        if self.side_climbing == -1 {
                    
            // wall jump
            if (self.left_pressed || self.right_pressed) && self.jump_pressed && self.can_jump {
                self.can_jump = false;
                self.is_jumping = true;
                self.on_ground = false;
                self.x_velocity -= 0.5;
                self.y_velocity -= 1.0;
            } else if self.left_pressed {
                self.x_velocity -= 0.05;
                self.coyote_timer_instant = Instant::now();
                self.coyote_timer_active = true;
            }
        }

        // climbing on the right side of a wall
        if self.side_climbing == 1 {

            // wall jump
            if (self.left_pressed || self.right_pressed) && self.jump_pressed && self.can_jump {
                self.can_jump = false;
                self.is_jumping = true;
                self.on_ground = false;
                self.x_velocity += 0.5;
                self.y_velocity -= 1.0;
            } else if self.right_pressed {
                self.x_velocity += 0.05;
                self.coyote_timer_instant = Instant::now();
                self.coyote_timer_active = true;
            }
        }


    }

    fn update_movement(&mut self, _tiles : H) {
        // might want to make it so that holding down w and running into a wall doesnt trigger the walljump immediately
        // also might want to add a "coyote" timer of some sorts in case they walk off of the wall before they jump and fail the walljump
        
        // everything is upside down ahahhhhhhhhh
        
        if self.is_climbing {
            self.update_climbing();
        } else {
            self.update_motion();
            self.detect_wall_jumpable_walls(_tiles);
        }
        self.update_can_jump();
    }
        
    fn update_climbing(&mut self) {
        if self.up_pressed {
            self.y_velocity -= 0.1;
        }
        // else {
        //     self.y_velocity += 0.01;
        // }

        if self.down_pressed {
            self.y_velocity += 0.1;
        }

        self.wall_jump();


        self.x_velocity *= 0.8;

        if self.y_velocity < 0.01 && self.y_velocity > -0.01 {
            self.y_velocity = 0.0;
        } else {
            self.y_velocity *= 0.6;
        }
    }

    fn update_motion(&mut self) {
        let mut multiplier = 0.7;
        
        // why is everything upside down and reversed oh god oh no
        if self.on_ground {
            if self.jump_pressed && self.can_jump {
                self.y_velocity -= 1.25;
                self.on_ground = false;
                self.is_jumping = true;
                self.can_jump = false;
            }

            if self.left_pressed {
                self.x_velocity -= 0.25;
            }
            if self.right_pressed {
                self.x_velocity += 0.25;
            }

            // lol
            if self.x_velocity < 0.01 && self.x_velocity > -0.01 {
                self.x_velocity = 0.0;
            } else {
                self.x_velocity *= 0.5;
            }
        } else {
            if !self.jump_pressed {
                self.is_jumping = false;
            }
            
            if self.is_jumping {
                multiplier = 0.9;
                self.y_velocity -= 0.01;
            } else {
                self.y_velocity -= 0.005;                
            }
            
            if self.y_velocity > 0.0 {
                multiplier = 1.0;
                self.is_jumping = false;
            }

            // coyote timer thing so more room for one to jump off of a wall
            if self.coyote_timer_active && self.can_jump {
                let now = Instant::now();
                let duration = now.duration_since(self.coyote_timer_instant);
                if duration.as_secs_f32() < 0.1 {
                    if self.jump_pressed {
                        self.wall_jump();
                        // self.y_velocity -= 1.0;
                        // self.is_jumping = true;
                        // self.coyote_timer_active = false;
                    }
                }
            }
            
            // cap max falling speed lol
            if self.y_velocity < 2.0 {
                self.y_velocity += self.gravity;
            }
            
            if self.left_pressed {
                self.x_velocity -= 0.05;
            }
            if self.right_pressed {
                self.x_velocity += 0.05;
            }


            self.x_velocity *= 0.8;
        }

        self.y_velocity *= multiplier;
    }

    fn vamonos(&mut self) {
        self.player_x += self.x_velocity;
        self.player_y += self.y_velocity;
    }

    fn update_can_jump(&mut self) {
        if !self.jump_pressed {
            self.can_jump = true;
        }
    }
}
