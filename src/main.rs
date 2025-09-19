use bevy::prelude::*;
use bozos::plugin_def::CameraPlugin;
use bozos::plugin_def::KeysPlugin;
use bozos::plugin_def::TestPlugin;
use bozos::plugin_def::WorldLoadingPlugin;

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
        .add_plugins(WorldLoadingPlugin)
        .add_plugins(KeysPlugin)
        .add_plugins(TestPlugin)
        .run();
}
