use TowerController;
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

pub struct TowerViewSet {
    pub position: [f64;2],
    pub size_x: f64,
    pub size_y: f64,
    pub size: f64,
    pub user_color:Color,
    pub tile_size: f64,
    pub bg_color: Color,
    pub border_color: Color,
    pub edge_color_board: Color,
    pub edge_color_tile: Color,
    pub board_edge_radius: f64,
    pub tile_edge_radius: f64,
    pub texture_settings: TextureSettings,
    pub text_color: Color,
    pub shot_radius:f64,
    pub shot_color:Color,
}
impl TowerViewSet {
    pub fn new() -> TowerViewSet {
        TowerViewSet {
            position: [0.0; 2],
            size: 400.0,
            size_x:400.0,
            size_y:400.0,
            tile_size: 15.0,
            texture_settings: TextureSettings::new().filter(Filter::Nearest),
            bg_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            user_color:[1.0, 0.0, 0.6, 1.0],
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            board_edge_radius: 3.0,
            shot_color: [1.0, 0.0, 0.4, 1.0],
            shot_radius: 1.0,
            tile_edge_radius: 1.0,
            text_color: [0.0, 0.0, 0.2, 1.0],
        }
    }
}
pub struct TowerView {
    pub settings:TowerViewSet,
    pub done_first:bool,
    pub textures:Vec<Texture>
}
impl TowerView {
    pub fn new(settings:TowerViewSet) -> TowerView {
        TowerView {
            textures:vec![Texture::from_path(Path::new("assets/tower.png"), &settings.texture_settings).unwrap(), Texture::from_path(Path::new("assets/spieder.png"), &settings.texture_settings).unwrap()],
            settings:settings,
            done_first:false,
        }
    }
    pub fn draw<G: Graphics, C>(&mut self, controller:&mut TowerController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use std::f64::consts::PI;
        use graphics::{Image, Line, Rectangle, Transformed, Text, CircleArc, ellipse};

        let ref settings = self.settings;
        if !self.done_first {
            controller.tower.reset([settings.size_x,settings.size_y]);
            self.done_first = true;
        }
        let mut i = 0.0;
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        while i < (settings.size_x / settings.tile_size).round()  {
            c_line.draw([i*settings.tile_size,0.0,i*settings.tile_size,settings.size_y],&c.draw_state,c.transform,g);
            i += 1.0;
        }
        let edLine = Line::new(settings.edge_color_board, settings.board_edge_radius);
        i = 0.0;
        while i < (settings.size_y / settings.tile_size).round(){
            c_line.draw([0.0,i*settings.tile_size,settings.size_x,i*settings.tile_size],&c.draw_state,c.transform,g);
            i+=1.0;
        }

        edLine.draw([0.0,0.0,settings.size_x,0.0],&c.draw_state, c.transform, g);
        edLine.draw([0.0,0.0,0.0,settings.size_y], &c.draw_state, c.transform,g);
        edLine.draw([0.0,settings.size_y,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x,settings.size_y,settings.size_x,0.0], &c.draw_state, c.transform, g);
        Image::new().rect([0.0,0.0,60.0,90.0]).draw(&self.textures[0],&c.draw_state,c.transform.trans(settings.size_x/2.0 - 30.0,settings.size_y/2.0 - 45.0),g);
        let sp_rect = [0.0,0.0,30.0,30.0];
        let mut gen = OsRng::new().unwrap();
        for sp in &controller.tower.spiders {
            let mut rand_x = 0.75 * gen.next_f64();
            let mut rand_y = 0.75 * gen.next_f64();
            if rand_x % 0.2 != 0.0 {
                rand_x *= -1.0;
            }
            if rand_y % 0.2 != 0.0 {
                rand_y *= -1.0;
            }
            Image::new().rect(sp_rect).draw(&self.textures[1],&c.draw_state,c.transform.trans(sp.position[0] + rand_x ,sp.position[1] + rand_y),g);
        }
        for u  in &controller.tower.users {
            if !u.dead{
            Rectangle::new(get_color(u.id)).draw([0.0,0.0,30.0,60.0],&c.draw_state,c.transform.trans(u.position[0],u.position[1]),g);
            }
        }
        for s in &controller.tower.shots {
            ellipse::Ellipse::new(settings.shot_color).draw([0.0,0.0,15.0,15.0], &c.draw_state, c.transform.trans(s.position[0],s.position[1]),g);
        }
        if controller.tower.paused {
            if controller.tower.start_tick == -1.0 {
                Text::new_color(settings.edge_color_board, 200).draw("PAUSED", glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);
            } else {
                Text::new_color(settings.edge_color_board, 200).draw(&controller.tower.start_tick.ceil().to_string(), glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);

            }
        }
        let mut h = 0;
        while h < controller.score.len() {
            if controller.score[h as usize] > 0 {

                Text::new_color(get_color((h) as i32), 50).draw(&(controller.score[h as usize].to_string()), glyphs, &c.draw_state, c.transform.trans(settings.size_x - ((4-h) as f64 *120.0), 50.0), g);
            }
            h+= 1;

        }
    }
}
