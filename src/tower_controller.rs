use piston::input::{GenericEvent, UpdateArgs};
use Tower;
use std::num;
pub struct TowerController {
    pub tower: Tower,
    pub keys: Keys,
    pub score:Vec<i32>
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
impl TowerController {
    pub fn new (mut tower: Tower) -> TowerController {
        let mut t = TowerController {
            tower:tower,
            keys: Keys::new(),
            score:vec![0,0,0,0]
        };
        t
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E)
    {
        use piston::input::Key;
        use piston::input::Button::Keyboard;
        let ref mut tower = self.tower;
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

        if let Some(UpdateArgs) = e.update_args() {
            let dt = e.update_args().unwrap().dt;
            for mut sp in &mut tower.spiders {
                sp.drop(dt);
            }
        }
    }

    pub fn update(&mut self, sizes:(f64, f64)) {
        /*let mut i : usize= 0;
          while i  < spiders.len() {
          let mut cur = &mut spiders[i];
          let pos = cur.position;
          self.tower.users.sort_by(|x,y|{
          ((pos[0] - x.position[0]).powf(2.0) + (pos[1]-x.position[1]).powf(2.0)).sqrt().partial_cmp(&((pos[0] - y.position[0]).powf(2.0) + (pos[1]-y.position[1]).powf(2.0)).sqrt()).unwrap()
          });
          cur.tick(self.tower.users[0].position, spiders);
          i+=1;
          }*/
        if self.keys.left && self.keys.up {
            self.tower.users[0].facing = 3.5;
        } else if self.keys.left && self.keys.down {
            self.tower.users[0].facing = 2.5;
        } else if self.keys.left {
            self.tower.users[0].facing = 3.0;
        } else if self.keys.right && self.keys.up {
            self.tower.users[0].facing = 0.5;
        } else if self.keys.right && self.keys.down {
            self.tower.users[0].facing = 1.5;
        } else if self.keys.right {
            self.tower.users[0].facing = 1.0;
        } else if self.keys.down {
            self.tower.users[0].facing = 2.0;
        } else if self.keys.up {
            self.tower.users[0].facing = 0.0;
        }
        if self.keys.a && self.keys.w {
            self.tower.users[1].facing = 3.5;
        } else if self.keys.a && self.keys.s {
            self.tower.users[1].facing = 2.5;
        } else if self.keys.a {
            self.tower.users[1].facing = 3.0;
        } else if self.keys.d && self.keys.w {
            self.tower.users[1].facing = 0.5;
        } else if self.keys.d && self.keys.s {
            self.tower.users[1].facing = 1.5;
        } else if self.keys.d {
            self.tower.users[1].facing = 1.0;
        } else if self.keys.s {
            self.tower.users[1].facing = 2.0;
        } else if self.keys.w {
            self.tower.users[1].facing = 0.0;
        }

        if self.keys.up  || self.keys.down || self.keys.right || self.keys.left {
            self.tower.move_u(0);
        }
        if self.keys.w || self.keys.a || self.keys.s || self.keys.d  {
            self.tower.move_u(1);
        }

        self.tower.tick();
    }
}
