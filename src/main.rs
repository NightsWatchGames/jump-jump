use std::time::Duration;

use crate::camera::*;
use crate::platform::*;
use crate::player::*;
use crate::ui::*;
use bevy::prelude::*;
// use bevy_hanabi::prelude::*;

mod camera;
mod platform;
mod player;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     app.add_plugin(HanabiPlugin);
    // }

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     game_playing_set_on_update =
    //         game_playing_set_on_update.with_system(animate_accumulation_particle_effect);
    // }

    app.init_state::<GameState>()
        .insert_resource(CameraMoveState::default())
        .insert_resource(Score(0))
        .insert_resource(Accumulator(None))
        .insert_resource(JumpState::default())
        .insert_resource(FallState::default())
        .insert_resource(GenerateAccumulationParticleEffectTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .insert_resource(PrepareJumpTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )))
        .insert_resource(ScoreUpQueue(Vec::new()))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_ground,
                setup_ui_images,
                setup_game_sounds,
            ),
        )
        // Main Menu
        .add_systems(
            OnEnter(GameState::MainMenu),
            (
                setup_main_menu,
                clear_player,
                clear_platforms,
                despawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (click_button,).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (despawn_screen::<OnMainMenuScreen>,),
        )
        // Playing
        .add_systems(
            OnEnter(GameState::Playing),
            (
                clear_player,
                clear_platforms,
                despawn_scoreboard,
                setup_first_platform.after(clear_platforms),
                setup_player.after(clear_player),
                setup_scoreboard.after(despawn_scoreboard),
                reset_score,
                reset_prepare_jump_timer,
            ),
        )
        .add_systems(
            Update,
            (
                prepare_jump,
                generate_next_platform,
                move_camera,
                player_jump,
                update_scoreboard,
                animate_jump,
                animate_fall,
                animate_player_accumulation,
                animate_platform_accumulation.after(player_jump),
                spawn_score_up_effect,
                sync_score_up_effect,
                shift_score_up_effect,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // GameOver
        .add_systems(OnEnter(GameState::GameOver), (setup_game_over_menu,))
        .add_systems(
            Update,
            (click_button,).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            (despawn_screen::<OnGameOverMenuScreen>,),
        )
        .run();
}
