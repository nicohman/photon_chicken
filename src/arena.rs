use std::collections::HashMap;
const SIZE: f64 = 50.0;
const SPEED: f64 = 1.0;
#[derive(Clone, Copy)]
pub struct LightDrop {
    pub position: [f64;2],
    pub owner:i32,
}

pub struct Lightcycle {
    pub position: [f64;2],
    pub dir:f64,
    pub num: i32,
    pub dead: bool,
    pub owner: i32,
    pub trail:Vec<LightDrop>
}
pub struct Arena {
    pub start_tick:f64,
    pub paused: bool,
    pub cycles:Vec<Lightcycle>,
    pub trails:HashMap<String, LightDrop>,
}
impl Arena {
    pub fn new ()-> Arena {
        let mut a =  Arena {
            cycles: Vec::new(),
            paused: false,
            start_tick:0.0,
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
num:50,
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
    pub fn check_game(&mut self) -> i32 {
        if self.cycles.iter().filter(|x| !x.dead).collect::<Vec<&Lightcycle>>().len() == 1 {
            let win = self.cycles.iter().filter(|x| !x.dead).next().unwrap().owner.to_owned();
            self.reset_game();
            self.cycles[(win-1) as usize].num +=20;
            for cy in &mut self.cycles {
                if cy.owner != win && cy.num >= 50 {
                    if cy.num <= 60 {
                        cy.num = 50;
                    } else {
                        cy.num -=10;
                    }
                }
            }
            win
        } else if self.cycles.iter().filter(|x| !x.dead).collect::<Vec<&Lightcycle>>().len() < 1 {
            self.reset_game();
            -2
        } else{
            return -1;
        }
    }
    pub fn reset_game(&mut self) {
        if self.cycles.len() > 0 {
            self.cycles = Vec::new();
        }
        self.create_cycle([900.0, 100.0]);
        self.create_cycle([300.0, 100.0]);
        self.trails = HashMap::new();
        self.start_tick = 3.0;
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
        let last = cy.trail.first().unwrap();
        let mut ok = true;
        let mut t = 0;
        let mut lim = 4;
        if cy.trail.len() < 4 {
            lim = cy.trail.len();
        }
        while t < lim {
             if drop.position[0] == cy.trail[t].position[0] && drop.position[1] == cy.trail[t].position[1] 
             {
                ok = false   
             } else {
                 
             }
             t+=1;
        }
        if (( (  (  ((drop.position[0] * 15.0) +15.0) > (cy.position[0])  ) && (  (drop.position[0] * 15.0) < (cy.position[0]+xd)  )  ) && (  (((drop.position[1] * 15.0 )+  15.0) > (cy.position[1] )  ) && (  (drop.position[1] * 15.0) < (cy.position[1] + yd)  )))) && ((cy.owner != drop.owner ) || ok) {

            //println!("{}x{},{}x{},{}x{}",last.position[0],last.position[1],drop.position[0],drop.position[1],cy.position[0] / 15.0,cy.position[1] / 15.0);
            true
        } else {
            false
        }
    }
    pub fn move_cycles(&mut self, sizes: (f64, f64), multi:f64) -> Vec<i32>{
        let mut deaths : Vec<i32> = Vec::new();
        if !self.paused{
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
                        if cy.trail.len() >= cy.num as usize {
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
