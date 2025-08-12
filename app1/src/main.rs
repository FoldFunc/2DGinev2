use engine::Canvas;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new("My SDL2 Window", 800, 600)?;

    canvas.e_add_key_event("ESCAPE", || {
        println!("Escape pressed, exiting...");
        std::process::exit(0);
    })?;

    canvas.e_add_key_event("SPACE", || {
        println!("Space pressed!");
    })?;

    canvas.run(|canvas| {
        canvas.e_set_draw_color(0, 0, 0)?;
        canvas.e_clear()?;
        let _pixel = canvas.e_draw_pixel(100, 100, 100, 100, 100)?;
        canvas.e_clear()?;
        Ok(())
    })?;

    Ok(())
}
