use graphics;
use opengl_graphics;
use std::f64::consts;
use rand::os::OsRng;
use opengl_graphics::{Texture, TextureSettings, Filter};
use std::path::Path;
use rand::Rng;
use color_gen::get_color;
use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use BricksController;
pub struct BricksViewSet {
    pub position: [f64; 2],
    pub size: f64,
    pub size_x: f64,
    pub size_y : f64,
    pub tile_size: f64,
    pub bg_color: Color,
    pub border_color: Color,
    pub edge_color_board: Color,
    pub edge_color_tile: Color,
    pub board_edge_radius: f64,
    pub tile_edge_radius: f64,
    pub texture_settings: TextureSettings,
    pub text_color: Color,
}
impl BricksViewSet {
    pub fn new() -> BricksViewSet {
        BricksViewSet {
            position: [0.0; 2],
            size: 400.0,
            size_x:400.0,
            size_y:400.0,
            tile_size: 15.0,
            texture_settings: TextureSettings::new().filter(Filter::Nearest),
            bg_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            board_edge_radius: 3.0,
            tile_edge_radius: 1.0,
            text_color: [0.0, 0.0, 0.2, 1.0],
        }
    }
}
pub struct BricksView {
    pub settings: BricksViewSet,
    pub  textures: Vec<Texture>
}
impl BricksView {
    pub fn new (settings: BricksViewSet) -> BricksView {
        BricksView {
            settings:settings,
            textures:vec![],
        }
    }
    pub fn draw<G: Graphics, C>(&self, controller:&mut BricksController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed, Text};
        let ref settings = self.settings;

    }    
}
