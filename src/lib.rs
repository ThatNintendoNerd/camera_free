mod app;
mod controller;
mod hooks;
mod offsets;

#[skyline::main(name = "camera_free")]
fn main() {
    use skyline::patching::Patch;

    use offsets::Offsets;

    skyline::install_hooks!(
        hooks::camera_melee_replay_controller_update,
        hooks::camera_melee_photo_controller_update,
        hooks::stage_camera_normal_param_read,
        hooks::stage_camera_pause_param_read,
    );

    Patch::in_text(Offsets::get().camera_melee_replay_controller_distance_limit_check)
        .nop()
        .unwrap();
    Patch::in_text(Offsets::get().camera_melee_replay_controller_fov_check)
        .nop()
        .unwrap();
    Patch::in_text(Offsets::get().camera_melee_photo_controller_fov_check)
        .nop()
        .unwrap();
    Patch::in_text(Offsets::get().camera_melee_photo_controller_distance_limit_check)
        .nop()
        .unwrap();
}
