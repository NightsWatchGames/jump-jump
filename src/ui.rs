use bevy::{app::AppExit, prelude::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
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

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(ImageBundle {
                        image: asset_server.load("texture/title.png").into(),
                        ..default()
                    });

                    // 开始按钮
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.), Val::Px(60.)),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: asset_server.load("texture/btn_start.png").into(),
                            ..default()
                        },
                        MenuButtonAction::StartGame,
                    ));
                });
        });
}

pub fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(ImageBundle {
                        image: asset_server.load("texture/title.png").into(),
                        ..default()
                    });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // 返回按钮
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(50.), Val::Px(50.)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: asset_server.load("texture/btn_back.png").into(),
                                    background_color: Color::YELLOW_GREEN.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToMainMenu,
                            ));

                            // 开始按钮
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.), Val::Px(60.)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: asset_server.load("texture/btn_restart.png").into(),
                                    ..default()
                                },
                                MenuButtonAction::RestartGame,
                            ));
                        });
                });
        });
}

pub fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("fonts/num.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/num.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(30.0),
                    left: Val::Px(30.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Scoreboard);
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    if score.is_changed() {
        let mut text = query.single_mut();
        text.sections[1].value = score.0.to_string();
    }
}

pub fn click_button(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match menu_button_action {
                MenuButtonAction::StartGame => {
                    info!("StartGame button clicked");
                    game_state.set(GameState::Playing).unwrap();
                }
                MenuButtonAction::RestartGame => {
                    info!("RestartGame button clicked");
                    game_state.set(GameState::Playing).unwrap();
                }
                MenuButtonAction::BackToMainMenu => {
                    info!("BackToMainMenu button clicked");
                    game_state.set(GameState::MainMenu).unwrap();
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
