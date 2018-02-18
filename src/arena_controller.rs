use piston::input::{GenericEvent, UpdateArgs};
use Arena;
use MenuController;
use menu::Mode;
pub struct ArenaController {
    pub arena: Arena,
    pub multi: f64,
    pub score: Vec<i32>,
    pub deaths: Vec<i32>
}
impl ArenaController {
    pub fn new(mut arena: Arena) -> ArenaController {
        let mut c = ArenaController {
            arena: arena,
            multi:1.0,
            score: vec![0,0,0,0],
            deaths:Vec::new(),
        };
        c
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, menu: &mut MenuController, e: &E)
    {
        use piston::input::Key;
        use piston::input::Button::Keyboard;
        let ref mut arena = self.arena;
        if let Some(Keyboard(Key::Left)) = e.press_args()   {
            if arena.cycles[0].dir != 3.0{
                arena.cycles[0].dir = 1.0;
            }
        }
        if let Some(Keyboard(Key::Right)) = e.press_args()  {
            if arena.cycles[0].dir != 1.0{
                arena.cycles[0].dir = 3.0;
            }
        }
        if let Some(Keyboard(Key::Down)) = e.press_args() {
            if arena.cycles[0].dir != 0.0
            {
                arena.cycles[0].dir = 2.0;
            }
        }
        if let Some(Keyboard(Key::Up)) = e.press_args() {
            if arena.cycles[0].dir != 2.0{
                arena.cycles[0].dir = 0.0;
            }
        }
        if let Some(Keyboard(Key::A)) = e.press_args()  {
            if arena.cycles[1].dir != 3.0 {
                arena.cycles[1].dir = 1.0;
            }
        }
        if let Some(Keyboard(Key::D)) = e.press_args() {
            if  arena.cycles[1].dir != 1.0 {
                arena.cycles[1].dir = 3.0;
            }
        }
        if let Some(Keyboard(Key::S)) = e.press_args() {
            if arena.cycles[1].dir != 0.0 {
                arena.cycles[1].dir = 2.0;
            }
        }
        if let Some(Keyboard(Key::W)) = e.press_args() {
            if arena.cycles[1].dir != 2.0 {
                arena.cycles[1].dir = 0.0;
            }
        }
        if let Some(Keyboard(Key::P)) = e.press_args() {
            arena.paused = !arena.paused;
        }
        if let Some(Keyboard(Key::Backspace)) = e.press_args() {
            menu.menu.switch(Mode {name:String::from("menu")});
        }
        if let Some(UpdateArgs) = e.update_args() {
            if arena.start_tick != -1.0 {
                arena.start_tick -= 4.0 * e.update_args().unwrap().dt;
            }
        }
    }
    pub fn update(&mut self, sizes:(f64, f64)) {

        self.multi = self.multi * 1.0001;
        let ref mut arena = self.arena;
        if arena.start_tick != -1.0 {
            arena.paused = true;
            if arena.start_tick < 0.0 {
                arena.paused = false;
                arena.start_tick = -1.0;
            }
        }
        for d in arena.move_cycles(sizes, self.multi) {
            self.deaths.push(d);
        }
        for d in arena.check_deaths(){
            self.deaths.push(d);
        }
        let check = arena.check_game();
        if check != -1 {
            if check  != -2 {
                self.score[(check -1) as usize] += 1;
            }
            self.multi = 1.0;
        }
    }
}
