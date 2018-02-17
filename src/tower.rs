const START: f64 = 8.0;
const INIT: f64 = 0.25;
const SPI_SPEED: f64 = 0.01;
const SHOT_SPEED : f64 =  1.5;
const U_SPEED : f64 = 1.0;
use rand::Rng;
use rand::os::OsRng;
#[derive(Clone, Copy)]
pub struct Spider {
    pub splitter: bool,
    pub cooldown: f64,
    pub position: [f64;2],
    pub splitting : bool,
    pub split_an: f64,
}
#[derive(Clone, Copy)]
pub struct Shot {
    pub position: [f64;2],
    pub dir: f64
}

pub struct User {
    pub position: [f64;2],
    pub id: i32,
    pub facing: f64,
    pub shot_cooldown: f64,
}
pub struct Tower {
    pub users: Vec<User>,
    pub spiders:Vec<Spider>,
    pub start_tick:f64,
    pub shots: Vec<Shot>,
    pub paused: bool,
}
impl Spider {
    pub fn drop(&mut self, dt:f64){
        self.cooldown -=dt;
        self.split_an -=dt;
    }
}
impl Tower {
    pub fn new() -> Tower {
        Tower {
            start_tick: 0.0,
            shots:Vec::new(),
            users:Vec::new(),
            spiders:Vec::new(),
            paused:false,
        }
    }
    pub fn shoot(&mut self, u: i32) {
        let ref mut cur = self.users[u as usize];
        if cur.shot_cooldown <= 0.0 {
            self.shots.push(Shot {
                position:[cur.position[0]+15.0,cur.position[1]+30.0],
                dir:cur.facing
            });

            println!("Shot!{}", cur.facing);
            cur.shot_cooldown = 0.5;
        }
    }
    pub fn move_u(&mut self, u: i32, sizes:[f64;2]){
        let ref mut cur = self.users[u as usize];
        println!("{}",cur.facing);
        let to_move = match cur.facing {
            0.0 => [cur.position[0], cur.position[1] - U_SPEED],
            0.5 => [cur.position[0] + U_SPEED, cur.position[1] - U_SPEED],
            1.0 => [cur.position[0] + U_SPEED, cur.position[1]],
            1.5 => [cur.position[0] + U_SPEED, cur.position[1] + U_SPEED],
            2.0 => [cur.position[0], cur.position[1] + U_SPEED],
            2.5 => [cur.position[0] - U_SPEED, cur.position[1] + U_SPEED],
            3.0 => [cur.position[0] - U_SPEED, cur.position[1]],
            3.5 => [cur.position[0] - U_SPEED, cur.position[1] - U_SPEED],
            _ => cur.position
        };
        if to_move[0] >= 0.0 && to_move[0] +30.0< sizes[0]  && to_move[1] >= 0.0 && to_move[1] +60.0< sizes[1] {
            cur.position = to_move;
        } else {

        }
    }
    pub fn tick(&mut self)  {
        let mut gen = OsRng::new().unwrap();
        let USER: [f64;2] = [30.0,60.0];
        let SPIDER : [f64;2] = [30.0,30.0];
        let mut i : usize = 0;
        while i < self.spiders.len(){
            let mut cur = &mut self.spiders[i].clone();
            let pos = cur.position;
            let mut cp = self.users.iter().collect::<Vec<&User>>();
            cp.sort_by(|x,y|{
                ((pos[0] - x.position[0]).powf(2.0) + (pos[1]-x.position[1]).powf(2.0)).sqrt().partial_cmp(&((pos[0] - y.position[0]).powf(2.0) + (pos[1]-y.position[1]).powf(2.0)).sqrt()).unwrap()
            });
            let u_p = cp[0].position;
            let mut o = 0;
            let mut skip = false;
            while o < self.shots.len() {
                if self.shots[o].position[0] + 15.0 > cur.position[0] && self.shots[o].position[0] < 30.0 + cur.position[0] && self.shots[o].position[1] + 15.0 > cur.position[1] && self.shots[o].position[1] < cur.position[1] + 30.0 {
                    println!("Kill spider");
                    self.spiders.remove(i);
                    self.shots.remove(o);
                    skip = true;
                }
                o += 1;

            }
            if skip {
                continue;
            }
            if cur.cooldown <= 0.0  && cur.splitter{
                self.spiders[i].cooldown = 8.0;
                self.spiders[i].split_an = 3.0;
                self.spiders[i].splitting = true;
                self.spiders[i].splitter = false;

            } else if cur.split_an <= 0.0 && cur.splitting {
                self.spiders.push(Spider {
                    position: [cur.position[0] + 15.0, cur.position[1] ],
                    splitter:true,
                    splitting:false,
                    split_an:0.0,
                    cooldown:8.0,
                });
                self.spiders.push(Spider {
                    position: [cur.position[0] -15.0 , cur.position[1] ],
                    splitter:true,
                    split_an:0.0,
                    splitting:false,
                    cooldown:8.0,
                });
                self.spiders[i].splitting = false;

            } else if cur.split_an > 0.0 && cur.splitting {

            } else {
                let rand_x = 1.0 * gen.next_f64();
                let rand_y = 1.0 * gen.next_f64();

                cur = &mut self.spiders[i];
                if u_p[0] < cur.position[0] {
                    cur.position[0] -= SPI_SPEED + rand_x;
                } else if u_p[0] > cur.position[0] {
                    cur.position[0] += SPI_SPEED + rand_x;
                }
                if u_p[1] < cur.position[1] {
                    cur.position[1] -= SPI_SPEED + rand_y;
                } else if u_p[1] > cur.position[1] {
                    cur.position[1] += SPI_SPEED + rand_y;
                }
            }
            if cur.position[0] < u_p[0] + USER[0] && cur.position[0] + SPIDER[0] > u_p[0] && u_p[1] + USER[1] > cur.position[1] && cur.position[1] + SPIDER[1] > u_p[1] {
                //println!("DEAD!");
            } else {
            }

            i+=1;
        }
        for mut cur in &mut self.shots {
            cur.position = match cur.dir {
                0.0 => [cur.position[0], cur.position[1] - SHOT_SPEED],
                0.5 => [cur.position[0] + SHOT_SPEED, cur.position[1] - SHOT_SPEED],
                1.0 => [cur.position[0] + SHOT_SPEED, cur.position[1]],
                1.5 => [cur.position[0] + SHOT_SPEED, cur.position[1] + SHOT_SPEED],
                2.0 => [cur.position[0], cur.position[1] + SHOT_SPEED],
                2.5 => [cur.position[0] - SHOT_SPEED, cur.position[1] + SHOT_SPEED],
                3.0 => [cur.position[0] - SHOT_SPEED, cur.position[1]],
                3.5 => [cur.position[0] - SHOT_SPEED, cur.position[1] - SHOT_SPEED],
                _ => cur.position
            };

        }
    }

    pub fn reset (&mut self) {
        self.users = vec![User {position:[400.0,400.0],facing:1.0,id:0,shot_cooldown:0.0}, User{position:[500.0,500.0],id:1,facing:1.0,shot_cooldown:0.0}];
        self.spiders = Vec::new();
        let mut i = 0.0;
        while i< START {
            self.spiders.push(Spider {
                position: [ 100.0 * i, 100.0 * i],
                cooldown:0.0,
                splitting:false,
                split_an:0.0,
                splitter: i < START * INIT,
            });
            i += 1.0;
        }
        self.start_tick = 3.0;
        self.paused = true;
    }
    pub fn check_win(&mut self, sizes: [f64;2]) -> i32{
        for u in &self.users {
            if u.position[0] + 30.0 > sizes[0] && u.position[0] < sizes[0]+60.0 && u.position[1] + 60.0 > sizes[1] && u.position[1] < sizes[1] + 90.0 {
                println!("Victory achieved");
                return u.id;
            }
        }
        -1
    }
}

