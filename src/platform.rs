use bevy::prelude::*;
use rand::Rng;

use crate::player::Accumulator;

// 当前所站的平台
#[derive(Debug, Component)]
pub struct CurrentPlatform;

// 下一个平台
#[derive(Debug, Component)]
pub struct NextPlatform;

#[derive(Debug, Component)]
pub enum PlatformShape {
    Box,
    Cylinder,
}

impl PlatformShape {
    pub fn mesh(&self) -> Mesh {
        match self {
            Self::Box => Mesh::from(Cuboid::new(1.5, 1.0, 1.5)),
            Self::Cylinder => Mesh::from(Cylinder::new(0.75, 1.0)),
        }
    }
    // 是否落到平台上
    pub fn is_landed_on_platform(&self, platform_pos: Vec3, landing_pos: Vec3) -> bool {
        dbg!(platform_pos);
        dbg!(landing_pos);
        match self {
            Self::Box => {
                (landing_pos.x - platform_pos.x).abs() < 1.5 / 2.0
                    && (landing_pos.z - platform_pos.z).abs() < 1.5 / 2.0
            }
            Self::Cylinder => {
                (landing_pos.x - platform_pos.x).abs() < 0.75
                    && (landing_pos.z - platform_pos.z).abs() < 0.75
            }
        }
    }
    // 是否接触到角色
    pub fn is_touched_player(
        &self,
        platform_pos: Vec3,
        landing_pos: Vec3,
        player_radius: f32,
    ) -> bool {
        match self {
            Self::Box => {
                (landing_pos.x - platform_pos.x).abs() < (1.5 / 2.0 + player_radius)
                    && (landing_pos.z - platform_pos.z).abs() < (1.5 / 2.0 + player_radius)
            }
            Self::Cylinder => {
                (landing_pos.x - platform_pos.x).abs() < (0.75 + player_radius)
                    && (landing_pos.z - platform_pos.z).abs() < (0.75 + player_radius)
            }
        }
    }
}

pub fn setup_first_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let platform_shape = rand_platform_shape();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(platform_shape.mesh()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        CurrentPlatform,
        platform_shape,
    ));
}

// 生成下一个平台
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
            let rand_distance = rng.gen_range(2.5..4.0);
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

            let platform_shape = rand_platform_shape();
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(platform_shape.mesh()),
                    material: materials.add(rand_platform_color()),
                    transform: Transform::from_translation(next_pos),
                    ..default()
                },
                NextPlatform,
                platform_shape,
            ));
        }
    }
}

// 平台蓄力效果
pub fn animate_platform_accumulation(
    accumulator: Res<Accumulator>,
    mut q_current_platform: Query<&mut Transform, With<CurrentPlatform>>,
    time: Res<Time>,
) {
    let mut current_platform = q_current_platform.single_mut();
    match accumulator.0 {
        Some(_) => {
            current_platform.scale.y =
                (current_platform.scale.y - 0.15 * time.delta_seconds()).max(0.6);
        }
        None => {
            // TODO 回弹效果
            current_platform.scale = Vec3::ONE;
        }
    }
}

pub fn clear_platforms(mut commands: Commands, q_platforms: Query<Entity, With<PlatformShape>>) {
    for platform in &q_platforms {
        commands.entity(platform).despawn();
    }
}

fn rand_platform_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

fn rand_platform_shape() -> PlatformShape {
    let mut rng = rand::thread_rng();
    let selection = rng.gen_range(0..2);
    match selection {
        0 => PlatformShape::Box,
        1 => PlatformShape::Cylinder,
        _ => PlatformShape::Box,
    }
}
