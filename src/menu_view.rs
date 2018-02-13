use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use graphics;
use opengl_graphics;
use std::f64::consts;
use rand::os::OsRng;
use opengl_graphics::{Texture, TextureSettings, Filter};
use std::path::Path;
use rand::Rng;
use color_gen::get_color;
pub struct MenuViewSet {
    size_x:f64,
    size_y:f64,
    board_edge_radius:f64,
    tile_size:f64,
    edge_color_board:Color,
    edge_color_tile: Color,
    tile_edge_radius: 1.0,
}
impl MenuViewSet {
    pub fn new() -> MenuViewSet {
        MenuViewSet {
            size_x:400.0,
            size_y:400.0,
            board_edge_radius:3.0,
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            tile_edge_radius: 1.0,
        }
    }
}
pub struct MenuView {
    settings:MenuViewSet
}
impl MenuView {
    pub fn new(settings:MenuViewSet) -> MenuView {
        MenuView {
            settings:settings
        }
    }
    pub fn draw<G: Graphics, C>(&self, controller:&mut ArenaController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed, Text};
        let ref settings = self.settings;
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        while i < (settings.size_y / settings.tile_size).round(){
            c_line.draw([0.0,i*settings.tile_size,settings.size_x,i*settings.tile_size],&c.draw_state,c.transform,g);
            i+=1.0;
        }
        let edLine = Line::new(settings.edge_color_board, settings.board_edge_radius);
        edLine.draw([0.0,0.0,settings.size_x,0.0],&c.draw_state, c.transform, g);
        edLine.draw([0.0,0.0,0.0,settings.size_y], &c.draw_state, c.transform,g);
        edLine.draw([0.0,settings.size_y,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x,settings.size_y,settings.size_x,0.0], &c.draw_state, c.transform, g);
        Text::new_color(settings.edge_color_board, 400).draw("PHOTONCHICKEN", &c.draw_state,c.transform,g);
        let mut i =1;
        for mode in controller.modes {
            Text::new_color(settings.edge_color_board, 40).draw(mode.name, &c.draw_state, c.transform.trans(0, settings.tile_size*4*i, g);
                i+=1;
        }

                        }
                        }
