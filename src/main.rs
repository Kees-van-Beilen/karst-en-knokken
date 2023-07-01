use std::env::args;
use bevy::prelude::*;
mod testing_scenes;
pub mod plugins;
pub mod util;
pub mod mesh_gen;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
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
