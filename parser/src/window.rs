use crate::display::DisplayCommand;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::time::Duration;

pub fn make_window(
    width: u32,
    height: u32,
    displaylist: Vec<DisplayCommand>,
) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Positron", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut FontMap: HashMap<u16, Font> = HashMap::new();

    let texture_creator = canvas.texture_creator();

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
                    let pointheight = rect.height * 0.75;

                    if let Some(font) = FontMap.get(&(pointheight as u16)) {
                        let surface = font
                            .render(text)
                            .blended(Color::RGBA(color.r, color.g, color.b, color.a))
                            .map_err(|e| e.to_string())?;

                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .map_err(|e| e.to_string())?;

                        canvas.copy(
                            &texture,
                            None,
                            Some(Rect::new(
                                rect.x as i32,
                                rect.y as i32,
                                surface.width() as u32,
                                surface.height() as u32,
                            )),
                        )?;
                    } else {
                        let pointheight = rect.height * 0.75;
                        let f = ttf_context
                            .load_font("./assets/OpenSans-Regular.ttf", pointheight as u16)?;
                        FontMap.insert(pointheight as u16, f);

                        if let Some(font) = FontMap.get(&(pointheight as u16)) {
                            let surface = font
                                .render(text)
                                .blended(Color::RGBA(color.r, color.g, color.b, color.a))
                                .map_err(|e| e.to_string())?;

                            let texture = texture_creator
                                .create_texture_from_surface(&surface)
                                .map_err(|e| e.to_string())?;

                            canvas.copy(
                                &texture,
                                None,
                                Some(Rect::new(
                                    rect.x as i32,
                                    rect.y as i32,
                                    surface.width() as u32,
                                    surface.height() as u32,
                                )),
                            )?;
                        }
                    };
                }
            }
        }
        canvas.present();
    }
    Ok(())
}
