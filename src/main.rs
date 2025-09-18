use bevy::prelude::*;
use bozos::plugin_def::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bozos".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .run();
}
