extern crate sdl2;

use sdl2::{video::{Window, WindowContext}, VideoSubsystem, render::{Canvas, TextureCreator, Texture}, pixels::{PixelFormatEnum, Color}, surface::Surface};

const PIXEL_FORMAT: PixelFormatEnum = PixelFormatEnum::ARGB8888;
const BYTES_PER_PIXEL: usize = 4; // this is because the ARGB8888 uses 4 bytes
const _WINDOW_SIZE: (usize, usize) = (800, 800);



pub struct Display {
    pub canvas: Canvas<Window>,
}

impl Display {
    pub fn new<'a>(video_subsystem: VideoSubsystem, title: &str, window_size: (u32, u32)) -> Self {
        let n_window = video_subsystem.window(title, window_size.0, window_size.1)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = n_window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
        canvas.clear();

        Display {
            canvas,
        }
    }

    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    pub fn display_texture(&mut self, pixel_display: &PixelTexture) {
        self.canvas.copy(&pixel_display.texture, None, None).expect("Error copying pixels to display");
    }
}

pub struct PixelTexture<'a> {
    pub texture: Texture<'a>,
    pub size: (usize, usize),
    pitch: usize,
}

impl PixelTexture<'_> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>, texture_size: (usize, usize)) -> PixelTexture {
        PixelTexture {
            texture: Surface::new(texture_size.0 as u32, texture_size.1 as u32, PIXEL_FORMAT)
                .unwrap()
                .as_texture(&texture_creator)
                .unwrap(),
            size: texture_size,
            pitch: (texture_size.0) * BYTES_PER_PIXEL,
        }
    }

    pub fn update_texture(&mut self, pixel_slice: &[u8]) {
        self.texture.update(None, pixel_slice, self.pitch).expect("Error updating texture");
    }
}