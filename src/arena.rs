use graphics::types::Color;
use std::collections::HashMap;
const SIZE: f64 = 50.0;
const SPEED: f64 = 3.0;
#[derive(Clone, Copy)]
pub struct LightDrop {
    pub position: [f64;2],
    pub owner:i32,
}

pub struct Lightcycle {
    pub position: [f64;2],
    pub dir:f64,
    pub dead: bool,
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
    pub fn create_cycle(&mut self, pos: [f64; 2]) {
        let mut nc = Lightcycle {
            dir:2.0,
            trail:Vec::new(),
            dead: false,
            owner: self.cycles.len() as i32 +1,
            position: pos
        };
        &self.cycles.push(nc);
    }
    pub fn check_deaths(&mut  self)  -> Vec<i32>{
        let mut cycles = &mut self.cycles;
        let mut y = 0;
        let mut deaths:Vec<i32> = Vec::new();
        for (key, drop) in &self.trails {
            let mut i = 0;
            while i != cycles.len() {
                if cycles[i].dead != true {
                    if Arena::is_dead(&cycles[i], drop) {
                        cycles[i].dead = true;
                        let mut t = 0;
                        while t < 30 {
                            deaths.push((i+1) as i32);
                            t+= 1
                        }
                    } else {
                        i += 1;
                    }
                } else {
                    i+=1;
                }
            }        }
        deaths
    }
    pub fn check_game(&mut self) -> bool {
        if self.cycles.iter().filter(|x| !x.dead).collect::<Vec<&Lightcycle>>().len() < 2 {
            println!("GAME OVER");
            self.reset_game();
            true
        } else {
            false
        }
    }
    pub fn reset_game(&mut self) {
        if self.cycles.len() > 0 {
            self.cycles = Vec::new();
        }
        self.create_cycle([900.0, 100.0]);
        self.create_cycle([300.0, 100.0]);
        self.trails = HashMap::new();
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
        } else {
            false
        }
    }
    pub fn move_cycles(&mut self, sizes: (f64, f64), multi:f64) -> Vec<i32>{
        let mut deaths : Vec<i32> = Vec::new();
        for mut cy in &mut self.cycles {
            if cy.dead != true {
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
                    0.0 => (cy.position[0], cy.position[1] - (SPEED*multi)),
                    1.0 => (cy.position[0] - (SPEED * multi), cy.position[1]),
                    2.0 => (cy.position[0], cy.position[1] + (multi *SPEED)),
                    3.0 => (cy.position[0] + (multi * SPEED),cy.position[1]),
                    _ => (0.0, 0.0)
                };
                if to.0 - (xd/2.0) > 0.0 && to.0 +(xd/2.0)  < sizes.0 && to.1 -(yd/2.0)> 0.0 && to.1 +(yd/2.0) < sizes.1 {
                    let flx = (cy.position[0] / 15.0).floor();
                    let fly = (cy.position[1] / 15.0).floor();
                    cy.position[0] = to.0;
                    cy.position[1] = to.1;
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

                } else {
                        cy.dead = true;
                        let mut t = 0;
                        while t < 30 {
                            deaths.push((cy.owner) as i32);
                            t+= 1
                        }

                }
            }
        }
        deaths
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
