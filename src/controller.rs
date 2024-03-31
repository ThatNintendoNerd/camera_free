use hash40::Hash40;
use parking_lot::Mutex;
use smash_stage::{
    app::{StageCameraNormalParam, StageCameraPauseParam},
    phx,
};

use crate::app::{ButtonCheckType, CameraMeleePhotoController, CameraMeleeReplayController};

/// The controller for freeing a camera controller of its restrictions.
pub struct CameraMeleeFreeController {
    /// The working camera position.
    work_pos: phx::Vector3f,

    /// The working camera distance from the pivot point.
    work_distance: f32,
}

impl CameraMeleeFreeController {
    /// Constructs a new instance of `CameraMeleeFreeController`.
    const fn new() -> Self {
        Self {
            work_pos: phx::Vector3f::ZERO,
            work_distance: 0.0,
        }
    }

    /// Returns a reference to a `Mutex` containing the current instance of `CameraMeleeFreeController`.
    pub fn instance() -> &'static Mutex<Self> {
        static INSTANCE: Mutex<CameraMeleeFreeController> =
            Mutex::new(CameraMeleeFreeController::new());

        &INSTANCE
    }

    /// Updates the camera based on the current game state.
    pub fn update(&self, camera: &mut impl Camera) {
        if Self::is_reset(camera) {
            return;
        }

        camera.set_distance(self.calc_distance(camera));
        camera.set_pos(self.calc_pos(camera));
    }

    /// Returns `true` if either of the reset orders have been executed.
    fn is_reset(camera: &impl Camera) -> bool {
        if camera.is_order(Hash40::new("reset"), ButtonCheckType::Press) {
            return true;
        }

        if camera.is_order(Hash40::new("reset_2"), ButtonCheckType::Press) {
            return true;
        }

        false
    }

    /// Calculates the camera's distance from the pivot point.
    fn calc_distance(&self, camera: &impl Camera) -> f32 {
        const TRACK_ACCEL: f32 = 0.04;

        self.work_distance * camera.track_speed().mul_add(TRACK_ACCEL, 1.0)
    }

    /// Calculates the camera's position with the z-axis in mind.
    fn calc_pos(&self, camera: &impl Camera) -> phx::Vector3f {
        let vel = self.calc_velocity(camera);

        self.work_pos + vel
    }

    /// Calculates the camera's velocity with the z-axis in mind.
    fn calc_velocity(&self, camera: &impl Camera) -> phx::Vector3f {
        let pos = camera.pos();
        let delta_pos = phx::Vector3f::new(pos.x - self.work_pos.x, pos.y - self.work_pos.y, 0.0);
        let rotate = camera.angle();
        let rotate_matrix = phx::MatrixT4x4f::from_rotation_x(rotate.x)
            * phx::MatrixT4x4f::from_rotation_y(rotate.y);
        let vel = phx::Vector3f::transform(&delta_pos, &rotate_matrix);

        phx::Vector3f::new(vel.x, vel.y, vel.z)
    }

    /// Sets the camera position and distance from the pivot point.
    pub fn set_work_param(&mut self, pos: phx::Vector3f, distance: f32) {
        self.work_pos = pos;
        self.work_distance = distance;
    }

    /// Sets the unrestricted normal camera parameters.
    pub fn set_normal_camera_param(param: &mut StageCameraNormalParam) {
        param.vr_camera_position_min.x = f32::NAN;
        param.vr_camera_position_min.y = f32::NAN;
        param.vr_camera_position_min.z = f32::NEG_INFINITY;
        param.vr_camera_position_max.x = f32::NAN;
        param.vr_camera_position_max.y = f32::NAN;
        param.vr_camera_position_max.z = f32::INFINITY;
        param.vr_mode_notice_range_side = 0.0;
        param.vr_mode_notice_range_upper = 0.0;
        param.vr_mode_notice_range_lower = 0.0;
    }

    /// Sets the unrestricted pause camera parameters.
    pub fn set_pause_camera_param(param: &mut StageCameraPauseParam) {
        param.min_fov = f32::MIN_POSITIVE.to_radians();
        param.max_fov = 180.0_f32.to_radians();
        param.limit_pos_top = f32::NAN;
        param.limit_pos_bottom = f32::NAN;
        param.limit_pos_left = f32::NAN;
        param.limit_pos_right = f32::NAN;
        param.limit_angle_up = f32::NAN;
        param.limit_angle_down = f32::NAN;
        param.limit_angle_left = f32::NAN;
        param.limit_angle_right = f32::NAN;
        param.gyro_limit_angle_up = f32::NAN;
        param.gyro_limit_angle_down = f32::NAN;
        param.gyro_limit_angle_left = f32::NAN;
        param.gyro_limit_angle_right = f32::NAN;
    }
}

/// A trait for getting and setting values in a camera controller.
pub trait Camera {
    /// Returns `true` if the order is being executed.
    fn is_order(&self, order: Hash40, check_type: ButtonCheckType) -> bool;

    /// Returns the camera's position.
    fn pos(&self) -> phx::Vector3f;

    /// Sets the camera's position.
    fn set_pos(&mut self, pos: phx::Vector3f);

    /// Returns the camera's rotation in radians.
    fn angle(&self) -> phx::Vector3f;

    /// Returns the camera's distance from the pivot point.
    fn distance(&self) -> f32;

    /// Sets the camera's distance from the pivot point.
    fn set_distance(&mut self, distance: f32);

    /// Returns the camera's zoom speed.
    fn track_speed(&self) -> f32;
}

macro_rules! camera_impl {
    ($camera_melee_controller:ty) => {
        impl Camera for $camera_melee_controller {
            fn is_order(&self, order: Hash40, check_type: ButtonCheckType) -> bool {
                self.is_order_impl(order, check_type)
            }

            fn pos(&self) -> phx::Vector3f {
                self.pos
            }

            fn set_pos(&mut self, pos: phx::Vector3f) {
                self.pos = pos;
            }

            fn angle(&self) -> phx::Vector3f {
                self.angle
            }

            fn distance(&self) -> f32 {
                self.distance
            }

            fn set_distance(&mut self, distance: f32) {
                self.distance = distance;
            }

            fn track_speed(&self) -> f32 {
                self.track_speed
            }
        }
    };
}

camera_impl!(CameraMeleePhotoController);
camera_impl!(CameraMeleeReplayController);
