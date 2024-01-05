mod camera_melee_photo_controller;
mod camera_melee_replay_controller;

pub use camera_melee_photo_controller::*;
pub use camera_melee_replay_controller::*;

#[allow(dead_code)]
#[repr(C)]
pub enum ButtonCheckType {
    Down = 0,
    Press = 1,
    Interval = 2,
}
