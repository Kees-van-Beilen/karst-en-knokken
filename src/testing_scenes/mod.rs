use bevy::prelude::App;

mod mesh_gen;
mod win;


pub fn pick_scene(name:&str,app_ref:&mut App){
    app_ref.add_plugin(bevy_debug_text_overlay::OverlayPlugin::default());
    match name {
        "mesh_gen1"=>mesh_gen::main(app_ref),
        "mesh_gen2"=>mesh_gen::main2(app_ref),
        "mesh_gen3"=>mesh_gen::main3(app_ref),
        "win1"=>win::main(app_ref),
        e=>panic!("no test scene named: {e}")
    };
}