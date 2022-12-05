use sfml::window::Event as SFMLEvent;

pub enum Event {
    SFMLEvent(SFMLEvent),
    ImageLoadEvent(String, u32, u32, u32),
}
