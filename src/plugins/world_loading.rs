#![allow(unused)]

use crate::resources::blocks::{BlockBundle, BlockHandle, BlockHandles, load_blocks};
use bevy::prelude::*;

#[derive(Component)]
struct Truc;

pub fn setup_world_loading(
    commands: Commands,
    mesh: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<StandardMaterial>>,
) {
    load_blocks(commands, mesh, material);
}

pub fn update_loading(mut commands: Commands) {}
