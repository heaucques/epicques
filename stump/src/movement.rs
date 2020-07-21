use std::time::Instant;
// use std::time::Duration;

pub struct Movement {
    pub y_velocity: f64,
    pub x_velocity: f64,
    pub gravity: f64,

    pub on_ground: bool,
    pub is_jumping: bool,

    pub is_climbing: bool,
    // idk how else to fix this this seems to be the easiest way
    pub side_climbing: i64,

    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub jump_pressed: bool,
    pub grab_pressed: bool,

    pub can_jump: bool,

    pub player_x: f64,
    pub player_y: f64,

    // coyote timer thing (i think thats what its called)
    pub coyote_timer_instant: Instant,
    pub coyote_timer_active: bool,

    pub temp_debug: Vec<super::Position>,

}