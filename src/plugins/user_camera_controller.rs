use bevy::{prelude::*, math::vec2};

use crate::util::extensions::InputExtension;

pub struct CameraPlugin{
    allow_user_input:bool
}
impl CameraPlugin{
    pub fn new_user_controlled()->Self{
        Self{
            allow_user_input:true
        }
    }
    pub fn new()->Self{
        Self{
            allow_user_input:false
        }
    }
}
#[derive(Resource)]
pub struct UserCameraControl{
    user_positioning_frozen:bool,//user input no longer affects position and the camera no longer lerp to the user position

}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UserCameraControl{
            user_positioning_frozen: !self.allow_user_input,
        }).add_startup_system(camera_init).add_system(camera_positioning);
    }
}
fn camera_init(mut commands:Commands){
    commands.spawn(Camera2dBundle{
        transform:Transform::from_xyz(0.0,0.0,10.0),
        projection:OrthographicProjection{
            scaling_mode:bevy::render::camera::ScalingMode::AutoMin { min_width: 10.0, min_height: 10.0 },
            ..Default::default()
        },
        ..Default::default()
    });
}
fn camera_positioning(mut position:Local<Vec2>,mut query:Query<&mut Transform,With<Camera2d>>,control:Res<UserCameraControl>,time:Res<Time>,input:Res<Input<KeyCode>>){
    if control.user_positioning_frozen {return}
    let Ok(mut transform) = query.get_single_mut() else {return;};
    transform.translation = transform.translation.lerp(position.extend(10.0), 1.0-0.05f32.powf(time.delta_seconds()));
    let (h,v) = input.axes_main();
    *position += vec2(h, v) * time.delta_seconds() * 10.0;
}