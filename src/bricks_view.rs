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
    pub pickup_color: Color,
    pub border_color: Color,
    pub edge_color_board: Color,
    pub edge_color_tile: Color,
    pub board_edge_radius: f64,
    pub brick_color: Color,
    pub shot_color : Color,
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
            pickup_color: [0.2,0.3,0.3,1.0],
            texture_settings: TextureSettings::new().filter(Filter::Nearest),
            bg_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            edge_color_board: [0.0, 0.0, 0.6, 1.0],
            edge_color_tile: [0.0, 0.0, 0.4, 1.0],
            shot_color: [1.0, 0.0, 0.4, 1.0],
            brick_color : [0.0,0.7,0.6,0.2],
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
        use graphics::{Image, Line, Rectangle, Transformed, Text, ellipse};
        use std::f64::consts::PI;

        let ref settings = self.settings;
        let bricks_rect = [settings.position[0], settings.position[1], settings.size_x, settings.size_y];
        Rectangle::new(settings.border_color).draw(bricks_rect, &c.draw_state, c.transform, g);
        let mut i = 0.0;
        let mut gen = OsRng::new().unwrap();
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        let mut rand_col = settings.border_color;
        while i < (settings.size_x / settings.tile_size).round()  {
            c_line.draw([i*settings.tile_size,0.0,i*settings.tile_size,settings.size_y],&c.draw_state,c.transform,g);
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
        edLine.draw([0.0, settings.size_y / 3.0, settings.size_x / 2.5, settings.size_y / 3.0], &c.draw_state, c.transform, g);
        edLine.draw([0.0, settings.size_y / 3.0 * 2.0, settings.size_x / 2.5, settings.size_y / 3.0 * 2.0], &c.draw_state, c.transform, g);
        edLine.draw([ settings.size_x - (settings.size_x / 2.5) , settings.size_y / 3.0,settings.size_x, settings.size_y / 3.0], &c.draw_state, c.transform, g);
        edLine.draw([ settings.size_x - (settings.size_x / 2.5), settings.size_y / 3.0 * 2.0, settings.size_x, settings.size_y / 3.0 * 2.0], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x / 2.5, 0.0, settings.size_x / 2.5, settings.size_y / 3.0], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x - (settings.size_x / 2.5), 0.0, settings.size_x - (settings.size_x / 2.5), settings.size_y / 3.0], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x / 2.5, settings.size_y, settings.size_x / 2.5, settings.size_y / 3.0 * 2.0], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x - (settings.size_x / 2.5), settings.size_y, settings.size_x - (settings.size_x / 2.5), settings.size_y / 3.0 * 2.0], &c.draw_state, c.transform, g);
        let ref bricks = controller.bricks;
        for wall in bricks.walls.iter() {
            for brick in wall.bricks.iter() {
                if !brick.dead {
                    let bx =brick.position[0] * 12.0 - 10.0;
                    let by = brick.position[1] * 12.0 - 10.0;
                    Rectangle::new(settings.brick_color).draw([0.0,0.0,10.0,10.0], &c.draw_state, c.transform.trans(wall.position[0],wall.position[1]).rot_rad(PI * wall.dir).trans(bx,by), g);
                }
            }

        }
        for u  in &controller.bricks.users {
            if !u.dead{
                Rectangle::new(get_color(u.id)).draw([0.0,0.0,30.0,30.0],&c.draw_state,c.transform.trans(u.position[0],u.position[1]),g);
            }
        }
        for pickup in &controller.bricks.pickups {
            Rectangle::new(settings.pickup_color).draw([0.0,0.0,20.0,20.0], &c.draw_state, c.transform.trans(pickup.position[0], pickup.position[1]), g);
        }
        for s in &controller.bricks.shots {
            ellipse::Ellipse::new(settings.shot_color).draw([0.0,0.0,15.0,15.0], &c.draw_state, c.transform.trans(s.position[0],s.position[1]),g);
        }
        if controller.bricks.paused {
            if controller.bricks.start_tick == -1.0 {
                Text::new_color(settings.edge_color_board, 200).draw("PAUSED", glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);
            } else {
                Text::new_color(settings.edge_color_board, 200).draw(&controller.bricks.start_tick.ceil().to_string(), glyphs, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0 + 100.0), g);

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
