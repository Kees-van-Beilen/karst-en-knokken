use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::mesh_gen::mesh_hexagon_new;

use super::user_tile_selector::{WIDTH_OFFSET_GRID, HEIGHT_OFFSET_GRID};

pub struct WorldBuilderPlugin;
#[derive(Component)]
pub struct GameWorld{
    pub image_asset_url:String,
}
#[derive(Component)]
pub struct GameWorldData{
    pub solid_terrain:Vec<bool>,
    pub width:usize,
    pub height:usize,
}
#[derive(Component)]
pub struct WorldTempTextureHandle(Handle<Image>);


impl Plugin for WorldBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_world_init)
        .add_system(game_world_load_texture);
    }
}

fn game_world_init(query:Query<(Entity,&GameWorld),Added<GameWorld>>,mut commands:Commands,assets:Res<AssetServer>){
    for (entity,world) in query.iter() {
        //TODO: this should set a fail state if None;
        let Some(mut e) = commands.get_entity(entity) else {continue;};
        e.insert(WorldTempTextureHandle(assets.load(&world.image_asset_url)));
    }
}
fn game_world_load_texture(mut query:Query<(Entity,&WorldTempTextureHandle)>,mut commands:Commands,mut images:ResMut<Assets<Image>>,mut meshes:ResMut<Assets<Mesh>>,mut materials:ResMut<Assets<ColorMaterial>>){
    for (entity,texture_handle) in query.iter_mut() {
        let Some(texture) = images.remove(&texture_handle.0) else {continue;};
        // images.remove(handle)
        let dynamic_texture = texture.try_into_dynamic().unwrap();
        let height = dynamic_texture.height();
        let width = dynamic_texture.width();
        let data = dynamic_texture.as_rgba8().unwrap();
        commands.get_entity(entity).unwrap().remove::<WorldTempTextureHandle>();

        let material = materials.add(ColorMaterial { color: Color::GREEN, texture: None });

        let mesh_hexa = meshes.add(mesh_hexagon_new(0.5));
        let mut spawn_buffer = Vec::with_capacity((width*height) as usize);

        for (x,y,col) in data.enumerate_pixels() {
            if col.0[3] < 10 {continue;};
            let local_x_offset = if y%2 == 1 {WIDTH_OFFSET_GRID*0.5}else{0.0};
            spawn_buffer.push(MaterialMesh2dBundle{
                mesh: Mesh2dHandle(mesh_hexa.clone()),
                material:material.clone(),
                transform:Transform::from_xyz(x as f32 * WIDTH_OFFSET_GRID+local_x_offset, (height-y) as f32 * HEIGHT_OFFSET_GRID, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..Default::default()
            });
        }
        commands.spawn_batch(spawn_buffer);
    }

}

