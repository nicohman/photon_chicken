const START: f64 = 8.0;
const INIT: f64 = 0.25;
const SPI_SPEED: f64 = 0.5;
const U_SPEED : f64 = 1.0;
#[derive(Clone, Copy)]
pub struct Spider {
    pub splitter: bool,
    pub cooldown: f64,
    pub position: [f64;2],
}
pub struct User {
    pub position: [f64;2],
    pub facing: f64,
    pub shot_cooldown: f64,
}
pub struct Tower {
    pub users: Vec<User>,
    pub spiders:Vec<Spider>,
    pub start_tick:f64,
    pub paused: bool,
}
impl Spider {
    pub fn drop(&mut self, dt:f64){
        self.cooldown -=dt;
    }
}
impl Tower {
    pub fn new() -> Tower {
        Tower {
            start_tick: 0.0,
            users:Vec::new(),
            spiders:Vec::new(),
            paused:false,
        }
    }
    pub fn move_u(&mut self, u: i32){
        let ref mut cur = self.users[u as usize];
        println!("{}",cur.facing);
        cur.position = match cur.facing {
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
    }
    pub fn tick(&mut self)  {
        //println!("ticking");
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
            if cur.cooldown <= 0.0  && cur.splitter{
                self.spiders[i].cooldown = 5.0;
                println!("Doubling");
                self.spiders.push(Spider {
                    position: [cur.position[0] + 15.0, cur.position[1]],
                    splitter:true,
                    cooldown:5.0,
                });
                self.spiders.push(Spider {
                    position: [cur.position[0] -15.0, cur.position[1]],
                    splitter:true,
                    cooldown:5.0,
                });

                self.spiders[i].splitter = false;

            } else {
                cur = &mut self.spiders[i];
                if u_p[0] < cur.position[0] {
                    cur.position[0] -= SPI_SPEED;
                } else if u_p[0] > cur.position[0] {
                    cur.position[0] += SPI_SPEED;
                }
                if u_p[1] < cur.position[1] {
                    cur.position[1] -= SPI_SPEED;
                } else if u_p[1] > cur.position[1] {
                    cur.position[1] += SPI_SPEED;
                }
            }
            if cur.position[0] < u_p[0] + USER[0] && cur.position[0] + SPIDER[0] > u_p[0] && u_p[1] + USER[1] > cur.position[1] && cur.position[1] + SPIDER[1] > u_p[1] {
                //println!("DEAD!");
            } else {
            }
        i+=1;
        }


    }
    pub fn reset (&mut self) {
        self.users = vec![User {position:[400.0,400.0],facing:1.0,shot_cooldown:0.0}, User{position:[500.0,500.0],facing:1.0,shot_cooldown:0.0}];
        self.spiders = Vec::new();
        let mut i = 0.0;
        while i< START {
            self.spiders.push(Spider {
                position: [ 100.0 * i, 100.0 * i],
                cooldown:0.0,
                splitter: i < START * INIT,
            });
            i += 1.0;
        }
        self.start_tick = 3.0;
        self.paused = true;
    }
}
