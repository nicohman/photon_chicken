use piston::input::{GenericEvent, UpdateArgs};
use Menu;
use menu::Mode;
pub struct MenuController {
    pub menu: Menu,
    pub mandle:bool
}
impl MenuController {
    pub fn new(menu: Menu) -> MenuController {
        MenuController {
            menu:menu,
            mandle:false
        }
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E)
    {
                use piston::input::Key;
        use piston::input::Button::Keyboard;
        if let Some(Keyboard(Key::Right)) = e.press_args() {
                self.menu.to_point = 0.5;
            //self.menu.switch(Mode {name: String::from("cycles")});
        }

        if let Some(Keyboard(Key::D)) = e.press_args() {
                self.menu.to_point = 0.5;
        
           // self.menu.switch(Mode {name:String::from("cycles")});
        }
        if let Some(Keyboard(Key::A)) = e.press_args() {
           // self.menu.switch(Mode {name:String::from("tower")});
           self.menu.to_point = 1.5; 
        
        }
        if let Some(Keyboard(Key::Left)) = e.press_args() {
            
           self.menu.to_point = 1.5; 
            //self.menu.switch(Mode {name:String::from("tower")});
        
        }
        if let Some(Keyboard(Key::M)) = e.press_args() {
            self.mandle = !self.mandle;
        }
        if let Some(UpdateArgs) = e.update_args() {
            let dt = e.update_args().unwrap().dt;
        if self.menu.to_point !=-1.0{
        if self.menu.to_point +0.05 > self.menu.act_point && self.menu.to_point < self.menu.act_point + 0.05 {
            let cp = self.menu.to_point.clone();
            self.menu.run(cp);
        }
        if self.menu.to_point < self.menu.act_point {
            self.menu.act_point  = self.menu.act_point - 10.0* dt;
        } else {
            self.menu.act_point = self.menu.act_point + 10.0 * dt;
        }
        }
        }
    }
    pub fn update (&mut self) {
             
    }
}
