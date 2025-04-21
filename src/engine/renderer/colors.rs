use wgpu::core::command::AttachmentErrorLocation::Color;

#[derive(Copy, Clone, Debug)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
    Gray,
    Purple,
    Pink,
    Brown,
    Orange,
    Magenta,
    Cyan,
    Custom(u8,u8,u8,u8)
}

impl Colors {
    pub fn as_f32(&self) -> [f32; 4] {
        match self {
            Colors::Red => [1.0, 0.0, 0.0, 1.0],
            Colors::Green => [0.0, 1.0, 0.0, 1.0],
            Colors::Blue => [0.0, 0.0, 1.0, 1.0],
            Colors::Yellow => [1.0, 1.0, 0.0, 1.0],
            Colors::Black => [0.0, 0.0, 0.0, 1.0],
            Colors::White => [1.0, 1.0, 1.0, 1.0],
            Colors::Gray => [0.5, 0.5, 0.5, 1.0],
            Colors::Purple => [0.5, 0.0, 0.5, 1.0],
            Colors::Pink => [1.0, 0.0, 1.0, 1.0],
            Colors::Brown => [0.6, 0.3, 0.1, 1.0],
            Colors::Orange => [1.0, 0.5, 0.0, 1.0],
            Colors::Magenta => [1.0, 0.0, 1.0, 1.0],
            Colors::Cyan => [0.0, 1.0, 1.0, 1.0],
            Colors::Custom(r, g, b, a) => [
                *r as f32 / 255.0,
                *g as f32 / 255.0,
                *b as f32 / 255.0,
                *a as f32 / 255.0,
            ],
        }
    }
}
