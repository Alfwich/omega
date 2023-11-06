use sfml::window::Event as SFMLEvent;

use super::{renderer::window::WindowConfig, resource::AsyncLoadHandle};

#[derive(Debug, Clone, Copy)]
pub struct ImageLoadEventPayload {
    pub handle: AsyncLoadHandle,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateRenderablePayload {
    X(f32),
    Y(f32),
    MoveX(f32),
    MoveY(f32),
    Width(f32),
    Height(f32),
    Rotation(f32),
    ScaleX(f32),
    ScaleY(f32),
    ColorMod(f32, f32, f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Raw SFMLEvent
    SFMLEvent(SFMLEvent),

    /// Async image load data is available
    ImageLoadEvent(ImageLoadEventPayload),

    /// Request from parent Entity to update some renderable feature
    UpdateRenderable(UpdateRenderablePayload),

    // Window has been changed
    WindowUpdated(WindowConfig),
}
