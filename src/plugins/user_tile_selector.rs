use bevy::{prelude::*, math::{vec2, uvec2}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
// use bevy_debug_text_overlay::screen_print;

use crate::{mesh_gen::mesh_hexagon_new};

use super::input::MousePosition;

pub struct SelectionPlugin{
    allow_user_select:bool
}
impl SelectionPlugin{
    pub fn new_allow_selection()->Self{
        Self{
            allow_user_select:true
        }
    }
    pub fn new()->Self{
        Self{
            allow_user_select:false
        }
    }
}
#[derive(Resource)]
pub struct UserSelectionControl{
    user_selection_frozen:bool,//user input no longer affects position and the camera no longer lerp to the user position
}
#[derive(Resource,Default)]
pub struct UserSelection{
    pub cursor_tile_grid_position:UVec2,
}

#[derive(Component)]
pub struct SelectionObject;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        // dbg!(self.allow_user_select);
        app.insert_resource(UserSelectionControl{
            user_selection_frozen: !self.allow_user_select,
        }).insert_resource(UserSelection::default()).add_startup_system(selection_init).add_system(selection_positioning);
    }
}

//sin(1/3*pi)
pub const WIDTH_OFFSET_GRID:f32 = 0.86602540378;
//1-cos(1/3*pi)*0.5
pub const HEIGHT_OFFSET_GRID:f32 = 0.75;


fn selection_init(mut commands:Commands,mut meshes:ResMut<Assets<Mesh>>,mut materials:ResMut<Assets<ColorMaterial>>){
    let material = materials.add(ColorMaterial { color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.5 }, texture: None });
    let mesh_hexagon = meshes.add(mesh_hexagon_new(0.5));
    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(mesh_hexagon.clone()),
        material,
        transform:Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        // transform:Transform::from_xyz(x as f32 * WIDTH_OFFSET_GRID+local_x_offset, y as f32 * HEIGHT_OFFSET_GRID, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    }).insert(SelectionObject);
}

//TODO: split the cursor object and grid logic
fn selection_positioning(
    mut position:Local<Vec2>,
    mut query:Query<&mut Transform,With<SelectionObject>>,
    mut selection:ResMut<UserSelection>,
    control:Res<UserSelectionControl>,
    time:Res<Time>,
    mouse:Res<MousePosition>
){
    // screen_print!("pos: {} {}",mouse.get(),control.user_selection_frozen);
    if control.user_selection_frozen {return}
    let Ok(mut transform) = query.get_single_mut() else {return;};
    transform.translation = transform.translation.lerp(position.extend(9.0), 1.0-0.000001f32.powf(time.delta_seconds()));
    let pos = mouse.get();
    let y = (pos.y/HEIGHT_OFFSET_GRID).round() as i32;
    let x_offset = if y % 2 == 1 {WIDTH_OFFSET_GRID * 0.5}else{0.0};
    let x = ((pos.x - x_offset)/WIDTH_OFFSET_GRID).round() as i32;
    let calc_pair = |ix:i32,iy:i32|{
        ((ix,iy),(x as f32 * WIDTH_OFFSET_GRID + if iy % 2 == 1{WIDTH_OFFSET_GRID*0.5}else{0.0},iy as f32 * HEIGHT_OFFSET_GRID))
    };
    let pairs = [
        calc_pair(x,y),
        calc_pair(x+1,y),
        calc_pair(x-1,y),
        calc_pair(x,y+1),
        calc_pair(x+1,y+1),
        calc_pair(x-1,y+1),
        calc_pair(x,y-1),
        calc_pair(x+1,y-1),
        calc_pair(x-1,y-1),
    ];
    let mut least_dist = 1000.0;
    let mut coords = (x,y);
    for (c,(px,py)) in pairs{
        let d = ((pos.x-px).powi(2) + (pos.y-py).powi(2)).sqrt();
        if d < least_dist {
            least_dist = d;
            coords = c;
        }
    };

    
    
    let (x,y) = coords;
    if x > 0 && y > 0 {
        selection.cursor_tile_grid_position = uvec2(x as u32, y as u32);
    }
    // screen_print!("least dist = {} coords = ({},{})",least_dist,x,y);
    let x_offset = if y % 2 == 1 {WIDTH_OFFSET_GRID * 0.5}else{0.0};
    *position = vec2(x as f32 * WIDTH_OFFSET_GRID + x_offset, y as f32 * HEIGHT_OFFSET_GRID);
}