use bevy::{prelude::*, render::camera::RenderTarget, window::{self, PrimaryWindow}};
pub struct Input2DPlugin;

impl Plugin for Input2DPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MousePosition>()
        .add_system(input_system);
    }
}

#[derive(Resource,Default,Debug)]
pub struct MousePosition(Vec2);

impl MousePosition {
    #[inline]
    pub fn get(&self)->Vec2{
        self.0
    }
    #[inline]
    fn set(&mut self,to:Vec2){
        self.0 = to;
    }
}

fn target_is_primary_window(target:&RenderTarget)->bool{
    match target {
        RenderTarget::Window(window_ref) => match window_ref {
            window::WindowRef::Primary => true,
            _=>false
        },
        _ => false
    }
}

fn input_system(
    windows: Query<&Window,With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut mouse_position:ResMut<MousePosition>
){
    let Ok(window) = windows.get_single() else {return};
    let Some((camera, camera_transform)) = camera_q.iter()
        .find(|(c,_)|target_is_primary_window(&c.target)) else {return};
    
    let Some(position) = window.cursor_position()
        .and_then(|viewport_position|camera.viewport_to_world_2d(camera_transform, viewport_position)) else {return};

    mouse_position.set(position);
}
