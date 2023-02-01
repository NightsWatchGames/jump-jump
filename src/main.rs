use crate::camera::*;
use crate::platform::*;
use crate::player::*;
use crate::ui::*;
use bevy::prelude::*;

mod camera;
mod platform;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CameraMoveState::default())
        .insert_resource(Score(0))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_startup_system(setup_first_platform)
        .add_startup_system(setup_player)
        .add_startup_system(setup_scoreboard)
        .add_system(generate_next_platform)
        .add_system(move_camera)
        .add_system(player_jump)
        .add_system(update_scoreboard)
        .run();
}
