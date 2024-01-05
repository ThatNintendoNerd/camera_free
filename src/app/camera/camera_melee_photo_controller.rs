use bitflags::bitflags;

use hash40::Hash40;
use smash::cpp;
use smash_stage::{lib::ParameterResource, phx};

use crate::{app::ButtonCheckType, offsets::OFFSETS};

#[repr(C)]
pub struct CameraMeleePhotoController {
    _0x0: [u8; 0xF8],
    pub resource: ParameterResource,
    pub param: Param,
    pub shift: cpp::Tree<Hash40, Shift>,
    pub assign: cpp::Tree<Hash40, Assign>,
    _0x190: u32,
    _0x198: usize,
    _0x1a0: i32,
    pub is_active: bool,
    pub reset_pos: phx::Vector3f,
    pub reset_distance: f32,
    pub reset_fov: f32,
    pub limit_pos_min: phx::Vector3f,
    pub limit_pos_max: phx::Vector3f,
    pub limit_angle_min: phx::Vector3f,
    pub limit_angle_max: phx::Vector3f,
    pub gyro_limit_angle_min: phx::Vector3f,
    pub gyro_limit_angle_max: phx::Vector3f,
    pub distance_min: f32,
    pub distance_max: f32,
    pub fov_min: f32,
    pub fov_max: f32,
    _0x240: phx::MatrixT4x4f,
    _0x280: phx::MatrixT4x4f,
    _0x2c0: [u8; 0x110],
    pub pos: phx::Vector3f,
    pub angle: phx::Vector3f,
    pub roll: f32,
    pub distance: f32,
    pub fov: f32,
    _0x3fc: f32,
    pub is_gyro_enabled: bool,
    pub is_gyro_reset: bool,
    pub gyro_reset_rotate: phx::Quaternion,
    _gyro_rotate_a: phx::Quaternion,
    _gyro_rotate_b: phx::Quaternion,
    pub track_speed: f32,
    pub roll_speed: f32,
    pub zoom_speed: f32,
    pub limit_pos_x_sign: i8,
    pub limit_pos_y_sign: i8,
    pub limit_distance_sign: i8,
    pub limit_angle_x_sign: i8,
    pub limit_angle_y_sign: i8,
    pub limit_roll_sign: i8,
    pub limit_gyro_angle_x_sign: i8,
    pub limit_gyro_angle_y_sign: i8,
    _0x454: i8,
    pub limit_pos_x_prev_sign: i8,
    pub limit_pos_y_prev_sign: i8,
    pub limit_distance_prev_sign: i8,
    pub limit_angle_x_prev_sign: i8,
    pub limit_angle_y_prev_sign: i8,
    pub limit_roll_prev_sign: i8,
    pub limit_gyro_angle_x_prev_sign: i8,
    pub limit_gyro_angle_y_prev_sign: i8,
    _0x45d: i8,
    pub filter_setting: i32,
    pub frame_setting: i32,
    pub effect_setting: i32,
    pub figure_frame: i32,
    pub is_figure_visible: bool,
    pub is_guide_visible: bool,
    _0x478: usize,
    _0x480: [u8; 0x20],
}

impl CameraMeleePhotoController {
    pub fn is_order_impl(&self, order: Hash40, check_type: ButtonCheckType) -> bool {
        #[skyline::from_offset(OFFSETS.camera_melee_photo_controller_is_order)]
        fn is_order(
            this: *const CameraMeleePhotoController,
            order: Hash40,
            check_type: ButtonCheckType,
        ) -> bool;

        unsafe { is_order(self, order, check_type) }
    }
}

#[repr(C)]
pub struct Param {
    pub move_speed: f32,
    pub move_dec_rate: f32,
    pub pitch_speed: f32,
    pub yaw_speed: f32,
    pub track_time: f32,
    pub track_acc_frame: f32,
    pub track_up_dec_dist: f32,
    pub track_up_dec_rate: f32,
    pub roll_speed: f32,
    pub roll_acc_frame: f32,
    pub roll_max: f32,
    pub zoom_time: f32,
    pub zoom_acc_frame: f32,
}

#[repr(C)]
pub struct Shift {
    pub button: Button,
    pub button_gc: Button,
}

#[repr(C)]
pub struct Assign {
    pub shift: Hash40,
    pub shift_gc: Hash40,
    pub button: Button,
    pub button_gc: Button,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    #[repr(C)]
    pub struct Button: u32 {
        const UP = 0x1;
        const RIGHT = 0x2;
        const DOWN = 0x4;
        const LEFT = 0x8;
        const X = 0x10;
        const A = 0x20;
        const B = 0x40;
        const Y = 0x80;
        const L = 0x100;
        const R = 0x200;
        const ZL = 0x400;
        const ZR = 0x800;
        const SLIDE_L = 0x10000;
        const SLIDE_R = 0x20000;

        const Z = 0x800;
    }
}
