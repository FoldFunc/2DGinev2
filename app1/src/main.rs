use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use engine::{Canvas, Circle, Point, Square};

fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new("My SDL2 Window", 800, 600)?;
    let cicrcle = Rc::new(RefCell::new(Circle::new(
        200,
        200,
        100,
        vec![255, 255, 255],
    )));
    let pixel = Rc::new(RefCell::new(Point::new(100, 100, vec![255, 255, 255])));
    let square_no_fill = Rc::new(RefCell::new(Square::new(
        500,
        500,
        100,
        100,
        vec![255, 255, 255],
    )));
    let square_fill = Rc::new(RefCell::new(Square::new(
        400,
        400,
        100,
        100,
        vec![100, 100, 100],
    )));

    // Key events for moving the pixel
    {
        let rect = Rc::clone(&square_fill);
        canvas.e_add_key_event("L", move || {
            let (x, y) = {
                let r = rect.borrow();
                (r.x, r.y)
            };
            rect 
                .borrow_mut()
                .change_pos(x + 10, y, vec![100, 100, 100]);
        })?;
    }
    {
        let rect = Rc::clone(&square_fill);
        canvas.e_add_key_event("H", move || {
            let (x, y) = {
                let r = rect.borrow();
                (r.x, r.y)
            };
            rect 
                .borrow_mut()
                .change_pos(x - 10, y, vec![100, 100, 100]);
        })?;
    }
    {
        let rect = Rc::clone(&square_fill);
        canvas.e_add_key_event("K", move || {
            let (x, y) = {
                let r = rect.borrow();
                (r.x, r.y)
            };
            rect 
                .borrow_mut()
                .change_pos(x, y - 10, vec![100, 100, 100]);
        })?;
    }
    {
        let rect = Rc::clone(&square_fill);
        canvas.e_add_key_event("J", move || {
            let (x, y) = {
                let r = rect.borrow();
                (r.x, r.y)
            };
            rect 
                .borrow_mut()
                .change_pos(x, y + 10, vec![100, 100, 100]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("D", move || {
            let (x, y) = {
                let p = pixel.borrow();
                (p.x, p.y)
            };
            pixel
                .borrow_mut()
                .change_pos(x + 10, y, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("A", move || {
            let (x, y) = {
                let p = pixel.borrow();
                (p.x, p.y)
            };
            pixel
                .borrow_mut()
                .change_pos(x - 10, y, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("W", move || {
            let (x, y) = {
                let p = pixel.borrow();
                (p.x, p.y)
            };
            pixel
                .borrow_mut()
                .change_pos(x, y - 10, vec![200, 200, 200]);
        })?;
    }
    {
        let pixel = Rc::clone(&pixel);
        canvas.e_add_key_event("S", move || {
            let (x, y) = {
                let p = pixel.borrow();
                (p.x, p.y)
            };
            pixel
                .borrow_mut()
                .change_pos(x, y + 10, vec![200, 200, 200]);
        })?;
    }

    // Other key events
    canvas.e_add_key_event("ESCAPE", || {
        println!("Escape pressed, exiting...");
        std::process::exit(0);
    })?;

    canvas.e_add_key_event("SPACE", || {
        println!("Space pressed!");
    })?;

    // Rendering loop
    let pixel_for_render = Rc::clone(&pixel);
    let square_for_render_no_fill = Rc::clone(&square_no_fill);
    let square_for_render_fill = Rc::clone(&square_fill);
    let circle_for_render_fill = Rc::clone(&cicrcle);
    canvas.run(move |canvas| {
        // Clear screen & background
        canvas.e_background(vec![0, 0, 0])?;

        // Draw objects
        canvas.e_draw_pixel(&pixel_for_render.borrow())?;
        canvas.e_draw_sqare(&square_for_render_no_fill.borrow(), Some(false))?;
        canvas.e_draw_sqare(&square_for_render_fill.borrow(), Some(true))?;
        canvas.e_draw_circle(&circle_for_render_fill.borrow(), Some(true))?;
        Ok(())
    })?;

    Ok(())
}
