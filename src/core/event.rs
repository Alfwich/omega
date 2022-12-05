use sfml::window::Event as SFMLEvent;

pub struct ImageLoadEventPayload(pub String, pub u32, pub u32, pub u32);

pub enum Event {
    SFMLEvent(SFMLEvent),
    ImageLoadEvent(ImageLoadEventPayload),
}
