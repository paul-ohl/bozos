use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ConsoleOverlay {
    pub visible: bool,
    pub wsize: f32,
    pub hsize: f32,
    pub color: Color,
}

#[derive(Debug, Component)]
pub struct LogText {
    pub tab: Vec<[String; 100]>,
}

pub fn console_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
        LogText { tab: vec![] },
        ConsoleOverlay {
            visible: false,
            wsize: 600.0,
            hsize: 400.0,
            color: Color::srgb(0.0, 0.0, 0.1),
        },
    ));
}

pub fn console_update() {}
