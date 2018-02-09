

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::{RenderEvent, PressEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
pub use arena::Arena;
pub use arena_controller::ArenaController;
pub use arena_view::{ArenaView, ArenaViewSet};
mod arena;
mod arena_controller;
mod arena_view;
fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("PC", [512;2]).opengl(opengl).exit_on_esc(false);
    let mut window : GlutinWindow = settings.build().expect("Couldn't create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);
    let mut arena = Arena::new();
    let mut arena_controller = ArenaController::new(arena);
    let mut arena_view_settings = ArenaViewSet::new();
    let mut arena_view = ArenaView::new(arena_view_settings);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/font.ttf", (), texture_settings).expect("Couldn't load font");
    arena_controller.arena.create_cycle([20.0, 20.0], arena_view.settings.lightcycle_color);
    while let Some(e) = events.next(&mut window) {

        arena_controller.update();  
        arena_controller.event(arena_view.settings.position, arena_view.settings.size, &e);
        if let Some(args) = e.render_args() {
            arena_view.settings.size_x = args.draw_width as f64;
            arena_view.settings.size_y = args.draw_height as f64; 
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                //clear([1.0;4], g);
                arena_view.draw(&arena_controller, glyphs, &c, g);
            })


        }

    }
}
