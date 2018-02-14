use piston::input::{GenericEvent, UpdateArgs};
use Tower;
pub struct TowerController {
    pub tower: Tower,
    pub score:Vec<i32>
}
impl TowerController {
    pub fn new (mut tower: Tower) -> TowerController {
        TowerController {
            tower:tower,
            score:vec![0,0,0,0]
        }
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E)
    {

    }
    pub fn update(&mut self, sizes:(f64, f64)) {
    }
}
