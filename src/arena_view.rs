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
    pub trail_color: Color,
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
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            board_edge_radius: 3.0,
            tile_edge_radius: 1.0,
            text_color: [0.0, 0.0, 0.2, 1.0],
            trail_color: [0.0, 0.6, 0.2, 1.0],
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
    pub fn draw<G: Graphics, C>(&self, controller:&mut ArenaController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed, Text};
        let ref settings = self.settings;
        let arena_rect = [settings.position[0], settings.position[1], settings.size_x, settings.size_y];
        Rectangle::new(settings.border_color).draw(arena_rect, &c.draw_state, c.transform, g);
        let mut i = 0.0;
        let mut gen = OsRng::new().unwrap();
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        let mut rand_col = settings.border_color;
        while i < (settings.size_x / settings.tile_size).round()  {
            c_line.draw([i*settings.tile_size,0.0,i*settings.tile_size,settings.size_y],&c.draw_state,c.transform,g);
            /*rand_col = [rand_col[0] + gen.next_f32(),rand_col[1] + gen.next_f32(),
              rand_col[2] + gen.next_f32(),
              rand_col[3] + gen.next_f32()];*/
           // let x = (i % (settings.size_x / settings.tile_size).round()) * settings.tile_size;
            //let y = (i / (settings.size_x / settings.tile_size).round()).floor() * settings.tile_size;
            // Rectangle::new(rand_col).draw([0.0,0.0, settings.tile_size, settings.tile_size], &c.draw_state, c.transform.trans(x,y), g);
            //println!("Drawing{} at {}, {}", i,x,y);
            i += 1.0;
        }
        i = 0.0;
        while i < (settings.size_y / settings.tile_size).round(){
            c_line.draw([0.0,i*settings.tile_size,settings.size_x,i*settings.tile_size],&c.draw_state,c.transform,g);
        i+=1.0;
        } 
    let edLine = Line::new(settings.edge_color_board, settings.board_edge_radius);
        edLine.draw([0.0,0.0,settings.size_x,0.0],&c.draw_state, c.transform, g);
        edLine.draw([0.0,0.0,0.0,settings.size_y], &c.draw_state, c.transform,g);
        edLine.draw([0.0,settings.size_y,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x,settings.size_y,settings.size_x,0.0], &c.draw_state, c.transform, g);
        let mut y = 0;
        for (key, value) in &controller.arena.trails {
            if value.owner > 0 {
                Rectangle::new(get_color(value.owner)).draw([value.position[0] * 15.0, value.position[1] * 15.0, settings.tile_size, settings.tile_size], &c.draw_state, c.transform, g);
            }

        }
        for cy in &controller.arena.cycles {
            if cy.dead != true {
                let deg = match cy.dir {
                    3.0 => consts::PI /2.0,
                    2.0 => consts::PI / 1.0,
                    1.0 => consts:: PI * 1.5 ,
                    0.0 => consts::PI / 0.5,
                    _ => 0.0
                };
                let transf = c.transform.trans(cy.position[0], cy.position[1]).rot_rad(deg).trans(-7.5, -15.0);
                Rectangle::new(get_color(cy.owner)).draw([0.0,0.0, 15.0, 30.0], &c.draw_state, transf, g);
                Image::new().rect([0.0, 0.0, 15.0, 30.0]).draw(&self.textures[0], &c.draw_state,transf , g);
                c.reset();
            }
        }

        if &controller.deaths.len() > &0 {
            let toDisp = &controller.deaths.pop().unwrap();
            Text::new_color(get_color(*toDisp), 50).draw("DEAD", glyphs, &c.draw_state, c.transform.trans(/*controller.arena.cycles[(*toDisp -1) as usize].position[0], controller.arena.cycles[(*toDisp-1) as usize].position[1]*/50.0, 50.0), g);
        }
        let mut h = 0;
        while h < controller.score.len() {
            if controller.score[h as usize] > 0 {
                
                Text::new_color(get_color((h+1) as i32), 50).draw(&(controller.score[h as usize].to_string()), glyphs, &c.draw_state, c.transform.trans(settings.size_x - ((4-h) as f64 *120.0), 50.0), g);
            }
        h+= 1;
        }
        if controller.arena.paused {
            if controller.arena.start_tick == -1.0 {
            Text::new_color(settings.edge_color_board, 200).draw("PAUSED", glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);
            } else {
            Text::new_color(settings.edge_color_board, 200).draw(&controller.arena.start_tick.ceil().to_string(), glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);
            
            }
        }
    }
}
