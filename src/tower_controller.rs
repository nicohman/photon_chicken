use piston::input::{GenericEvent, UpdateArgs};
use Tower;
use std::cmp::Ordering;
use std::num;
use tower::User;
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
        /*let mut i : usize= 0;
        while i  < spiders.len() {
            let mut cur = &mut spiders[i];
            let pos = cur.position;
        self.tower.users.sort_by(|x,y|{
            ((pos[0] - x.position[0]).powf(2.0) + (pos[1]-x.position[1]).powf(2.0)).sqrt().partial_cmp(&((pos[0] - y.position[0]).powf(2.0) + (pos[1]-y.position[1]).powf(2.0)).sqrt()).unwrap()
        });
            cur.tick(self.tower.users[0].position, spiders);
        i+=1;
        }*/
        self.tower.tick();
    }
}
