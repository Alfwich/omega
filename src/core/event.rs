use crate::util::rect::Rect;
use sfml::window::Event as SFMLEvent;

pub struct ImageLoadEventPayload {
    pub url: String,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

pub struct UpdateRenderablePayload {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub w: Option<f32>,
    pub h: Option<f32>,
    pub r: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
}

pub enum Event {
    SFMLEvent(SFMLEvent),
    ImageLoadEvent(ImageLoadEventPayload),
    UpdateRenderable(UpdateRenderablePayload),
}
