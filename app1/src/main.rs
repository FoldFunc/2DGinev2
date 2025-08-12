use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use engine::Point;
use engine::Canvas;
fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new("My SDL2 Window", 800, 600)?;

    let pixel = Rc::new(RefCell::new(engine::Point::new(
        100, 100,
        vec![100, 100, 100],
    )));

    // Clone Rc for each closure
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("D", move || {
            let pixel_x = pixel.borrow().x;
            let pixel_y = pixel.borrow().y;
            pixel.borrow_mut().change_pos(pixel_x + 100, pixel_y, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("A", move || {
            let pixel_x = pixel.borrow().x;
            let pixel_y = pixel.borrow().y;
            pixel.borrow_mut().change_pos(pixel_x - 100, pixel_y, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("W", move || {
            let pixel_x = pixel.borrow().x;
            let pixel_y = pixel.borrow().y;
            pixel.borrow_mut().change_pos(pixel_x, pixel_y - 100, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("S", move || {
            let pixel_x = pixel.borrow().x;
            let pixel_y = pixel.borrow().y;
            pixel.borrow_mut().change_pos(pixel_x, pixel_y + 100, vec![200, 200, 200]);
        })?;
    }

    canvas.e_add_key_event("ESCAPE", || {
        println!("Escape pressed, exiting...");
        std::process::exit(0);
    })?;

    canvas.e_add_key_event("SPACE", || {
        println!("Space pressed!");
    })?;

    let pixel_for_render = Rc::clone(&pixel);
    canvas.run(move |canvas| {
        canvas.e_set_draw_color(0, 0, 0)?;
        canvas.e_clear()?;
        canvas.e_draw_pixel(&pixel_for_render.borrow())?;
        Ok(())
    })?;

    Ok(())
}

