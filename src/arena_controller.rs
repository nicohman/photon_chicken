use piston::input::GenericEvent;
use Arena;
pub struct ArenaController {
    pub arena: Arena,
    pub multi: f64,
    pub deaths: Vec<i32>
}
impl ArenaController {
    pub fn new(mut arena: Arena) -> ArenaController {
        let mut c = ArenaController {
            arena: arena,
            multi:1.0,
            deaths:Vec::new(),
        };
        c
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E)
    {
        use piston::input::Key;
        use piston::input::Button::Keyboard;
        let ref mut arena = self.arena;
        if let Some(Keyboard(Key::Left)) = e.press_args() {
            arena.cycles[0].dir = 1.0;
        }
        if let Some(Keyboard(Key::Right)) = e.press_args() {
            arena.cycles[0].dir = 3.0;
        }
        if let Some(Keyboard(Key::Down)) = e.press_args() {
            arena.cycles[0].dir = 2.0;
        }
        if let Some(Keyboard(Key::Up)) = e.press_args() {
            arena.cycles[0].dir = 0.0;
        }
        if let Some(Keyboard(Key::A)) = e.press_args() {
            arena.cycles[1].dir = 1.0;
        }
        if let Some(Keyboard(Key::D)) = e.press_args() {
            arena.cycles[1].dir = 3.0;
        }
        if let Some(Keyboard(Key::S)) = e.press_args() {
            arena.cycles[1].dir = 2.0;
        }
        if let Some(Keyboard(Key::W)) = e.press_args() {
            arena.cycles[1].dir = 0.0;
        }

    }
    pub fn update(&mut self, sizes:(f64, f64)) {
        self.multi = self.multi * 1.0001;
        println!("{}", self.multi);
        let ref mut arena = self.arena;
        for d in arena.move_cycles(sizes, self.multi) {
            self.deaths.push(d);
        }
        for d in arena.check_deaths(){
            self.deaths.push(d);
        }
        if arena.check_game() {
            self.multi = 1.0;
        }
    }
}
