extern crate sdl2;

use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point as PointS, Rect};
use sdl2::render::TextureQuery;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::collections::HashMap;
use std::error::Error;
use std::result::Result;
use std::time::Duration;

pub struct Canvas {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    _sdl_context: sdl2::Sdl,
    _event_pump: sdl2::EventPump,
    _ttf_context: sdl2::ttf::Sdl2TtfContext,
    _image_context: sdl2::image::Sdl2ImageContext,
    events: HashMap<Keycode, Box<dyn FnMut()>>,
    texture_cache: HashMap<String, sdl2::render::Texture<'static>>,
}

/// Image handle like Circle/Square.
#[derive(Clone)]
pub struct Image {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub path: Option<String>,
}

impl Image {
    /// Create an Image handle without loading the texture.
    pub fn new(id: &str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Image {
            id: id.to_string(),
            x,
            y,
            w,
            h,
            path: None,
        }
    }

    /// Create an Image handle and remember a path. Does not automatically load; call `canvas.load_image` or let `e_draw_image` auto-load.
    pub fn new_with_path(id: &str, path: &str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Image {
            id: id.to_string(),
            x,
            y,
            w,
            h,
            path: Some(path.to_string()),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), Box<dyn Error>> {
        canvas.e_draw_image(self)
    }
}

#[derive(Clone)]
pub struct Font {
    x: i32,
    y: i32,
    size: u16,
    path: String,
    text: String,
    color: Color,
}

impl Font {
    pub fn new(x: i32, y: i32, size: u16, path: &str, text: &str, color: Vec<u8>) -> Self {
        Font {
            x,
            y,
            size,
            path: path.to_string(),
            text: text.to_string(),
            color: Color::RGB(color[0], color[1], color[2]),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), Box<dyn Error>> {
        let font = canvas
            ._ttf_context
            .load_font(&self.path, self.size)
            .map_err(|e| e.to_string())?;

        let surface = font
            .render(&self.text)
            .blended(self.color)
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(self.x, self.y, width, height);
        canvas.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
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
            x,
            y,
            r,
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
                let _ = canvas.canvas.draw_line(
                    PointS::new(start_x, center_y + dy),
                    PointS::new(end_x, center_y + dy),
                );
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
    /// Load an image into the canvas texture cache. `id` is the key you will use to draw it later.
    /// Note: this uses an unsafe transmute to store textures as `'static` in the cache. It's a
    /// pragmatic approach commonly used with SDL2 when the TextureCreator is tied to a long-lived
    /// canvas. Keep the Canvas alive for the lifetime of textures.
    pub fn load_image(&mut self, id: &str, path: &str) -> Result<(), Box<dyn Error>> {
        if self.texture_cache.contains_key(id) {
            return Ok(());
        }

        // SAFETY: we transmute a reference to the texture_creator to a `'static` reference so we
        // can create and store `Texture<'static>` in the cache. This is unsafe because it tells
        // the compiler the creator outlives the program; in practice keep this Canvas alive for as
        // long as you use the cached textures.
        let texture_creator: &'static _ = unsafe {
            std::mem::transmute::<
                &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
                &'static sdl2::render::TextureCreator<sdl2::video::WindowContext>,
            >(&self.canvas.texture_creator())
        };

        let texture = texture_creator.load_texture(path)?;
        self.texture_cache.insert(id.to_string(), texture);
        Ok(())
    }

    /// Draw a previously loaded image by `id`. If the id is not present nothing happens.
    pub fn e_draw_image(&mut self, image: &Image) -> Result<(), Box<dyn Error>> {
        // If texture is already cached, draw it.
        if let Some(texture) = self.texture_cache.get(&image.id) {
            let dest = Rect::new(image.x, image.y, image.w, image.h);
            self.canvas.copy(texture, None, Some(dest))?;
            return Ok(());
        }

        // If Image knows its path, attempt to load it on first use.
        if let Some(path) = &image.path {
            self.load_image(&image.id, path)?;
            if let Some(texture) = self.texture_cache.get(&image.id) {
                let dest = Rect::new(image.x, image.y, image.w, image.h);
                self.canvas.copy(texture, None, Some(dest))?;
            }
        }

        Ok(())
    }

    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).map_err(|e| e.to_string())?;
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
            _ttf_context: ttf_context,
            _image_context: image_context,
            events: HashMap::new(),
            texture_cache: HashMap::new(),
        })
    }

    pub fn e_draw_font(&mut self, font: &Font) -> Result<(), Box<dyn Error>> {
        font.draw(self)?;
        Ok(())
    }
    pub fn e_background(&mut self, colors: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.canvas
            .set_draw_color(Color::RGB(colors[0], colors[1], colors[2]));
        self.canvas.clear();
        Ok(())
    }

    pub fn e_draw_pixel(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        let _ = point.draw(self);
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
                    Event::KeyDown { keycode: Some(key), .. } => {
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
