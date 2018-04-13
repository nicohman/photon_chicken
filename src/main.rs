#![allow(warnings)]

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
pub use tower::Tower;
pub use tower_controller::TowerController;
pub use tower_view::{TowerView, TowerViewSet};
pub use arena_controller::ArenaController;
pub use arena_view::{ArenaView, ArenaViewSet};
use menu::Menu;
use menu_controller::MenuController;
use menu_view::{MenuView, MenuViewSet};
use bricks::Bricks;
use bricks_controller::BricksController;
use bricks_view::{BricksView, BricksViewSet};
mod arena;
mod color_gen;
mod arena_controller;
mod menu;
mod tower;
mod tower_view;
mod tower_controller;
mod menu_view;
mod menu_controller;
mod arena_view;
mod bricks;
mod bricks_controller;
mod bricks_view;
fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("PC", [512;2]).opengl(opengl).fullscreen(true).exit_on_esc(false).srgb(false);
    let mut window : GlutinWindow = settings.build().expect("Couldn't create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);
    let mut arena = Arena::new();
    let mut menu = Menu::new();
    let mut menu_controller = MenuController::new(menu);
    let mut menu_view_settings = MenuViewSet::new();
    let mut menu_view = MenuView::new(menu_view_settings);
    let mut arena_controller = ArenaController::new(arena);
    let mut arena_view_settings = ArenaViewSet::new();
    let mut arena_view = ArenaView::new(arena_view_settings);
    let mut tower = Tower::new();
    let mut tower_controller = TowerController::new(tower);
    let mut tower_view_settings = TowerViewSet::new();
    let mut tower_view = TowerView::new(tower_view_settings);
    let mut bricks = Bricks::new();
    let mut bricks_controller = BricksController::new(bricks);
    let mut bricks_view_settings = BricksViewSet::new();
    let mut bricks_view = BricksView::new(bricks_view_settings);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/font.ttf", (), texture_settings).expect("Couldn't load font");
    arena_controller.arena.reset_game();
    tower_controller.tower.reset([tower_view.settings.size_x,tower_view.settings.size_y]);
    bricks_controller.bricks.reset([bricks_view.settings.size_x, bricks_view.settings.size_y]);
    while let Some(e) = events.next(&mut window) {
        match menu_controller.menu.selected.name.as_ref() {
            "menu" => {
                menu_controller.update();
                menu_controller.event(menu_view.settings.position, menu_view.settings.size,&e);
            },
            "cycles" => {
                arena_controller.update((arena_view.settings.size_x, arena_view.settings.size_y));
                arena_controller.event(arena_view.settings.position, arena_view.settings.size, &mut menu_controller, &e);

            },
            "bricks" => {
                bricks_controller.update((bricks_view.settings.size_x, bricks_view.settings.size_y));
                bricks_controller.event(bricks_view.settings.position, bricks_view.settings.size, &mut menu_controller, &e);
            }
            "tower" => {
                tower_controller.update((tower_view.settings.size_x, tower_view.settings.size_y));
                tower_controller.event(tower_view.settings.position, tower_view.settings.size, &mut menu_controller, &e);

            },
            _ => {
                menu_controller.update();
                menu_controller.event(menu_view.settings.position, menu_view.settings.size,&e);
            }
        }
        if let Some(args) = e.render_args() {
            arena_view.settings.size_x = args.draw_width as f64;
            arena_view.settings.size_y = args.draw_height as f64;
            menu_view.settings.size_x = args.draw_width as f64;
            menu_view.settings.size_y = args.draw_height as f64;
            tower_view.settings.size_x = args.draw_width as f64;
            tower_view.settings.size_y = args.draw_height as f64;
            bricks_view.settings.size_x = args.draw_width as f64;
            bricks_view.settings.size_y = args.draw_height as f64;
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear(arena_view.settings.border_color, g);
                match menu_controller.menu.selected.name.as_ref() {
                    "menu" => {
                        menu_view.draw(&mut menu_controller, glyphs, &c, g);
                    },
                    "cycles" => {

                        arena_view.draw(&mut arena_controller, glyphs, &c, g);
                    },
                    "tower" => {
                        tower_view.draw(&mut tower_controller, glyphs, &c, g);

                    },
                    "bricks" => {
                        bricks_view.draw(&mut bricks_controller, glyphs, &c, g);
                    },
                    _ => {
                        menu_view.draw(&mut menu_controller, glyphs, &c, g);
                    }
                }
            })


        }

    }
}
