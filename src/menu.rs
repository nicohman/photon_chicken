use graphics::types::Color;
pub struct Mode {
    pub name:String
}
pub struct Menu {
    pub modes: Vec<Mode>,
    pub selected:Mode
}
impl Menu {
    pub fn new() -> Menu {
        Menu {
            modes:vec![Mode {name:String::from("cycles")}, Mode {name:String::from("menu")}],
            selected: Mode {
                name:String::from("menu")
            }
        }
    }
    pub fn switch(&mut self, to: Mode) {
        self.selected = to;
    }
}
