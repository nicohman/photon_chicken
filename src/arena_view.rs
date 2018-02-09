use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use graphics;
use opengl_graphics;
use rand::os::OsRng;
use opengl_graphics::{Texture, TextureSettings, Filter};
use std::path::Path;
use rand::Rng;
use ArenaController;
pub struct ArenaViewSet {
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
    pub lightcycle_color: Color,
}

impl ArenaViewSet {
    pub fn new() -> ArenaViewSet {
        ArenaViewSet {
            position: [0.0; 2],
            size: 400.0,
            size_x:400.0,
            size_y:400.0,
            tile_size: 15.0,
            texture_settings: TextureSettings::new().filter(Filter::Nearest),
            bg_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            edge_color_board: [0.0, 0.0, 0.2, 1.0],
            edge_color_tile: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            tile_edge_radius: 1.0,
            text_color: [0.0, 0.0, 0.2, 1.0],
            lightcycle_color: [0.0, 0.0, 0.2, 1.0],
        }
    }
}
pub struct ArenaView {
    pub settings: ArenaViewSet,
        pub textures: Vec<Texture>
}
impl ArenaView {
    pub fn new(settings:ArenaViewSet) -> ArenaView {
        ArenaView {
            textures: vec![Texture::from_path(Path::new("assets/cycle.png"),&settings.texture_settings).unwrap()],
            settings:settings,
        }
    }
    pub fn draw<G: Graphics, C>(&self, controller:&ArenaController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed};
        let ref settings = self.settings;
        let arena_rect = [settings.position[0], settings.position[1], settings.size_x, settings.size_y];
        Rectangle::new(settings.bg_color).draw(arena_rect, &c.draw_state, c.transform, g);
        let mut i = 0.0;
        let mut gen = OsRng::new().unwrap();

        while i < ((settings.size_x / settings.tile_size).round() * (settings.size_y / settings.tile_size).round()) {
            let mut rand_col = settings.border_color;
            rand_col = [rand_col[0] + gen.next_f32(),rand_col[1] + gen.next_f32(),
rand_col[2] + gen.next_f32(),
rand_col[3] + gen.next_f32()];
            Rectangle::new(rand_col).draw([(i % (settings.size_x / settings.tile_size).round()) * settings.tile_size, (i / (settings.size_x / settings.tile_size).round()).floor() * settings.tile_size, settings.tile_size, settings.tile_size], &c.draw_state, c.transform, g);

            i += 1.0;
        }
        for cy in &controller.arena.cycles {
            Image::new().rect([cy.position[0], cy.position[1], 15.0, 30.0]).draw(&self.textures[0], &c.draw_state, c.transform, g);
        }
    }
}
