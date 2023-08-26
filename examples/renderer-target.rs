extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::{Color, PixelFormatEnum};
use sdl3::render::{FRect, FPoint};

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl3 resource-manager demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

    let mut angle = 0.0;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        angle = (angle + 0.5) % 360.;
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                texture_canvas
                    .fill_rect(Some(FRect::new(0.0f32, 0.0f32, 400.0f32, 300.0f32)))
                    .expect("could not fill rect");
            })
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(FRect::new(0.0f32, 0.0f32, 400.0f32, 300.0f32));
        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            angle,
            Some(FPoint::new(400.0f32, 300.0f32)),
            false,
            false,
        )?;
        canvas.present();
    }

    Ok(())
}
