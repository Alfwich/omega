use sfml::window::Event as SFMLEvent;

pub struct ImageLoadEventPayload {
    pub url: String,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
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
    /// Raw SFMLEvent which Entities can respond to
    SFMLEvent(SFMLEvent),

    /// Async image load data is available
    ImageLoadEvent(ImageLoadEventPayload),

    /// Request from parent Entity to update renderable layout
    UpdateRenderable(UpdateRenderablePayload),
}
