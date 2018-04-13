use piston::input::{GenericEvent, UpdateArgs};
use Bricks;
use std::num;
use MenuController;
use menu::Mode;
pub struct BricksController {
    pub bricks: Bricks,
    pub keys : Keys,
    pub score: Vec<i32>
}
pub struct Keys {
    left:bool,
    right:bool,
    up:bool,
    down:bool,
    w:bool,
    a:bool,
    s:bool,
    d:bool
}
impl Keys {
    pub fn new () -> Keys {
        Keys {
            a:false,
            w:false,
            s:false,
            d:false,
            left:false,
            up:false,
            down:false,
            right:false,
        }
    }
}
impl BricksController {
    pub fn new (bricks:Bricks) -> BricksController {
        BricksController {
            bricks:bricks,
            keys:Keys::new(),
            score:vec![0,0,0,0]
        }
    }
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, menu: &mut MenuController, e: &E){

    }
    pub fn update(&mut self, sizes:(f64, f64)) {
    }
}
