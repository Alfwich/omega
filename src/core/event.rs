use crate::util::rect::Rect;
use sfml::window::Event as SFMLEvent;

pub struct ImageLoadEventPayload {
    pub url: String,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

pub enum Event {
    SFMLEvent(SFMLEvent),
    ImageLoadEvent(ImageLoadEventPayload),
}
