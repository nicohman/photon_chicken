const PLAYERS: i32 = 4;
const SHOT_SPEED : f64 =  1.5;
const U_SPEED : f64 = 1.0;
pub struct Player {
    pub position: [f64;2],
    pub id : i32,
    pub facing: f64,
    pub dead :bool,
    pub shot_cooldown: f64,
}
pub struct Wall {
    pub position:[f64;2],
    pub dir: f64,
    pub bricks: Vec<Brick>,
}
impl Wall {
    pub fn new(id: f64, sx: &f64, sy: &f64) -> Wall {
        let mut i = 0;
        let mut pos = /*[((sx / 2.0) - (sx / 2.0) * ((id % 2.0)-2.0)), sy / 2.0 * (id % 2.0) * (id - (1.0 - (id % 3.0))) ];*/ match id {
            0.0 => [*sx / 2.0 +10.0 * 15.0 /2.0, sy / 4.0],
            1.0 => [*sx * 0.75, *sy / 2.0 + 10.0 * 15.0 / 2.0],
            2.0 => [*sx / 2.0 - 10.0 * 15.0 / 2.0, *sy * 0.75],
            3.0 => [sx / 4.0, *sy / 2.0 - 10.0 * 15.0 / 2.0],
            4.0 => [0.0,0.0],
            _ => [0.0,0.0]
        };
        let mut bricks = Vec::new();
        while i < 60 {
            bricks.push(Brick {
                position: [(i as f64 / 15.0).floor(), (i % 15) as f64]
            });
            i+= 1;
        }
        let dir = id / 2.0 + 0.5;
        Wall {
            dir:dir,
            bricks:bricks,
            position:pos
        }
    }
}
pub struct Brick {
    pub position:[f64;2],
}
pub struct Pickup {
    pub position:[f64;2],
}
#[derive(Clone, Copy)]
pub struct Shot {
    pub position: [f64;2],
    pub dir: f64
}
pub struct Bricks {
    pub paused: bool,
    pub start_tick:f64,
    pub tick_time: f64,
    pub shots: Vec<Shot>,
    pub users : Vec<Player>,
    pub walls: Vec<Wall>
}
impl Player {
    pub fn new (pos: [f64;2], id:i32) -> Player {
        Player {
            position:pos,
            facing:1.0,
            id:id,
            shot_cooldown:0.0,
            dead:false,
        }
    }
}
impl Bricks {
    pub fn new () -> Bricks {
        Bricks {
            paused:false,
            start_tick: 0.0,
            tick_time:0.0,
            users:Vec::new(),
            shots:Vec::new(),
            walls:Vec::new(),
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
    pub fn tick(&mut self, dt: f64) {
        let ref mut walls = self.walls;
        for wall in walls.iter_mut() {
            for brick in wall.bricks.iter_mut() {
                if brick.position[1] > 15.0 {
                    brick.position[1] = 0.0;
                } else {
                    brick.position[1] += 3.0 * dt;
                }
            }
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
    pub fn reset (&mut self, sizes:[f64;2]){
        let ref sx = sizes[0];
        let ref sy = sizes[1];
        self.users = vec![Player::new([sx-60.0,sy-60.0], 0), Player::new([60.0;2], 1)];
        let mut i = 0;
        while i < PLAYERS {
            self.walls.push(Wall::new(i as f64, sx, sy));
            i += 1;
        }

    }
}
