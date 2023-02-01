use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Score(pub u32);

#[derive(Debug, Component)]
pub struct Scoreboard;

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
