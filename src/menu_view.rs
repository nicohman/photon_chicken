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
        /*for mode in &controller.menu.modes {
            if !(&mode.name == "menu"){
            let x = settings.size_x / 2.0 - (settings.choice_size *( mode.name.chars().count()as f64) / 2.0);
            let y = settings.tile_size * 6.0 * i + (settings.size_y /2.5);
            Text::new_color(settings.edge_color_board, settings.choice_size as u32).draw(&mode.name, glyphs,&c.draw_state, c.transform.trans(x,y,), g);
            let y1 = y + settings.size_y / 50.0;
            let x2 = x+(settings.choice_size*(mode.name.chars().count() as f64));
            let y2 = y-settings.choice_size ;
            chLine.draw([x,y1,x2, y1], &c.draw_state, c.transform, g);
            chLine.draw([x,y1,x,y2],&c.draw_state,c.transform,g);
            chLine.draw([x2,y1,x2,y2], &c.draw_state, c.transform, g);
            chLine.draw([x,y2,x2,y2], &c.draw_state,c.transform,g);
            i+=1.0;
        }*/
        chLine.draw([0.0,0.0,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        chLine.draw([0.0,settings.size_y,settings.size_x,0.0],&c.draw_state,c.transform,g);
        //chLine.draw([settings.size_x/2.0,0.0,settings.size_x/2.0,settings.size_y],&c.draw_state,c.transform,g);
        CircleArc::new(settings.edge_color_choice, settings.circle_radius, 0.0, PI*1.9999).draw([(settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0+settings.size_x/4.0),(settings.size_x/16.0+settings.size_x/4.0)],&c.draw_state,c.transform.trans(settings.size_x/2.0 - ((settings.size_x/16.0 - settings.size_x/8.0)/2.0),settings.size_y/2.0 - ((settings.size_x/15.0-settings.size_x/8.0)/2.0)),g);
        //}
    }
}
