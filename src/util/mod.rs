pub mod alpha;
pub mod color;
pub mod rect;
pub mod scale;
pub mod timer;

pub fn clamp<T: std::cmp::PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
