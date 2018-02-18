use piston::input::{GenericEvent, UpdateArgs};
use Menu;
use menu::Mode;
pub struct MenuController {
    pub menu: Menu,

}
impl MenuController {
    pub fn new(menu: Menu) -> MenuController {
        MenuController {
            menu:menu
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
    }
    pub fn update (&mut self) {
             
    }
}
