use engine::{Canvas, Font, Image};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new("My SDL2 Window", 800, 600)?;
    let hello = Font::new(
        100,
        100,
        24,
        "src/fonts/font.ttf",
        "Gentelman welcome to fight club",
        vec![100 as u8, 100 as u8, 100 as u8],
    );
    let texture = Image::new_with_path(
        "helloimage",
        "src/images/image.jpg",
        200,
        200,
        200,
        200,
    );
    canvas.run(move |canvas| {
        canvas.e_background(vec![0, 0, 0])?;
        canvas.e_draw_font(&hello)?;
        canvas.e_draw_image(&texture);
        Ok(())
    })?;
    Ok(())
}
