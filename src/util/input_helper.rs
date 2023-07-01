use bevy::prelude::{Input, KeyCode};


pub trait InputExtension {
    fn keyboard_axes(&self,min:KeyCode,max:KeyCode)->f32;
    fn keyboard_axes_horizontal(&self)->f32;
    fn keyboard_axes_vertical(&self)->f32;
    ///returns the horizontal and vertical axes
    fn axes_main(&self)->(f32,f32);
}
impl InputExtension for Input<KeyCode> {
    fn keyboard_axes(&self,min:KeyCode,max:KeyCode)->f32{
        return if self.pressed(min) {-1.0} else {0.0} + 
        if self.pressed(max) {1.0} else {0.0}
    }
    fn keyboard_axes_horizontal(&self)->f32{
        return self.keyboard_axes( KeyCode::A, KeyCode::D);
    }
    fn keyboard_axes_vertical(&self)->f32{
        return self.keyboard_axes( KeyCode::S, KeyCode::W);
    }
    
    ///returns the horizontal and vertical axes
    fn axes_main(&self)->(f32,f32){
        return (
            self.keyboard_axes_horizontal(),
            self.keyboard_axes_vertical()
        )
    }
}