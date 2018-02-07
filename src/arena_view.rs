use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use ArenaController;
pub struct ArenaViewSet {
    pub position: [f64; 2],
    pub size: f64,
    pub bg_color: Color,
    pub border_color: Color,
    pub edge_color_board: Color,
    pub edge_color_tile: Color,
    pub board_edge_radius: f64,
    pub tile_edge_radius: f64,
    pub text_color: Color,
    pub lightcycle_color: Color,
}

impl ArenaViewSet {
    pub fn new() -> ArenaViewSet {
        ArenaViewSet {
            position: [10.0; 2],
            size: 400.0,
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
    pub settings: ArenaViewSet
}
impl ArenaView {
    pub fn new(settings:ArenaViewSet) -> ArenaView {
        ArenaView {
            settings:settings
        }
    }
    pub fn draw<G: Graphics, C>(&self, controller:&ArenaController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed};
        let ref settings = self.settings;
        let arena_rect = [settings.position[0], settings.position[1], settings.size, settings.size];
        Rectangle::new(settings.bg_color).draw(arena_rect, &c.draw_state, c.transform, g);
    }
}
