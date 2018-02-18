use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use graphics;
use opengl_graphics;
use std::f64::consts;
use rand::os::OsRng;
use color_gen::get_color;
use MenuController;
pub struct MenuViewSet {
    pub size_x:f64,
    pub size_y:f64,
    pub board_edge_radius:f64,
    pub tile_size:f64,
    pub size: f64,
    pub circle_radius:f64,
    pub position: [f64;2],
    pub edge_color_board:Color,
    pub edge_color_tile: Color,
    pub tile_edge_radius: f64,
    pub choice_size: f64,
    pub edge_color_choice: Color,
    pub edge_choice_radius: f64,
    pub cycle_color:Color,
    pub not_color:Color,
    pub close_color:Color,
    pub tower_color:Color,
}
impl MenuViewSet {
    pub fn new() -> MenuViewSet {
        MenuViewSet {
            size_x:400.0,
            size: 400.0,
            position: [0.0;2],
            size_y:400.0,
            tile_size:15.0,
            board_edge_radius:3.0,
            circle_radius:4.0,
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            tile_edge_radius: 1.0,
            edge_choice_radius: 2.0,
            edge_color_choice: [0.0, 0.0, 0.8, 1.0],
            choice_size:40.0,
            cycle_color:[0.0, 1.0, 1.8, 1.0],
            close_color: [0.0, 0.0, 0.8, 1.0],
            not_color:[0.0, 0.0, 0.6, 1.0],
            tower_color: [0.0, 0.6, 0.2, 1.0]
        }
    }
}
pub struct MenuView {
    pub settings:MenuViewSet
}
impl MenuView {
    pub fn new(settings:MenuViewSet) -> MenuView {
        MenuView {
            settings:settings
        }
    }
    pub fn draw<G: Graphics, C>(&self, controller:&mut MenuController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed, CircleArc,Text};
        use std::f64::consts::PI;
        let ref settings = self.settings;
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        let mut t = 0.0;
        while t < (settings.size_y / settings.tile_size).round(){
            c_line.draw([0.0,t*settings.tile_size,settings.size_x,t*settings.tile_size],&c.draw_state,c.transform,g);
            t+=1.0;
        }
        let edLine = Line::new(settings.edge_color_board, settings.board_edge_radius);
        edLine.draw([0.0,0.0,settings.size_x,0.0],&c.draw_state, c.transform, g);
        edLine.draw([0.0,0.0,0.0,settings.size_y], &c.draw_state, c.transform,g);
        edLine.draw([0.0,settings.size_y,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x,settings.size_y,settings.size_x,0.0], &c.draw_state, c.transform, g);
        Text::new_color(settings.edge_color_board, 400).draw("PHOTONCHICKEN", glyphs,&c.draw_state,c.transform,g);
        let mut i =1.0;
        let chLine = Line::new(settings.edge_color_choice, settings.edge_choice_radius);
        chLine.draw([0.0,0.0,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        chLine.draw([0.0,settings.size_y,settings.size_x,0.0],&c.draw_state,c.transform,g);
        CircleArc::new(settings.edge_color_choice, settings.circle_radius, 0.0, PI*1.9999).draw([(settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0+settings.size_x/4.0),(settings.size_x/16.0+settings.size_x/4.0)],&c.draw_state,c.transform.trans(settings.size_x/2.0 - ((settings.size_x/16.0 - settings.size_x/8.0)/2.0),settings.size_y/2.0 - ((settings.size_x/15.0-settings.size_x/8.0)/2.0)),g);
        let mut i = 0.0;
        let mLine = Line::new(settings.cycle_color, settings.tile_edge_radius);
        i = 0.0;
        let pRect = [0.0,0.0,1.0,1.0];
        while i < settings.size_x {
            t = 0.0;
            while t < settings.size_y {
                let mut x0 = (i / settings.size_x * 3.5) - 2.5;
                let mut y0  = (t / settings.size_y * 2.0) - 1.0;
                let mut x= 0.0;
                let mut y = 0.0;
                let mut iteration = 0;
                let mut max_iteration = 1000;
                while x*x + y*y < 2.0*2.0 && iteration < max_iteration {
                    let xtemp = x*x - y*y + x0;
                    y = 2.0*x*y + y0;
                    x = xtemp;
                    iteration+=1;
                }
                let mut color = &settings.close_color;
                if iteration < 500 {
                    color = &settings.not_color;
                }
                Rectangle::new(*color).draw(pRect, &c.draw_state, c.transform.trans(i,t), g);
                t+= 1.0;
            }
            i += 1.0;
            t = 0.0;

        }
    }
}
