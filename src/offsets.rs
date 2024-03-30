use once_cell::sync::Lazy;

/// The container for cached offsets to code.
pub struct Offsets {
    pub camera_melee_replay_controller_update: usize,
    pub camera_melee_replay_controller_distance_limit_check: usize,
    pub camera_melee_replay_controller_fov_check: usize,
    pub camera_melee_photo_controller_fov_check: usize,
    pub camera_melee_photo_controller_update: usize,
    pub camera_melee_photo_controller_distance_limit_check: usize,
    pub camera_melee_photo_controller_is_order: usize,
    pub stage_camera_normal_param_read: usize,
    pub stage_camera_pause_param_read: usize,
}

impl Offsets {
    /// Constructs a new instance of `Offsets`.
    fn new() -> Self {
        let text = text();

        Self {
            camera_melee_replay_controller_update: Self::find(
                text,
                CAMERA_MELEE_REPLAY_CONTROLLER_UPDATE_SEARCH_CODE,
            )
            .unwrap_or(0x4F7B00),
            camera_melee_replay_controller_distance_limit_check: Self::find(
                text,
                CAMERA_MELEE_REPLAY_CONTROLLER_DISTANCE_LIMIT_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x4F953C),
            camera_melee_replay_controller_fov_check: Self::find(
                text,
                CAMERA_MELEE_REPLAY_CONTROLLER_FOV_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x4FA468),
            camera_melee_photo_controller_fov_check: Self::find(
                text,
                CAMERA_MELEE_PHOTO_CONTROLLER_FOV_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x50ADB0),
            camera_melee_photo_controller_update: Self::find(
                text,
                CAMERA_MELEE_PHOTO_CONTROLLER_UPDATE_SEARCH_CODE,
            )
            .unwrap_or(0x50AEA0),
            camera_melee_photo_controller_distance_limit_check: Self::find(
                text,
                CAMERA_MELEE_PHOTO_CONTROLLER_DISTANCE_LIMIT_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x50CF80),
            camera_melee_photo_controller_is_order: Self::find(
                text,
                CAMERA_MELEE_PHOTO_CONTROLLER_IS_ORDER_SEARCH_CODE,
            )
            .unwrap_or(0x50D3B0),
            stage_camera_normal_param_read: Self::find(
                text,
                STAGE_CAMERA_NORMAL_PARAM_READ_SEARCH_CODE,
            )
            .unwrap_or(0x2621470),
            stage_camera_pause_param_read: Self::find(
                text,
                STAGE_CAMERA_PAUSE_PARAM_READ_SEARCH_CODE,
            )
            .unwrap_or(0x2623330),
        }
    }

    /// Returns a reference to a `Lazy` containing the current instance of `Offsets`.
    pub fn get() -> &'static Lazy<Self> {
        static INSTANCE: Lazy<Offsets> = Lazy::new(Offsets::new);

        &INSTANCE
    }

    /// Returns the offset to the needle in the haystack, or `None` if it was not found.
    fn find(haystack: &[u8], needle: (&[u8], isize)) -> Option<usize> {
        use memchr::memmem;

        memmem::find(haystack, needle.0).map(|o| (o as isize + needle.1) as usize)
    }
}

/// Returns a byte slice representing the code segment of the target application.
fn text() -> &'static [u8] {
    use std::slice;

    use skyline::hooks::{getRegionAddress, Region};

    unsafe {
        let ptr = getRegionAddress(Region::Text) as *const u8;
        let len = (getRegionAddress(Region::Rodata) as usize) - (ptr as usize);

        slice::from_raw_parts(ptr, len)
    }
}

#[rustfmt::skip]
static CAMERA_MELEE_REPLAY_CONTROLLER_UPDATE_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xFD, 0x03, 0x03, 0x91,
        0x08, 0x90, 0x4C, 0x39,
        0x1A, 0x70, 0x0C, 0x91,
    ],
    -0x24,
);

#[rustfmt::skip]
static CAMERA_MELEE_REPLAY_CONTROLLER_DISTANCE_LIMIT_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x68, 0x9E, 0x4C, 0x39,
        0x28, 0x01, 0x00, 0x34,
    ],
    0x04,
);

#[rustfmt::skip]
static CAMERA_MELEE_REPLAY_CONTROLLER_FOV_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x09, 0x65, 0x4B, 0xB9,
        0x09, 0x24, 0x01, 0xB9,
    ],
    0x18,
);

#[rustfmt::skip]
static CAMERA_MELEE_PHOTO_CONTROLLER_FOV_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x09, 0x65, 0x4B, 0xB9,
        0x09, 0x34, 0x02, 0xB9,
    ],
    0x18,
);

#[rustfmt::skip]
static CAMERA_MELEE_PHOTO_CONTROLLER_UPDATE_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xFD, 0x03, 0x03, 0x91,
        0x08, 0xC4, 0x40, 0xB9,
        0xF3, 0x03, 0x00, 0xAA,
    ],
    -0x28,
);

#[rustfmt::skip]
static CAMERA_MELEE_PHOTO_CONTROLLER_DISTANCE_LIMIT_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x68, 0x5E, 0x51, 0x39,
        0x28, 0x01, 0x00, 0x34,
    ],
    0x04,
);

#[rustfmt::skip]
static CAMERA_MELEE_PHOTO_CONTROLLER_IS_ORDER_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x08, 0xCC, 0x40, 0xF9,
        0xC8, 0x02, 0x00, 0xB4,
    ],
    0x00,
);

#[rustfmt::skip]
static STAGE_CAMERA_NORMAL_PARAM_READ_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xE9, 0x23, 0xBD, 0x6D,
        0xF4, 0x4F, 0x01, 0xA9,
        0xFD, 0x7B, 0x02, 0xA9,
        0xFD, 0x83, 0x00, 0x91,
        0x08, 0x00, 0x40, 0xF9,
    ],
    0x00,
);

#[rustfmt::skip]
static STAGE_CAMERA_PAUSE_PARAM_READ_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xF4, 0x4F, 0xBE, 0xA9,
        0xFD, 0x7B, 0x01, 0xA9,
        0xFD, 0x43, 0x00, 0x91,
        0x08, 0x00, 0x40, 0xF9,
        0x08, 0x01, 0x40, 0xF9,
        0xF3, 0x03, 0x01, 0xAA,
    ],
    0x00,
);
