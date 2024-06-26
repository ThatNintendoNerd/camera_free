use smash_stage::app::{StageCameraNormalParam, StageCameraPauseParam, StageParamAccessor};

use crate::{
    app::{CameraMeleePhotoController, CameraMeleeReplayController},
    controller::CameraMeleeFreeController,
    offsets::Offsets,
};

#[skyline::hook(offset = Offsets::get().camera_melee_replay_controller_update)]
fn camera_melee_replay_controller_update(camera: &mut CameraMeleeReplayController) {
    let prev_pos = camera.pos;
    let prev_distance = camera.distance;

    original!()(camera);

    let mut instance = CameraMeleeFreeController::instance().lock();

    instance.set_work_param(prev_pos, prev_distance);
    instance.update(camera);
}

#[skyline::hook(offset = Offsets::get().camera_melee_photo_controller_update)]
fn camera_melee_photo_controller_update(camera: &mut CameraMeleePhotoController) {
    let prev_pos = camera.pos;
    let prev_distance = camera.distance;

    original!()(camera);

    let mut instance = CameraMeleeFreeController::instance().lock();

    instance.set_work_param(prev_pos, prev_distance);
    instance.update(camera);
}

#[skyline::hook(offset = Offsets::get().stage_camera_normal_param_read)]
fn stage_camera_normal_param_read(
    accessor: &StageParamAccessor,
    param: &mut StageCameraNormalParam,
) {
    original!()(accessor, param);

    CameraMeleeFreeController::set_normal_camera_param(param);
}

#[skyline::hook(offset = Offsets::get().stage_camera_pause_param_read)]
fn stage_camera_pause_param_read(accessor: &StageParamAccessor, param: &mut StageCameraPauseParam) {
    original!()(accessor, param);

    CameraMeleeFreeController::set_pause_camera_param(param);
}
