use bevy::prelude::*;

use crate::plugins::{user_camera_controller::CameraPlugin, windowing::{WindowManager, WindowingPlugin}, input::Input2DPlugin};

pub fn main(app:&mut App){
    app.add_plugin(CameraPlugin::new()).add_plugin(WindowingPlugin).add_plugin(Input2DPlugin).add_startup_system(start_spawn_text_box_center);
}

pub fn start_spawn_text_box_center(
    mut manager:ResMut<WindowManager>
){
    manager.createTextBox("Hello world this is a textbox".to_string());
}