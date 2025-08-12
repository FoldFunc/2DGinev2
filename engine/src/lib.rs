extern crate sdl2;

use sdl2::rect::{Point as PointS, Rect};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::time::Duration;

pub struct Canvas {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    _sdl_context: sdl2::Sdl,      // Keep SDL context alive
    _event_pump: sdl2::EventPump, // For polling events
    events: HashMap<Keycode, Box<dyn FnMut()>>,
}

#[derive(Clone)]
pub struct Circle {
    x: i32,
    y: i32,
    r: u32,
    pub color: Color,
}
impl Circle {
    pub fn new(x: i32, y: i32, r: u32, colors: Vec<u8>) -> Self {
        Circle {
            x: x,
            y: y,
            r: r,
            color: Color::RGB(colors[0], colors[1], colors[2]),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, fill: Option<bool>) {
        canvas.canvas.set_draw_color(self.color);

        let radius = self.r as i32;
        let center_x = self.x;
        let center_y = self.y;

        if fill.unwrap_or(false) {
            // Filled circle: draw horizontal lines inside the circle
            for dy in -radius..=radius {
                let dx = ((radius * radius - dy * dy) as f64).sqrt() as i32;
                let start_x = center_x - dx;
                let end_x = center_x + dx;
                let _ = canvas.canvas.draw_line(PointS::new(start_x, center_y + dy), PointS::new(end_x, center_y + dy));
            }
        } else {
            // Outline circle: draw points using midpoint circle algorithm
            let mut x = radius;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                let points = [
                    PointS::new(center_x + x, center_y + y),
                    PointS::new(center_x + y, center_y + x),
                    PointS::new(center_x - y, center_y + x),
                    PointS::new(center_x - x, center_y + y),
                    PointS::new(center_x - x, center_y - y),
                    PointS::new(center_x - y, center_y - x),
                    PointS::new(center_x + y, center_y - x),
                    PointS::new(center_x + x, center_y - y),
                ];

                for &point in &points {
                    let _ = canvas.canvas.draw_point(point);
                }

                y += 1;
                if err <= 0 {
                    err += 2 * y + 1;
                } else {
                    x -= 1;
                    err -= 2 * x + 1;
                }
            }
        }
    }
    pub fn change_pos(&mut self, x: i32, y: i32, color: Vec<u8>) {
        self.x = x;
        self.y = y;
        self.color = Color::RGB(color[0], color[1], color[2]);
    }
}
#[derive(Clone)]
pub struct Square {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub color: Color,
}

impl Square {
    pub fn new(x: i32, y: i32, w: u32, h: u32, colors: Vec<u8>) -> Self {
        Square {
            x,
            y,
            w,
            h,
            color: Color::RGB(colors[0], colors[1], colors[2]),
        }
    }
    pub fn draw(&self, canvas: &mut Canvas, fill: Option<bool>) {
        if fill.unwrap_or(false) {
            canvas.canvas.set_draw_color(self.color);
            let _ = canvas
                .canvas
                .fill_rect(Rect::new(self.x, self.y, self.w, self.h));
        } else {
            canvas.canvas.set_draw_color(self.color);
            let _ = canvas
                .canvas
                .draw_rect(Rect::new(self.x, self.y, self.w, self.h));
        }
    }
    pub fn change_pos(&mut self, x: i32, y: i32, color: Vec<u8>) {
        self.x = x;
        self.y = y;
        self.color = Color::RGB(color[0], color[1], color[2]);
    }
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32, color: Vec<u8>) -> Self {
        Point {
            x,
            y,
            color: Color::RGB(color[0], color[1], color[2]),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.canvas.set_draw_color(self.color);
        let _ = canvas.canvas.draw_point(PointS::new(self.x, self.y));
    }

    pub fn change_pos(&mut self, x: i32, y: i32, color: Vec<u8>) {
        self.x = x;
        self.y = y;
        self.color = Color::RGB(color[0], color[1], color[2]);
    }
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Canvas {
            canvas,
            _sdl_context: sdl_context,
            _event_pump: event_pump,
            events: HashMap::new(),
        })
    }

    pub fn e_background(&mut self, colors: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.canvas
            .set_draw_color(Color::RGB(colors[0], colors[1], colors[2]));
        self.canvas.clear();
        Ok(())
    }

    pub fn e_draw_pixel(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        point.draw(self);
        Ok(())
    }

    pub fn e_draw_sqare(&mut self, square: &Square, fill: Option<bool>) -> Result<(), Box<dyn Error>> {
        square.draw(self, fill);
        Ok(())
    }
    pub fn e_draw_circle(&mut self, circle: &Circle, fill: Option<bool>) -> Result<(), Box<dyn Error>> {
        circle.draw(self, fill);
        Ok(())
    }
    pub fn e_clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.canvas.clear();
        Ok(())
    }

    pub fn e_add_key_event<F>(&mut self, key: &str, key_handler: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut() + 'static,
    {
        let keycode = match key.to_uppercase().as_str() {
            "ESCAPE" => Keycode::Escape,
            "SPACE" => Keycode::Space,
            "RETURN" => Keycode::Return,
            "A" => Keycode::A,
            "D" => Keycode::D,
            "W" => Keycode::W,
            "S" => Keycode::S,
            "L" => Keycode::L,
            "K" => Keycode::K,
            "J" => Keycode::J,
            "H" => Keycode::H,
            _ => return Err(format!("Invalid key code: {:?}", key).into()),
        };

        self.events.insert(keycode, Box::new(key_handler));
        Ok(())
    }

    pub fn run<R>(&mut self, mut render: R) -> Result<(), Box<dyn Error>>
    where
        R: FnMut(&mut Canvas) -> Result<(), Box<dyn Error>>,
    {
        'running: loop {
            for event in self._event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if let Some(callback) = self.events.get_mut(&key) {
                            callback();
                        }
                    }
                    _ => {}
                }
            }

            render(self)?;
            self.canvas.present(); // only once per frame

            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
