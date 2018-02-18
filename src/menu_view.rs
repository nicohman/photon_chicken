use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};
use graphics;
use opengl_graphics;
use std::f64::consts;
use rand::os::OsRng;
use color_gen::get_color;
use MenuController;
pub fn gen_set(range: [f64;4]) -> MandSet {
    let mut i = range[0];
    let mut set :Vec<Vec<i32>>= Vec::new();
    let mut t = 0.0;
    while i < range[2]{
        t = range[1];
        while t < range[3]{
            let mut x0 = (i / range[2] * 3.5) - 2.5;
            let mut y0  = (t / range[3] * 2.0) - 1.0;
            let mut x= 0.0;
            let mut y = 0.0;
            let mut iteration = 0;
            let mut max_iteration = 150;
            while x*x + y*y < 2.0*2.0 && iteration < max_iteration {
                let xtemp = x*x - y*y + x0;
                y = 2.0*x*y + y0;
                x = xtemp;
                iteration+=1;
            }
            if set.get(i as usize).is_some() {
                set[i as usize].insert(t as usize,  iteration);
            } else {
                set.insert(i as usize, Vec::new());
                set[i as usize].insert(t as usize , iteration);
            }
            t+=1.0;
        }
        i += 1.0;
    }
    MandSet {
        pixels:set
    }
}
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
pub struct MandSet {
    pub pixels: Vec<Vec<i32>>
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
            close_color: [0.0, 0.0, 1.2, 1.0],
            not_color:[0.0, 0.0, 0.6, 1.0],
            tower_color: [0.0, 0.6, 0.2, 1.0]
        }
    }
}
pub struct MenuView {
    pub settings:MenuViewSet,
    pub msets:Vec<MandSet>,
    pub drawn_b4: bool,
    pub act_point:f64,
    pub to_point: f64,
}
impl MenuView {
    pub fn new(settings:MenuViewSet) -> MenuView {
        MenuView {
            settings:settings,
            msets:Vec::new(),
            drawn_b4: false,
            to_point:-1.0,
            act_point:0.5,
        }
    }

    pub fn init_sets(&mut self){
        self.msets = vec![gen_set([0.0,0.0,self.settings.size_x,self.settings.size_y])/*, gen_set([0.0,0.0,self.settings.size_x,self.settings.size_y/2.0])*/];
    }
    pub fn draw<G: Graphics, C>(&mut self, controller:&mut MenuController, glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>, G: Graphics<Texture = opengl_graphics::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed, CircleArc,Text};
        use std::f64::consts::PI;
        if !self.drawn_b4 {
            self.init_sets();
            self.drawn_b4 = true;
        }

        let ref settings = self.settings;
        let c_line = Line::new(settings.edge_color_tile, settings.tile_edge_radius);
        let mut t = 0.0;
        while t < (settings.size_y / settings.tile_size).round(){
            c_line.draw([0.0,t*settings.tile_size,settings.size_x,t*settings.tile_size],&c.draw_state,c.transform,g);
            t+=1.0;
        }
        let edLine = Line::new(settings.edge_color_board, settings.board_edge_radius);
        let mut rs = vec![0.0 * PI, 0.5 * PI, 1.0 * PI, 1.5 * PI];
        let mut i = 0.0;
        let dxs = [0.0,0.0,settings.size_x/2.0,0.0];
        let dys = [0.0,0.0,0.0,settings.size_y/2.0];
        /* while i < 4.0 {
           println!("Drawing {}",i);
           let toU = match i {
           0.0 => 1,
           1.0 => 0,
           2.0 => 1,
           3.0 => 0,
           _ => 0,
           };*/
        let mut x  = 0;
        let dir = 0.5;
        if controller.mandle {
            while x < self.msets[0].pixels.len() {
                let mut y = 0;
                while y < self.msets[0].pixels[x].len() {
                    let mut color = &settings.close_color;
                    if self.msets[0].pixels[x][y] > 75 {
                        //let pRect = [x as f64, y as f64, (x+1) as f64, (y+1) as f64];
                        let pRect = graphics::rectangle::square(0.0,0.0,1.0);
                        Rectangle::new(*color).draw(pRect, &c.draw_state, c.transform.trans(settings.size_x/2.0,settings.size_y/2.0).rot_rad((controller.menu.act_point-0.5) * PI).trans((x as f64)  - settings.size_x/8.0, (y as f64 - settings.size_y/2.0) as f64), g);

                    }

                    y+=1;
                }
                x+=1;
            }
        }
        edLine.draw([0.0,0.0,20.0,20.0],&c.draw_state,c.transform,g);

        i+=1.0;
        //}

        edLine.draw([0.0,0.0,settings.size_x,0.0],&c.draw_state, c.transform, g);
        edLine.draw([0.0,0.0,0.0,settings.size_y], &c.draw_state, c.transform,g);
        edLine.draw([0.0,settings.size_y,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        edLine.draw([settings.size_x,settings.size_y,settings.size_x,0.0], &c.draw_state, c.transform, g);
        let mut i =1.0;
        let chLine = Line::new(settings.edge_color_choice, settings.edge_choice_radius);
        chLine.draw([0.0,0.0,settings.size_x,settings.size_y], &c.draw_state, c.transform, g);
        chLine.draw([0.0,settings.size_y,settings.size_x,0.0],&c.draw_state,c.transform,g);
        CircleArc::new(settings.edge_color_choice, settings.circle_radius, 0.0, PI*1.9999).draw([(settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0 - settings.size_x/4.0), (settings.size_x/16.0+settings.size_x/4.0),(settings.size_x/16.0+settings.size_x/4.0)],&c.draw_state,c.transform.trans(settings.size_x/2.0 - ((settings.size_x/16.0 - settings.size_x/8.0)/2.0),settings.size_y/2.0 - ((settings.size_x/15.0-settings.size_x/8.0)/2.0)),g);
    }
}
