use bitflags::bitflags;

use hash40::Hash40;
use smash::cpp;
use smash_stage::{lib::ParameterResource, phx};

use crate::app::ButtonCheckType;

#[repr(C)]
pub struct CameraMeleeReplayController {
    _0x0: *const CameraMeleeReplayControllerSub,
    _0x8: *const Self,
    _0x10: [u8; 0x100],
    _0x110: phx::MatrixT4x4f,
    _0x150: phx::MatrixT4x4f,
    _0x190: [u8; 0x110],
    pub pos: phx::Vector3f,
    pub angle: phx::Vector3f,
    pub roll: f32,
    pub distance: f32,
    pub fov: f32,
    _0x2cc: f32,
    pub is_gyro_enabled: bool,
    pub is_gyro_reset: bool,
    _0x2d2: u8,
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
    _0x324: i8,
    pub limit_pos_x_prev_sign: i8,
    pub limit_pos_y_prev_sign: i8,
    pub limit_distance_prev_sign: i8,
    pub limit_angle_x_prev_sign: i8,
    pub limit_angle_y_prev_sign: i8,
    pub limit_roll_prev_sign: i8,
    pub limit_gyro_angle_x_prev_sign: i8,
    pub limit_gyro_angle_y_prev_sign: i8,
    _0x32d: i8,
    _0x32e: [u8; 0x6],
    _0x334: u8,
    _0x335: [u8; 0x3],
    _0x338: i32,
    _0x33c: u8,
    _0x33d: [u8; 0x3],
}

impl CameraMeleeReplayController {
    pub fn is_order_impl(&self, order: Hash40, check_type: ButtonCheckType) -> bool {
        unsafe { ((*self._0x0).vtable.is_order)(&*self._0x0, order, check_type) }
    }
}

#[repr(C)]
pub struct CameraMeleeReplayControllerSub {
    vtable: &'static CameraMeleeReplayControllerSubVTable, // Offset: 0x4F73300
    _0x8: [u8; 0x1A0],
    _0x1a8: *const CameraMeleeReplayParameter,
    _0x1b0: usize,
    _0x1b8: usize,
    _0x1c0: usize,
    _0x1c8: usize,
    _0x1d0: usize,
    _0x1d8: u32,
    _0x1dc: u8,
    _0x1dd: u8,
}

#[repr(C)]
pub struct CameraMeleeReplayControllerSubVTable {
    destructor: extern "C" fn(&mut CameraMeleeReplayControllerSub),
    deleter: extern "C" fn(*mut CameraMeleeReplayControllerSub),
    get_stage_param: extern "C" fn(&CameraMeleeReplayControllerSub) -> &CameraStageParam,
    get_param: extern "C" fn(&CameraMeleeReplayControllerSub) -> &Param,
    _0x20: extern "C" fn(&CameraMeleeReplayControllerSub) -> u32,
    _0x28: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x30: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x38: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x40: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x48: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x50: extern "C" fn(&CameraMeleeReplayControllerSub) -> bool,
    _0x58: extern "C" fn(&CameraMeleeReplayControllerSub) -> u32,
    _0x60: extern "C" fn(&CameraMeleeReplayControllerSub, bool),
    _0x68: extern "C" fn(&CameraMeleeReplayControllerSub) -> u32,
    _0x70: extern "C" fn(&CameraMeleeReplayControllerSub) -> usize,
    is_order: extern "C" fn(&CameraMeleeReplayControllerSub, Hash40, ButtonCheckType) -> bool,
    _0x80: extern "C" fn(&CameraMeleeReplayControllerSub) -> u8,
    _0x88: extern "C" fn(&CameraMeleeReplayControllerSub, u32) -> bool,
    _0x90: extern "C" fn(&CameraMeleeReplayControllerSub) -> u8,
    _0x98: extern "C" fn(&CameraMeleeReplayControllerSub) -> u8,
}

#[repr(C)]
pub struct CameraMeleeReplayParameter {
    pub resource_handle: u32,
    pub resource: ParameterResource,
    pub param: Param,
    pub shift: cpp::Tree<Hash40, Shift>,
    pub assign: cpp::Tree<Hash40, Assign>,
    pub stage_param: CameraStageParam,
    _0x130: i32,
    _0x134: [u8; 0xC],
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
    pub button_joyleft: Button,
    pub button_joyright: Button,
}

#[repr(C)]
pub struct Assign {
    pub shift: Hash40,
    pub shift_gc: Hash40,
    pub shift_joyleft: Hash40,
    pub shift_joyright: Hash40,
    pub button: Button,
    pub button_gc: Button,
    pub button_joyleft: Button,
    pub button_joyright: Button,
    pub x11a97fa1b2: bool,
    pub x0f7c903f2c: bool,
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

#[repr(C)]
pub struct CameraStageParam {
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
}
