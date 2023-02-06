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
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(HanabiPlugin);
    }

    let mut game_playing_set_on_update = SystemSet::on_update(GameState::Playing)
        .with_system(prepare_jump)
        .with_system(generate_next_platform)
        .with_system(move_camera)
        .with_system(player_jump)
        .with_system(update_scoreboard)
        .with_system(animate_jump)
        .with_system(animate_fall)
        .with_system(animate_player_accumulation)
        .with_system(animate_platform_accumulation);

    #[cfg(not(target_arch = "wasm32"))]
    {
        game_playing_set_on_update = game_playing_set_on_update.with_system(animate_accumulation_particle_effect);
    }

    app
        // .add_plugins(DefaultPlugins)
        .add_state(GameState::MainMenu)
        .insert_resource(CameraMoveState::default())
        .insert_resource(Score(0))
        .insert_resource(Accumulator(None))
        .insert_resource(JumpState::default())
        .insert_resource(FallState::default())
        .insert_resource(GenerateAccumulationParticleEffectTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .insert_resource(PrepareJumpTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ground)
        // Main Menu
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(setup_main_menu)
                .with_system(clear_player)
                .with_system(clear_platforms)
                .with_system(despawn_scoreboard),
        )
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu).with_system(despawn_screen::<OnMainMenuScreen>),
        )
        // Playing
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(clear_player)
                .with_system(clear_platforms)
                .with_system(despawn_scoreboard)
                .with_system(setup_first_platform.after(clear_platforms))
                .with_system(setup_player.after(clear_player))
                .with_system(setup_scoreboard.after(despawn_scoreboard))
                .with_system(reset_score)
                .with_system(reset_prepare_jump_timer),
        )
        .add_system_set(
            game_playing_set_on_update
        )
        // GameOver
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_game_over_menu))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(despawn_screen::<OnGameOverMenuScreen>),
        )
        .run();
}
