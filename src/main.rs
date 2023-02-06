use std::time::Duration;

use crate::camera::*;
use crate::platform::*;
use crate::player::*;
use crate::ui::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

mod camera;
mod platform;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HanabiPlugin)
        .insert_resource(CameraMoveState::default())
        .insert_resource(Score(0))
        .insert_resource(Accumulator(None))
        .insert_resource(JumpState::default())
        .insert_resource(FallState::default())
        .insert_resource(GenerateAccumulationParticleEffectTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .add_event::<GameOverEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        .add_startup_system(setup_first_platform)
        .add_startup_system(setup_player)
        .add_startup_system(setup_scoreboard)
        .add_system(generate_next_platform)
        .add_system(move_camera)
        .add_system(player_jump)
        .add_system(update_scoreboard)
        .add_system(animate_jump)
        .add_system(animate_fall)
        .add_system(animate_player_accumulation)
        .add_system(animate_platform_accumulation)
        .add_system(animate_accumulation_particle_effect)
        .add_system(handle_game_over_event)
        .run();
}
