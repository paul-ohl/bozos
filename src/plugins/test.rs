use bevy::prelude::*;

use crate::resources::blocks::{BlockBundle, BlockHandle, BlockHandles, BlockType, load_blocks};

#[derive(Debug, Component)]
pub struct Test {
    iterator: i128,
}

pub fn setup_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    commands.spawn((Test { iterator: 0 },));
}

pub fn update_test(
    mut commands: Commands,
    block_handles: Res<BlockHandles>,
    mut testquery: Query<&mut Test>,
) {
    for mut test in testquery.iter_mut() {
        test.iterator += 1;
        if test.iterator == 1 {
            println!(
                "block_type de block_handle: {:?}",
                block_handles.grass_handle.block_type
            );
            commands.spawn(BlockBundle::new(
                BlockType::Grass,
                Vec3::new(2.0, 0.5, 2.0),
                &block_handles.grass_handle,
            ));
        }
    }
}
