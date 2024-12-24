use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::{JumpState, INITIAL_PLAYER_POS};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

#[derive(Debug, Resource)]
pub struct GameSounds {
    pub start: Handle<AudioSource>,
    pub accumulation: Handle<AudioSource>,
    pub fall: Handle<AudioSource>,
    pub success: Handle<AudioSource>,
}

#[derive(Component)]
pub enum MenuButtonAction {
    StartGame,
    RestartGame,
    BackToMainMenu,
}

#[derive(Component)]
pub struct OnMainMenuScreen;
#[derive(Component)]
pub struct OnGameOverMenuScreen;

#[derive(Debug, Resource)]
pub struct Score(pub u32);

#[derive(Debug, Component)]
pub struct Scoreboard;

#[derive(Debug, Resource)]
pub struct ScoreUpQueue(pub Vec<ScoreUpEvent>);
#[derive(Debug)]
pub struct ScoreUpEvent {
    pub landing_pos: Vec3,
}

#[derive(Debug, Component)]
pub struct ScoreUpEffect(pub Vec3);

pub fn setup_game_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        start: asset_server.load("sounds/start.mp3"),
        accumulation: asset_server.load("sounds/accumulation.mp3"),
        fall: asset_server.load("sounds/fall.mp3"),
        success: asset_server.load("sounds/success.mp3"),
    });
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // 标题
                    parent.spawn((ImageNode::new(
                        asset_server.load("texture/title.png").into(),
                    ),));

                    // 开始按钮
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(150.),
                            height: Val::Px(60.),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ImageNode::new(asset_server.load("texture/btn_start.png").into()),
                        MenuButtonAction::StartGame,
                    ));
                });
        });
}

pub fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // 标题
                    parent.spawn((ImageNode::new(asset_server.load("texture/title.png")),));

                    parent
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            // 返回按钮
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(40.),
                                    height: Val::Px(40.),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ImageNode::new(asset_server.load("texture/btn_home.png")),
                                MenuButtonAction::BackToMainMenu,
                            ));

                            // 重新开始按钮
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(150.),
                                    height: Val::Px(60.),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ImageNode::new(asset_server.load("texture/btn_restart.png")),
                                MenuButtonAction::RestartGame,
                            ));
                        });
                });
        });
}

pub fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("Score: "),
            TextColor(Color::srgb(0.5, 0.5, 1.0)),
            TextFont {
                font: asset_server.load("fonts/num.ttf"),
                font_size: 40.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(30.0),
                left: Val::Px(30.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("0"),
            TextColor(Color::srgb(1.0, 0.5, 0.5)),
            TextFont {
                font: asset_server.load("fonts/num.ttf"),
                font_size: 40.0,
                ..default()
            },
            Scoreboard,
        ));
}

pub fn update_scoreboard(score: Res<Score>, mut span: Single<&mut TextSpan, With<Scoreboard>>) {
    if score.is_changed() {
        span.0 = score.0.to_string();
    }
}

// 当摄像机或飘分效果坐标变化时进行同步
pub fn sync_score_up_effect(
    mut q_score_up_effect: Query<(&mut Node, &mut ScoreUpEffect)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera, camera_global_transform) = q_camera.single();
    let window = q_windows.single();
    for (mut score_up_effect_style, score_up_effect) in &mut q_score_up_effect {
        let viewport_pos = camera
            .world_to_viewport(camera_global_transform, score_up_effect.0)
            .unwrap();
        score_up_effect_style.top = Val::Px(window.resolution.height() - viewport_pos.y);
        score_up_effect_style.left = Val::Px(viewport_pos.x);
    }
}

// 向上移动飘分效果
pub fn shift_score_up_effect(
    mut commands: Commands,
    mut q_score_up_effect: Query<(Entity, &mut TextColor, &mut ScoreUpEffect)>,
    time: Res<Time>,
) {
    for (entity, mut text_color, mut score_up_effect) in &mut q_score_up_effect {
        score_up_effect.0.y += 1.0 * time.delta_secs();
        // 边移动边增加透明度
        let alpha = text_color.0.alpha();
        text_color.0.set_alpha(alpha * 0.97);
        if score_up_effect.0.y > INITIAL_PLAYER_POS.y + 1.2 {
            commands.entity(entity).despawn();
        }
    }
}

// 创建飘分效果
pub fn spawn_score_up_effect(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score_up_queue: ResMut<ScoreUpQueue>,
    jump_state: Res<JumpState>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if jump_state.completed {
        let window = q_windows.single();
        // 启动score up动画
        for score_up_event in score_up_queue.0.iter_mut() {
            let (camera, camera_global_transform) = q_camera.single();
            let viewport_pos = camera
                .world_to_viewport(camera_global_transform, score_up_event.landing_pos)
                .unwrap();
            dbg!(viewport_pos);
            commands.spawn((
                Text::new("+1"),
                TextColor(Color::srgb(0.5, 0.5, 1.0)),
                TextFont {
                    font: asset_server.load("fonts/num.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(window.resolution.height() - viewport_pos.y),
                    left: Val::Px(viewport_pos.x),
                    ..default()
                },
                ScoreUpEffect(score_up_event.landing_pos),
            ));
        }
        score_up_queue.0.clear();
    }
}

pub fn click_button(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match menu_button_action {
                MenuButtonAction::StartGame => {
                    info!("StartGame button clicked");
                    next_game_state.set(GameState::Playing);
                }
                MenuButtonAction::RestartGame => {
                    info!("RestartGame button clicked");
                    next_game_state.set(GameState::Playing);
                }
                MenuButtonAction::BackToMainMenu => {
                    info!("BackToMainMenu button clicked");
                    next_game_state.set(GameState::MainMenu);
                }
            },
            _ => {}
        }
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_scoreboard(mut commands: Commands, q_scoreboard: Query<Entity, With<Scoreboard>>) {
    for scoreboard in &q_scoreboard {
        commands.entity(scoreboard).despawn();
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}
