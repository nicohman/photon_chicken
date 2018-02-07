

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
pub use arena::Arena;
pub use arena_controller::ArenaController;
pub use arena_view::ArenaView;
mod arena;
mod arena_controller;
mod arena_view;
fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Photon Chicken", (200, 200)).opengl(opengl).exit_on_esc(true);
    let mut window : GlutinWindow = settings.build().expect("Couldn't create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/font.ttf", (), texture_settings).expect("Couldn't load font");
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear([1.0;4], g);

            })
        }
    }
}
