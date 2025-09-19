use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Camera {
    cameramod: bool,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-100.5, 100.5, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { cameramod: false },
    ));
}

pub fn update_camera() {}
