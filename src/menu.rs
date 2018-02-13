use graphics::types::Color;
#[derive(Clone, Copy)]
pub struct Mode {
    name:String
}
pub struct Menu {
    modes: Vec<Mode>,
    selected:Mode
}
impl Menu {
    pub fn new() -> Menu {
        Menu {
            modes:vec![Mode {name:String::from("cycles")}, Mode {name:String::from("menu")}],
            selected: Mode {
                name:"menu"
            }
        }
    }
    pub fn switch(to:
}
