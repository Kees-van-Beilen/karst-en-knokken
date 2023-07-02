use std::env::args;
use bevy::prelude::*;
use settings::SettingsPlugin;
mod testing_scenes;
pub mod plugins;
pub mod util;
pub mod mesh_gen;
pub mod settings;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugin(SettingsPlugin);
    //read from the commandline if we need to use any testing scene
    #[cfg(feature = "dev")]
    {
        for args in args() {
            let mut v = args.split("=");
            if Some("test_scene") == v.next() { 
                let Some(scene_name) = v.next() else {continue;};
                testing_scenes::pick_scene(scene_name, &mut app);
            }
        }
    }
    

    app.run();
}
