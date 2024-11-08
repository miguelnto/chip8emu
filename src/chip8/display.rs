use sdl2::render::Canvas;
use sdl2;
use sdl2::video::Window;

use crate::CHIP8_DEFAULT_WIDTH;
use crate::CHIP8_DEFAULT_HEIGHT;
use crate::chip8::config::*;
use crate::chip8::colors::Colors;

const WINDOW_WIDTH: u32 = CHIP8_DEFAULT_WIDTH as u32 * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = CHIP8_DEFAULT_HEIGHT as u32 * WINDOW_SCALE;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(&WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(BACKGROUND_COLOR.as_color());
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_DEFAULT_WIDTH]; CHIP8_DEFAULT_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let _x = (x as u32) * WINDOW_SCALE;
                let _y: u32 = (y as u32) * WINDOW_SCALE;

                self.canvas.set_draw_color(Colors::from_u8(col).as_color());
                let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(_x as i32, _y as i32, WINDOW_SCALE, WINDOW_SCALE));
            }
        }
        self.canvas.present();
    }
}



