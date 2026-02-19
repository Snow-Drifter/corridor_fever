//! Camera system with smooth player tracking.

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, demo::player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_camera
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn center_camera_on_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&GlobalTransform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    let Ok(player_global) = player.single() else {
        return;
    };
    let Vec3 { x, y, .. } = player_global.translation();
    camera_transform.translation.x = x;
    camera_transform.translation.y = y;
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.0;

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&GlobalTransform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    let Ok(player_global) = player.single() else {
        return;
    };

    let Vec3 { x, y, .. } = player_global.translation();
    let target = Vec3::new(x, y, camera_transform.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera_transform
        .translation
        .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}
