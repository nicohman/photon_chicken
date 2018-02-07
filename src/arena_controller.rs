use piston::input::GenericEvent;
use Arena;
pub struct ArenaController {
    pub arena: Arena,
}
impl ArenaController {
    pub fn new(arena: Arena) -> ArenaController {
        ArenaController {
            arena: arena,
        }
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
    
    }
}
