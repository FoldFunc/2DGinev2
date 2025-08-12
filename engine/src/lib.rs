extern crate sdl2;

use sdl2::rect::Point as PointS;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, video};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::time::Duration;

pub struct Canvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    _sdl_context: sdl2::Sdl,      // Keep SDL context alive
    _event_pump: sdl2::EventPump, // For polling events
    events: HashMap<Keycode, Box<dyn FnMut()>>,
}
pub struct Point {
    x: i32,
    y: i32,
    color: Color,
}
impl Point {
    pub fn new(x: i32, y: i32, color: Vec<u8>) -> Self {
        Point { x: x, y: y, color: Color::RGB(color[0], color[1], color[2])}
    }
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.canvas.set_draw_color(self.color);
        canvas.canvas.draw_point(PointS::new(self.x, self.y));
    }
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
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
    pub fn e_draw_pixel(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
        x: i32,
        y: i32,
    ) -> Result<Point, Box<dyn std::error::Error>> {
        let mut point = Point::new(x, y, vec![r, g, b]);
        point.draw(self);
        Ok(point)
    }
    pub fn e_set_draw_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        Ok(())
    }

    pub fn e_clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.clear();
        Ok(())
    }

    /// Register a callback for a key by string name (e.g., "ESCAPE", "SPACE")
    pub fn e_add_key_event<F>(&mut self, key: &str, key_handler: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut() + 'static,
    {
        let keycode = match key.to_uppercase().as_str() {
            "ESCAPE" => Keycode::Escape,
            "SPACE" => Keycode::Space,
            "RETURN" => Keycode::Return,
            "A" => Keycode::A,
            "D" => Keycode::D,
            _ => return Err(format!("Invalid key code: {:?}", key).into()),
        };

        self.events.insert(keycode, Box::new(key_handler));
        Ok(())
    }

    /// Runs the main loop.
    /// User provides a render closure called every frame.
    /// The loop dispatches events to registered key handlers.
    pub fn run<R>(&mut self, mut render: R) -> Result<(), Box<dyn std::error::Error>>
    where
        R: FnMut(&mut Canvas) -> Result<(), Box<dyn std::error::Error>>,
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
            self.canvas.present();

            // Limit to ~60 FPS
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
