use graphics::types::Color;
const SIZE: f64 = 50.0;
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
        Arena {
            cycles: Vec::new(),
            trails:[[LightDrop {
                position: [0.0;2],
                owner: 0,
            };SIZE as usize];SIZE as usize],
        }
    }
    pub fn tile (&mut self, pos: [f64; 2])-> LightDrop {
        self.trails[pos[0] as usize][pos[1] as usize]
    }
    pub fn create_cycle(&mut self, pos: [f64; 2], clr: Color) {
        self.cycles.push(Lightcycle {
            dir:1.0,
            trail:Vec::new(),
            color:clr,
            position: pos
        });
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
