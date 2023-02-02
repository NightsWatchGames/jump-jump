use bevy::prelude::*;
use rand::Rng;

// 当前所站的平台
#[derive(Debug, Component)]
pub struct CurrentPlatform;

// 下一个平台
#[derive(Debug, Component)]
pub struct NextPlatform;

pub fn setup_first_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        CurrentPlatform,
    ));
}

// 生成下一个平台
// TODO 圆柱形平台
pub fn generate_next_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_current_platform: Query<&Transform, With<CurrentPlatform>>,
    q_next_platform: Query<Entity, With<NextPlatform>>,
) {
    if q_next_platform.is_empty() {
        for current_platform in &q_current_platform {
            let mut rng = rand::thread_rng();
            let rand_distance = rng.gen_range(1.5..3.0);
            let next_pos = if rng.gen_bool(0.5) {
                Vec3::new(
                    current_platform.translation.x + rand_distance,
                    0.5,
                    current_platform.translation.z,
                )
            } else {
                Vec3::new(
                    current_platform.translation.x,
                    0.5,
                    current_platform.translation.z - rand_distance,
                )
            };

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(rand_platform_color().into()),
                    transform: Transform::from_translation(next_pos),
                    ..default()
                },
                NextPlatform,
            ));
        }
    }
}

// TODO 平台蓄力效果
pub fn animate_platform_accumulation() {}

fn rand_platform_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

// 是否落到平台上
pub fn is_landed_on_platform(platform_pos: Vec3, landing_pos: Vec3) -> bool {
    dbg!(platform_pos);
    dbg!(landing_pos);
    (landing_pos.x - platform_pos.x).abs() < 0.5 && (landing_pos.z - platform_pos.z).abs() < 0.5
}
