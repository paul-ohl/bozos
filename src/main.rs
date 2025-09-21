use bevy::log::LogPlugin;
use bevy::prelude::*;
use bozos::plugin_def::CameraPlugin;
use bozos::plugin_def::KeysPlugin;
use bozos::plugin_def::TestPlugin;
use bozos::plugin_def::WorldLoadingPlugin;

const LOG_FILTERS: &str =
    "debug,bevy_render=info,bevy_app=info,wgpu_hal=warn,naga=warn,offset_allocator=warn";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bozos".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: String::from(LOG_FILTERS),
                    level: bevy::log::Level::DEBUG,
                    ..Default::default()
                }),
        )
        .add_plugins(CameraPlugin)
        .add_plugins(WorldLoadingPlugin)
        .add_plugins(KeysPlugin)
        .add_plugins(TestPlugin)
        //.add_plugins(ConsolePlugin)
        .run();
}
