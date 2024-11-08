use sdl2::pixels;
use crate::chip8::config::BACKGROUND_COLOR;
use crate::chip8::config::FOREGROUND_COLOR;

#[allow(dead_code)]
pub enum Colors {
    Black,
    Green,
    White,
    Red,
}

impl Colors {
    pub fn as_color(self) -> pixels::Color {
        match self {
            Self::Black => pixels::Color::RGB(0, 0, 0),
            Self::Green => pixels::Color::RGB(0, 250, 0),
            Self::White => pixels::Color::RGB(255, 255, 255),
            Self::Red => pixels::Color::RGB(255, 99, 71),
        }
    }
    pub fn from_u8(num: u8) -> Self {
        match num {
            0 => { BACKGROUND_COLOR }
            _ => { FOREGROUND_COLOR } 
        }
    }
}
