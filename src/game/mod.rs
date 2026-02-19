//! Gameplay: player, level, camera, movement, and animation.

use bevy::prelude::*;

mod animation;
mod camera;
pub use camera::center_camera_on_player;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        camera::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
}
