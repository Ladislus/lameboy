use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::log;

pub fn launch_gui() {
    let Ok(sdl_context) = sdl2::init() else { todo!() };
    let Ok(video_subsystem) = sdl_context.video() else { todo!() };

    log!("GUI", format!("Platform is \"{}\"", sdl2::get_platform()));

    let Ok(window) = video_subsystem.window("rust-sdl2 demo", 800, 600).position_centered().build() else { todo!() };

    let Ok(mut canvas) = window.into_canvas().build() else { todo!() };

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();
    let Ok(mut event_pump) = sdl_context.event_pump() else { todo!() };
    let mut i: u8 = 0;
    'running: loop {
        i = i.wrapping_add(1);
        canvas.set_draw_color(Color::RGB(i, 65, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }
}