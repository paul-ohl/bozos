use bevy::prelude::*;

use crate::{
    plugins::camera,
    resources::blocks::{BlockBundle, BlockHandles, BlockType},
};

#[derive(Debug, Component)]
pub struct Test {
    i: i32,
}

pub fn setup_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<camera::Camera>>,
) {
    // ground
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((Test { i: 0 },));
}

pub fn update_test(
    mut commands: Commands,
    block_handles: Res<BlockHandles>,
    mut testquery: Query<&mut Test>,
) {
    for mut test in testquery.iter_mut() {
        if test.i == 0 {
            for x in -5..5 {
                for y in -5..5 {
                    for z in -5..5 {
                        let block_type = if y < -2 {
                            BlockType::Stone
                        } else if y == -2 {
                            BlockType::Dirt
                        } else if y == -1 {
                            BlockType::Grass
                        } else {
                            BlockType::Bedrock
                        };
                        let position = Vec3::new(x as f32, y as f32, z as f32);
                        let block_handle = match block_type {
                            BlockType::Dirt => &block_handles.dirt_handle,
                            BlockType::Grass => &block_handles.grass_handle,
                            BlockType::Stone => &block_handles.stone_handle,
                            BlockType::Bedrock => &block_handles.bedrock_handle,
                        };
                        commands.spawn(BlockBundle::new(block_type, position, block_handle));
                    }
                }
            }
        }
        test.i = 1;
    }
}
