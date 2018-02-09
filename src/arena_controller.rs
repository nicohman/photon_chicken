use piston::input::GenericEvent;
use Arena;
pub struct ArenaController {
    pub arena: Arena,
}
impl ArenaController {
    pub fn new(mut arena: Arena) -> ArenaController {
        let mut c = ArenaController {
            arena: arena,
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
    }
    pub fn update(&mut self) {
        let ref mut arena = self.arena;
        arena.move_cycles();
    }
}
