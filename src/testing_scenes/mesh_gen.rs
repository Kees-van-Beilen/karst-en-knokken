// use std::intrinsics::sqrtf32;

use bevy::{prelude::{App, Commands, Assets, Mesh, ResMut, Color, Transform, Quat}, sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{plugins::{user_camera_controller::CameraPlugin, input::Input2DPlugin, user_tile_selector::SelectionPlugin, world_builder::{WorldBuilderPlugin, GameWorld}}, mesh_gen::{mesh_box_new, mesh_ngon_new, mesh_hexagon_new}};

pub fn main(app:&mut App){
    app.add_plugin(CameraPlugin::new_user_controlled()).add_startup_system(start);
}
pub fn main2(app:&mut App){
    app
        .add_plugin(CameraPlugin::new_user_controlled())
        .add_plugin(Input2DPlugin)
        .add_plugin(SelectionPlugin::new_allow_selection())
        .add_startup_system(start_grid);
}
pub fn main3(app:&mut App){
    app
        .add_plugin(CameraPlugin::new_user_controlled())
        .add_plugin(Input2DPlugin)
        .add_plugin(WorldBuilderPlugin)
        .add_plugin(SelectionPlugin::new_allow_selection())
        .add_startup_system(start_world_builder);
}

pub fn start(mut commands:Commands,mut meshes:ResMut<Assets<Mesh>>,mut materials:ResMut<Assets<ColorMaterial>>){
    let mesh_cube = meshes.add(mesh_box_new(1.0, 1.0));
    let mesh_circ = meshes.add(mesh_ngon_new(32,0.5));
    let mesh_hexa = meshes.add(mesh_hexagon_new(0.5));
    let material = materials.add(ColorMaterial { color: Color::GREEN, texture: None });

    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(mesh_cube),
        material:material.clone_weak(),
        ..Default::default()
    });

    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(mesh_circ),
        material:material.clone_weak(),
        transform:Transform::from_xyz(2.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(mesh_hexa.clone_weak()),
        material:material.clone_weak(),
        transform:Transform::from_xyz(-2.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(mesh_hexa),
        material,
        transform:Transform::from_xyz(-4.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    });
}

//sin(1/3*pi)
const WIDTH_OFFSET_GRID:f32 = 0.86602540378;
//1-cos(1/3*pi)*0.5
const HEIGHT_OFFSET_GRID:f32 = 0.75;

pub fn start_grid(mut commands:Commands,mut meshes:ResMut<Assets<Mesh>>,mut materials:ResMut<Assets<ColorMaterial>>){
    let material1 = materials.add(ColorMaterial { color: Color::GREEN, texture: None });
    let material2 = materials.add(ColorMaterial { color: Color::RED, texture: None });
    let material3 = materials.add(ColorMaterial { color: Color::BLUE, texture: None });
    let material4 = materials.add(ColorMaterial { color: Color::YELLOW, texture: None });
    let material5 = materials.add(ColorMaterial { color: Color::PURPLE, texture: None });
    // let mesh_hexa = meshes.add(mesh_hexagon_new(0.5));
    let materials = [material1,material2,material3,material4,material5];
    let mut materials = materials.iter().cycle();
    let mesh_hexa = meshes.add(mesh_hexagon_new(0.5));
    for y in 0..5 {
        let local_x_offset = if y%2 == 1 {WIDTH_OFFSET_GRID*0.5}else{0.0};
        for x in 0..7 {
            commands.spawn(MaterialMesh2dBundle{
                mesh: Mesh2dHandle(mesh_hexa.clone()),
                material:materials.next().unwrap().clone(),
                transform:Transform::from_xyz(x as f32 * WIDTH_OFFSET_GRID+local_x_offset, y as f32 * HEIGHT_OFFSET_GRID, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..Default::default()
            });
        }
    }
}

pub fn start_world_builder(mut commands:Commands){
    commands.spawn_empty().insert(GameWorld{
        image_asset_url: "map_image.png".to_string()
    });
}