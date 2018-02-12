use graphics::types::Color;
use std::collections::HashMap;
const SIZE: f64 = 50.0;
const SPEED: f64 = 4.0;
#[derive(Clone, Copy)]
pub struct LightDrop {
    pub position: [f64;2],
    pub owner:i32,
}

pub struct Lightcycle {
    pub position: [f64;2],
    pub color:Color,
    pub dir:f64,
    pub owner: i32,
    pub trail:Vec<LightDrop>
}
pub struct Arena {
    pub cycles:Vec<Lightcycle>,
    pub trails:HashMap<String, LightDrop>,
}
impl Arena {
    pub fn new ()-> Arena {
        let mut a =  Arena {
            cycles: Vec::new(),
            trails:HashMap::new(),
        };
        a
    }
    pub fn tile (&mut self, pos: [f64; 2])-> LightDrop {
        let name = pos[0].to_string()+"x"+&pos[1].to_string();
        *(self.trails.get(&name).or(Some(&LightDrop {
            position:pos,
            owner:0,
        } )).unwrap())
    }
    pub fn create_cycle(&mut self, pos: [f64; 2], clr: Color) {
        let mut nc = Lightcycle {
            dir:1.0,
            trail:Vec::new(),
            color:clr,
            owner: self.cycles.len() as i32 +1,
            position: pos
        };
        &self.cycles.push(nc);
    }
    pub fn check_deaths(&mut self) {
        let mut cycles = &mut self.cycles;
        let mut y = 0;
        for (key, drop) in &self.trails {
            let mut i = 0;
            while i != cycles.len() {
                if Arena::is_dead(&cycles[i], drop) {
                    cycles.remove(i);
                } else {
                    i += 1;
                }

            }        }
    }
    pub fn kill_cycle(&mut self, owner:i32){
        self.cycles.remove((owner-1) as usize);
    }
    pub fn is_dead(cy: &Lightcycle, drop: &LightDrop) -> bool {
        let xd = match cy.dir {
            1.0 => 30.0,
            3.0 => 30.0,
            2.0 => 15.0,
            0.0 => 15.0,
            _ => 15.0,
        } / 2.0;
        let yd = match cy.dir {
            1.0 => 15.0,
            3.0 => 15.0,
            2.0 => 30.0,
            0.0 => 30.0,
            _ => 30.0,
        } / 2.0;
        if (( (  (  (drop.position[0] * 15.0 +xd ) > (cy.position[0])  ) && (  (drop.position[0] * 15.0) < (cy.position[0]+xd)  )  ) && (  ((drop.position[1] * 15.0 +  yd) > (cy.position[1] )  ) && (  (drop.position[1] * 15.0) < (cy.position[1] + yd)  )))) && cy.owner != drop.owner {
            true
                // ::std::process::exit(0);
        } else {
            false
        }
    }
    pub fn move_cycles(&mut self, sizes: (f64, f64)) {
        for mut cy in &mut self.cycles {
            let xd = match cy.dir {
                0.0 => 15.0,
                1.0 => 30.0,
                3.0 => 30.0,
                2.0 => 15.0,
                _ => 0.0
            };
            let yd = match cy.dir {
                1.0 => 15.0,
                0.0 => 30.0,
                2.0 => 30.0,
                3.0 => 15.0,
                _ => 0.0
            };
            let mut to = match cy.dir {
                0.0 => (cy.position[0], cy.position[1] - SPEED),
                1.0 => (cy.position[0] - SPEED, cy.position[1]),
                2.0 => (cy.position[0], cy.position[1] + SPEED),
                3.0 => (cy.position[0] + SPEED,cy.position[1]),
                _ => (0.0, 0.0)
            };
            if to.0 - (xd/2.0) > 0.0 && to.0 +(xd/2.0)  < sizes.0 && to.1 -(yd/2.0)> 0.0 && to.1 +(yd/2.0) < sizes.1 {
                let flx = (cy.position[0] / 15.0).floor();
                let llx = (to.0 / 15.0).floor();
                let fly = (cy.position[1] / 15.0).floor();
                let lly = (to.0 /15.0).floor();
                // add_from([flx, fly], cy.owner);
                cy.position[0] = to.0;
                cy.position[1] = to.1;
                let got = *(self.trails.get(&(llx.to_string()+"x"+&lly.to_string())).or(Some(&LightDrop {
                    position:[llx, lly],
                    owner:0,
                } )).unwrap());
                if got.owner > 0 && got.owner != cy.owner && llx != flx && lly != fly {
                    // println!("DEAD AT {},{},{},{} owned by {} and  i am {}", llx,lly,fly,lly,got.owner,cy.owner);
                    //::std::process::exit(0);
                } else {
                    let mut pos = [flx, fly];
                    let mut owner = cy.owner;
                    let mut last: [f64;2];
                    let name = pos[0].to_string()+"x"+&pos[1].to_string();
                    if cy.trail.len() >= 50 {
                        last = cy.trail.pop().unwrap().position;
                        self.trails.remove(&(last[0].to_string()+"x"+&last[1].to_string()));
                    }
                    if self.trails.contains_key(&name) {
                    } else {
                        let light = LightDrop {
                            position: pos,
                            owner:owner
                        };
                        cy.trail.insert(0, light.clone());
                        self.trails.insert(name, light);
                    }
                }
            } else {
                //println!("NOPE{:?}{:?}", cy.position, sizes);
            }
        }
    }
    pub fn add_from(&mut self, pos: [f64;2], owner:i32) -> bool {
        let mut last: [f64;2];
        let name = pos[0].to_string()+"x"+&pos[1].to_string();
        if self.cycles[(owner-1) as usize].trail.len() >= 5 {

            last = self.cycles[(owner-1) as usize].trail.pop().unwrap().position;
            self.trails.remove(&name);
        }
        if self.trails.contains_key(&name) {
            false
        } else {
            let light = LightDrop {
                position: pos,
                owner:owner
            };
            self.cycles[(owner-1) as usize].trail.insert(0, light.clone());
            self.trails.insert(name, light);
            true
        }
    }
}
