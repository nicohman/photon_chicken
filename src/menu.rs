use graphics::types::Color;
pub struct Mode {
    pub name:String
}
pub struct Menu {
    pub modes: Vec<Mode>,
    pub selected:Mode,
    pub act_point:f64,
    pub to_point:f64,
}
impl Menu {
    pub fn new() -> Menu {
        Menu {
            modes:vec![Mode {name:String::from("tower")},Mode {name:String::from("cycles")}, Mode {name:String::from("menu")}],
            selected: Mode {
                name:String::from("menu")
            },
            act_point:0.5,
            to_point:-1.0,
        }
    }
    pub fn run(&mut self, dir: f64){
        match dir {
            0.0 => println!("Tanks not done yet!"),
            0.5 => self.switch(Mode {name:String::from("cycles")}),
            1.0 => println!("Breakout not done yet!"),
            1.5 => self.switch(Mode {name:String::from("tower")}),
            _ => println!("WTF")
        }
    }
    pub fn switch(&mut self, to: Mode) {
        self.selected = to;
    }
}
