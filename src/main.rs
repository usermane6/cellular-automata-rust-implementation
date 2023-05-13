extern crate sdl2;

use game::State;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;



// use std::time::Duration;

mod display;
use display::{Display, PixelTexture};

mod game;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut n_display = Display::new(video_subsystem, "Cells {rust test}", (800, 800));
    let texture_creator = n_display.texture_creator();
    let mut n_pixel_display = PixelTexture::new(&texture_creator, (800, 800));


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut state = State::random(&mut n_pixel_display);

    // let texture_creator = n_display.canvas.texture_creator();

    // fill_texture(&mut test_texture);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        n_display.canvas.clear();
        // The rest of the game loop goes here...
        // n_display.canvas.copy(&test_texture, None, None).expect("Error copying texture to renderer");

        state = State::from_previous(state);
        state.push_state(&mut n_pixel_display);

        n_display.display_texture(&n_pixel_display);
        n_display.canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
