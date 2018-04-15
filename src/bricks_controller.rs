use piston::input::{GenericEvent, UpdateArgs};
use Bricks;
use std::num;
use MenuController;
use menu::Mode;
pub struct BricksController {
    pub bricks: Bricks,
    pub keys : Keys,
    pub score: Vec<i32>
}
pub struct Keys {
    left:bool,
    right:bool,
    up:bool,
    down:bool,
    w:bool,
    a:bool,
    s:bool,
    d:bool
}
impl Keys {
    pub fn new () -> Keys {
        Keys {
            a:false,
            w:false,
            s:false,
            d:false,
            left:false,
            up:false,
            down:false,
            right:false,
        }
    }
}
impl BricksController {
    pub fn new (bricks:Bricks) -> BricksController {
        BricksController {
            bricks:bricks,
            keys:Keys::new(),
            score:vec![0,0,0,0]
        }
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, menu: &mut MenuController, e: &E){
        use piston::input::Key;
        use piston::input::Button::Keyboard;
        use piston::input::mouse::MouseButton;
        use piston::input::Button;
        let ref mut bricks = self.bricks;

        if let Some(Keyboard(Key::Left)) = e.press_args() {
            self.keys.left = true;
        }
        if let Some(Keyboard(Key::Right)) = e.press_args() {
            self.keys.right = true;
        }
        if let Some(Keyboard(Key::Up)) = e.press_args() {
            self.keys.up = true;
        }
        if let Some(Keyboard(Key::Down)) = e.press_args() {
            self.keys.down = true;
        }
        if let Some(Keyboard(Key::Left)) = e.release_args() {
            self.keys.left = false;
        }
        if let Some(Keyboard(Key::Right)) = e.release_args() {
            self.keys.right = false;
        }
        if let Some(Keyboard(Key::Up)) = e.release_args() {
            self.keys.up = false;
        }
        if let Some(Keyboard(Key::Down)) = e.release_args() {
            self.keys.down = false;

        }
        if let Some(Keyboard(Key::Backspace)) = e.press_args() {
            menu.menu.act_point = 0.5;
            menu.menu.to_point = -1.0;

            menu.menu.switch(Mode {name:String::from("menu")});
        }
        if let Some(Keyboard(Key::W)) = e.press_args() {
            self.keys.w =  true;
        }
        if let Some(Keyboard(Key::A)) = e.press_args() {
            self.keys.a = true;
        }
        if let Some(Keyboard(Key::D)) = e.press_args() {
            self.keys.d = true;
        }
        if let Some(Keyboard(Key::S)) = e.press_args() {
            self.keys.s = true;
        }
        if let Some(Keyboard(Key::D)) = e.release_args() {
            self.keys.d = false;
        }
        if let Some(Keyboard(Key::W)) = e.release_args() {
            self.keys.w = false;
        }
        if let Some(Keyboard(Key::A)) = e.release_args() {
            self.keys.a = false;
        }
        if let Some(Keyboard(Key::S)) = e.release_args() {
            self.keys.s = false;
        }
        if let Some(Keyboard(Key::E)) = e.press_args() {
            bricks.shoot(1);
        }
        if let Some(UpdateArgs) = e.update_args() {
            let dt = UpdateArgs.dt;
            bricks.tick(dt);
        }
    }
    pub fn update(&mut self, sizes:(f64, f64)) {
        if self.keys.left && self.keys.up {
            self.bricks.users[0].facing = 3.5;
        } else if self.keys.left && self.keys.down {
            self.bricks.users[0].facing = 2.5;
        } else if self.keys.left {
            self.bricks.users[0].facing = 3.0;
        } else if self.keys.right && self.keys.up {
            self.bricks.users[0].facing = 0.5;
        } else if self.keys.right && self.keys.down {
            self.bricks.users[0].facing = 1.5;
        } else if self.keys.right {
            self.bricks.users[0].facing = 1.0;
        } else if self.keys.down {
            self.bricks.users[0].facing = 2.0;
        } else if self.keys.up {
            self.bricks.users[0].facing = 0.0;
        }
        if self.keys.a && self.keys.w {
            self.bricks.users[1].facing = 3.5;
        } else if self.keys.a && self.keys.s {
            self.bricks.users[1].facing = 2.5;
        } else if self.keys.a {
            self.bricks.users[1].facing = 3.0;
        } else if self.keys.d && self.keys.w {
            self.bricks.users[1].facing = 0.5;
        } else if self.keys.d && self.keys.s {
            self.bricks.users[1].facing = 1.5;
        } else if self.keys.d {
            self.bricks.users[1].facing = 1.0;
        } else if self.keys.s {
            self.bricks.users[1].facing = 2.0;
        } else if self.keys.w {
            self.bricks.users[1].facing = 0.0;
        }

        if !self.bricks.paused {
            if self.keys.up  || self.keys.down || self.keys.right || self.keys.left {
                self.bricks.move_u(0, [sizes.0,sizes.1]);
            }
            if self.keys.w || self.keys.a || self.keys.s || self.keys.d  {
                self.bricks.move_u(1, [sizes.0,sizes.1]);
            }
            /*let victory = self.tower.check_win([sizes.0/2.0 - 30.0, sizes.1/2.0 -45.0]);
              if victory != -1 && victory != -2{
              self.tower.reset([sizes.0,sizes.1]);
              self.score[victory as usize] += 1;
              } else if victory == -2 {
              self.tower.reset([sizes.0,sizes.1]);
              }*/
        }
    }

}
