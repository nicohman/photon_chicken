use piston::input::{GenericEvent, UpdateArgs};
use Menu;
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
    
    }
    pub fn update (&mut self) {
    
    }
}
