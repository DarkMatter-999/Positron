use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use crate::display::DisplayCommand;

pub fn make_window(displaylist: Vec<DisplayCommand>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Positron", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        for element in &displaylist {
            match element {
                DisplayCommand::SolidColor(color, rect) => {
                    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    canvas.fill_rect(Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.width as u32,
                        rect.height as u32,
                    ))?;
                }
                DisplayCommand::Text(color, rect, text) => {
                    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    canvas.fill_rect(Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.width as u32,
                        rect.height as u32,
                    ))?;
                }
            }
        }
        canvas.present();
    }
    Ok(())
}
