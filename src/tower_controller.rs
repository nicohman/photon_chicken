use piston::input::{GenericEvent, UpdateArgs};
use Tower;
use std::num;
use MenuController;
use gilrs::{Gilrs, Button, Event};
use gilrs::Axis::{RightStickX,RightStickY,LeftStickX,LeftStickY};
use gilrs::Button::RightTrigger2;
use menu::Mode;
pub struct TowerController {
    pub tower: Tower,
    pub keys: Keys,
    pub cursor_pos: [f64;2],
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
            cursor_pos:[0.0;2],
            score:vec![0,0,0,0]
        };
        t
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, gilrs:&mut Gilrs,menu: &mut MenuController, e: &E)
    {
                while let Some(ev) = gilrs.next_event() {
            gilrs.update(&ev);
            // Do other things with event
        }
        use piston::input::Key;
        use piston::input::Button::Keyboard;
        use piston::input::mouse::MouseButton;
        use piston::input::Button;

        let ref mut tower = self.tower;
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
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
        if let Some(Keyboard(Key::E)) = e.press_args() {
            tower.shoot(0);
        }
        if gilrs[1].is_pressed(RightTrigger2) {
            tower.shoot(1);
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            println!("Mouse");
            let ref ca = self.cursor_pos;
            let ref cy = tower.users[0].position;
            let r = (ca[1] / (   (   (ca[0]-cy[0]).powi(2)+(ca[1]-cy[1]).powi(2)  ).sqrt()   )   ).asin();
            println!("HERE:{}",r);
        }
        if let Some(Keyboard(Key::RCtrl)) = e.press_args() {
            tower.shoot(0);
        }
        if let Some(Keyboard(Key::P)) = e.press_args() {
            tower.paused = !tower.paused;
        }
        if let Some(Keyboard(Key::Backspace)) = e.press_args() {
            menu.menu.act_point = 0.5;
            menu.menu.to_point = -1.0;

            menu.menu.switch(Mode {name:String::from("menu")});
        }
        if let Some(Keyboard(Key::Space)) = e.press_args() {
            tower.paused = !tower.paused;
        }
        if let Some(UpdateArgs) = e.update_args() {
            let dt = e.update_args().unwrap().dt;
            for mut sp in &mut tower.spiders {
                sp.drop(dt);
            }
            for mut u in &mut tower.users {
                u.shot_cooldown -= dt;
            }
            if tower.start_tick != -1.0 {
                tower.start_tick -= 4.0 * dt;
            }
            tower.tick_time += dt;
        }
    }
    pub fn update(&mut self, sizes:(f64, f64),gilrs:&mut Gilrs) {
        while let Some(ev) = gilrs.next_event() {
            gilrs.update(&ev);
            // Do other things with event
        }
        let c1 = vec![gilrs[1].value(LeftStickX),gilrs[1].value(LeftStickY),gilrs[1].value(RightStickX),gilrs[1].value(RightStickY)];
        let mut quad = 0;
        if c1[2] > 0.0 && c1[3] > 0.0 {
            quad = 0;
        } else if c1[2] > 0.0 {
            quad = 1;
        } else if c1[3] < 0.0 {
            quad = 2;
        } else if c1[3] > 0.0 {
            quad = 3;
        }
        let sdir = ((1.0/(c1[2].abs() + c1[3].abs())) * c1[2].abs()) as f64;
        println!("{}",sdir);
        self.tower.users[1].sfacing = (sdir + quad as f64 ) % 4.0;
        let movingC1 = c1[0] > 0.1 || c1[1] > 0.1 || c1[0] < -0.1 || c1[1] < -0.1;
        let c1RR = c1[0] > 0.1;
        let c1UR = c1[1] > 0.1;
        let c1LR = c1[0] < -0.1;
        let c1DR = c1[1] < -0.1;
        let c1RL = c1[2] > 0.1;
        let c1UL = c1[3] > 0.1;
        let c1LL = c1[2] < -0.1;
        let c1DL = c1[3] < -0.1;
        if c1LR && c1UR {
            self.tower.users[1].facing = 3.5;
        } else if c1LR && c1DR {
            self.tower.users[1].facing = 2.5;
        } else if c1LR {
            self.tower.users[1].facing = 3.0;
        } else if c1RR && c1UR {
            self.tower.users[1].facing = 0.5;
        } else if c1RR && c1DR {
            self.tower.users[1].facing = 1.5;
        } else if c1RR {
            self.tower.users[1].facing = 1.0;
        } else if c1DR{
            self.tower.users[1].facing = 2.0;
        } else if c1UR {
            self.tower.users[1].facing = 0.0;
        }

        /*if c1LL && c1UL {
            self.tower.users[1].sfacing = 3.5;
        } else if c1LL && c1DL {
            self.tower.users[1].sfacing = 2.5;
        } else if c1LL {
            self.tower.users[1].sfacing = 3.0;
        } else if c1RL && c1UL {
            self.tower.users[1].sfacing = 0.5;
        } else if c1RL && c1DL {
            self.tower.users[1].sfacing = 1.5;
        } else if c1RL {
            self.tower.users[1].sfacing = 1.0;
        } else if c1DL{
            self.tower.users[1].sfacing = 2.0;
        } else if c1UL {
            self.tower.users[1].sfacing = 0.0;
        }*/
        if (self.keys.left && self.keys.up) {
            self.tower.users[0].sfacing = 3.5;
        } else if self.keys.left && self.keys.down {
            self.tower.users[0].sfacing = 2.5;
        } else if self.keys.left {
            self.tower.users[0].sfacing = 3.0;
        } else if self.keys.right && self.keys.up {
            self.tower.users[0].sfacing = 0.5;
        } else if self.keys.right && self.keys.down {
            self.tower.users[0].sfacing = 1.5;
        } else if self.keys.right {
            self.tower.users[0].sfacing = 1.0;
        } else if self.keys.down {
            self.tower.users[0].sfacing = 2.0;
        } else if self.keys.up {
            self.tower.users[0].sfacing = 0.0;
        }
        if self.keys.a && self.keys.w {
            self.tower.users[0].facing = 3.5;
        } else if self.keys.a && self.keys.s {
            self.tower.users[0].facing = 2.5;
        } else if self.keys.a {
            self.tower.users[0].facing = 3.0;
        } else if self.keys.d && self.keys.w {
            self.tower.users[0].facing = 0.5;
        } else if self.keys.d && self.keys.s {
            self.tower.users[0].facing = 1.5;
        } else if self.keys.d {
            self.tower.users[0].facing = 1.0;
        } else if self.keys.s {
            self.tower.users[0].facing = 2.0;
        } else if self.keys.w {
            self.tower.users[0].facing = 0.0;
        }
        if self.tower.start_tick != -1.0 {
            self.tower.paused = true;
            if self.tower.start_tick < 0.0 {
                self.tower.paused = false;
                self.tower.start_tick = -1.0;
            }
        }
        if !self.tower.paused {
            if movingC1 {
                self.tower.move_u(1, [sizes.0,sizes.1]);
            }
            if self.keys.w || self.keys.a || self.keys.s || self.keys.d  {
                self.tower.move_u(0, [sizes.0,sizes.1]);
            }
            self.tower.tick();
            let victory = self.tower.check_win([sizes.0/2.0 - 30.0, sizes.1/2.0 -45.0]);
            if victory != -1 && victory != -2{
                self.tower.reset([sizes.0,sizes.1]);
                self.score[victory as usize] += 1;
            } else if victory == -2 {
                self.tower.reset([sizes.0,sizes.1]);
            }
        }
    }
}
