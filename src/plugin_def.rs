use bevy::prelude::*;

use crate::plugins::camera::{setup_camera, update_camera};
use crate::plugins::test::{setup_test, update_test};
use crate::plugins::world_loading::{setup_world_loading, update_loading};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

pub struct WorldLoadingPlugin;
impl Plugin for WorldLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_loading)
            .add_systems(Update, update_loading);
    }
}

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_test)
            .add_systems(Update, update_test);
    }
}
