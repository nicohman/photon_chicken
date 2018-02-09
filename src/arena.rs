use graphics::types::Color;
const SIZE: f64 = 50.0;
const SPEED: f64 = 2.0;
#[derive(Clone, Copy)]
pub struct LightDrop {
    position: [f64;2],
    owner:i32,
}
pub struct Lightcycle {
    pub position: [f64;2],
    color:Color,
    pub dir:f64,
    trail:Vec<LightDrop>
}
pub struct Arena {
    pub cycles:Vec<Lightcycle>,
    pub trails:[[LightDrop;SIZE as usize];SIZE as usize],
}
impl Arena {
    pub fn new ()-> Arena {
        let mut a =  Arena {
            cycles: Vec::new(),
            trails:[[LightDrop {
                position: [0.0;2],
                owner: 0,
            };SIZE as usize];SIZE as usize],
        };
        a
    }
    pub fn tile (&mut self, pos: [f64; 2])-> LightDrop {
        self.trails[pos[0] as usize][pos[1] as usize]
    }
    pub fn create_cycle(&mut self, pos: [f64; 2], clr: Color) {
        let mut nc = Lightcycle {
            dir:1.0,
            trail:Vec::new(),
            color:clr,
            position: pos
        };
        &self.cycles.push(nc);
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
                cy.position[0] = to.0;
                cy.position[1] = to.1;
            } else {
                println!("NOPE{:?}{:?}", cy.position, sizes);
            }
        }
    }
    pub fn add_from(&mut self, pos: [f64;2], owner:i32) -> bool {
        let mut last: [f64;2];
        if self.cycles[(owner-1) as usize].trail.len() > 5 {

            last = self.cycles[(owner-1) as usize].trail.pop().unwrap().position;
        } else {
            last = self.cycles[(owner-1) as usize].trail.last().unwrap().position;
        }
        self.trails[last[0] as usize][last[1] as usize] = LightDrop {
            position: [last[0], last[1]],
            owner:0
        };
        self.trails[pos[0] as usize][pos[1] as usize] = LightDrop {
            owner:owner,
            position: [pos[0], pos[1]]
        };
        true
    }
}
