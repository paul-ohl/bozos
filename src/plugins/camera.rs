use bevy::prelude::*;


#[derive(Debug, Component)]
pub struct Camera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera,
    ));
}

pub fn update_camera(// mut commands: Commands,
    // mut camera_query: Query<(Entity, &Camera)>,
    // mut resize_events: EventReader<WindowResized>,
) {
}
