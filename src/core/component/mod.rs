pub mod audio_clip;
pub mod image;
pub mod offset;
pub mod pre_frame;
pub mod text;

use std::any::Any;

use crate::app::App;

pub trait Component {
    fn get_name(&self) -> &str;
    fn z_index(&self) -> i32 {
        0
    }
    fn render(&self, _app: &App, _parent_offset: (f32, f32)) {}
    fn as_any(&mut self) -> &mut dyn Any;
}
