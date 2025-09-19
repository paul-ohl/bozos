use std::{fs::File, io::Read};

use bevy::{
    ecs::component,
    input::{
        ButtonState,
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

use serde::Deserialize;

use crate::{get_single_component, get_single_mut_component};

#[derive(Debug, Deserialize)]
pub enum KeyBinding {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
}

#[derive(Debug, Deserialize, Component)]
pub struct KeysConfig {
    pub move_forward: Vec<KeyBinding>,
    pub move_backward: Vec<KeyBinding>,
    pub move_left: Vec<KeyBinding>,
    pub move_right: Vec<KeyBinding>,
    pub move_up: Vec<KeyBinding>,
    pub move_down: Vec<KeyBinding>,
    pub exit: Vec<KeyBinding>,
    pub interact: Vec<KeyBinding>,
    pub option: Vec<KeyBinding>,
    pub camera_mode: Vec<KeyBinding>,
}

#[derive(Debug, Component)]
pub struct Keys {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub exit: bool,
    pub interact: bool,
    pub option: bool,
    pub camera_mode: bool,
}

pub fn keys_setup(mut commands: Commands) {
    commands.spawn((Keys {
        move_forward: false,
        move_backward: false,
        move_left: false,
        move_right: false,
        move_up: false,
        move_down: false,
        exit: false,
        interact: false,
        option: false,
        camera_mode: false,
    },));

    commands.spawn((KeysConfig {
        move_forward: vec![KeyBinding::KeyCode(KeyCode::KeyW)],
        move_backward: vec![KeyBinding::KeyCode(KeyCode::KeyS)],
        move_left: vec![KeyBinding::KeyCode(KeyCode::KeyA)],
        move_right: vec![KeyBinding::KeyCode(KeyCode::KeyD)],
        move_up: vec![KeyBinding::KeyCode(KeyCode::Space)],
        move_down: vec![KeyBinding::KeyCode(KeyCode::ShiftLeft)],
        exit: vec![KeyBinding::KeyCode(KeyCode::Escape)],
        interact: vec![KeyBinding::MouseButton(MouseButton::Left)],
        option: vec![KeyBinding::MouseButton(MouseButton::Right)],
        camera_mode: vec![KeyBinding::KeyCode(KeyCode::KeyC)],
    },));
    // let mut file =
    //     File::open("./settings/keys.toml").expect("Problem while opening the config file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let keys_config: KeysConfig =
    //     toml::from_str(&contents).expect("The config file is incorrectly formatted");
}

pub fn keys_update(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut evr_mouse: EventReader<MouseButtonInput>,
    mut evr_wheel: EventReader<MouseWheel>,
    mut evr_motion: EventReader<MouseMotion>,
    mut keys_query: Query<&mut Keys>,
    keys_config_query: Query<&KeysConfig>,
) {
    let mut keys = get_single_mut_component!(keys_query);
    let keys_config = get_single_component!(keys_config_query);
    println!("move forward: {:?} ", keys.move_forward);
    for ev in evr_kbd.read() {
        for key_binding in &keys_config.move_forward {
            if let KeyBinding::KeyCode(key_code) = key_binding {
                match ev.state {
                    ButtonState::Pressed => {
                        if &ev.key_code == key_code {
                            keys.move_forward = true;
                        }
                    }
                    ButtonState::Released => {
                        if &ev.key_code == key_code {
                            keys.move_forward = false;
                            println!("Key release: {:?} ({:?})", ev.key_code, ev.logical_key);
                        }
                    }
                }
            }
        }
    }

    for ev in evr_mouse.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }

    for ev in evr_wheel.read() {
        println!("Mouse wheel: x: {:?}, y: {:?}", ev.x, ev.y);
    }

    for ev in evr_motion.read() {
        println!("Mouse motion: x: {:?}, y: {:?}", ev.delta.x, ev.delta.y);
    }
}
