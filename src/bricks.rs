const PLAYERS: i32 = 4;
const SHOT_SPEED : f64 =  15.0;
const U_SPEED : f64 = 1.0;
pub struct Player {
    pub position: [f64;2],
    pub id : i32,
    pub facing: f64,
    pub dead :bool,
    pub shot_cooldown: f64,
    pub shot_time: f64,
}
pub struct Wall {
    pub position:[f64;2],
    pub dir: f64,
    pub bricks: Vec<Brick>,
}
fn collidesWith(pos1 : [f64;2], pos2:[f64;2], sz1:[f64;2], sz2:[f64;2])  -> bool{
    if pos1[0] + sz1[0] > pos2[0] && pos1[0] < pos2[0] + sz2[0] && pos2[1] + sz2[1] > pos1[1] && pos1[1] + sz1[1] > pos2[1] {
        true
    } else {
        false
    }
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
                position: [(i as f64 / 15.0).floor(), (i % 15) as f64],
                dead:false
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
    pub dead: bool,
}
pub struct Pickup {
    pub position:[f64;2],
}
#[derive(Clone, Copy)]
pub struct Shot {
    pub position: [f64;2],
    pub dir: f64,
    pub owner: i32,
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
            shot_time: 0.3,
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
                dir:cur.facing,
                owner: u,
            });

            cur.shot_cooldown = cur.shot_time;
        }
    }
    pub fn move_u(&mut self, u: i32, sizes:[f64;2]){
        let ref mut cur = self.users[u as usize];
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
            if ((to_move[0]+30.0 < (sizes[0] - (sizes[0] / 2.5))) && (to_move[0] > sizes[0] / 2.5)) || ((to_move[1] > sizes[1] /3.0 ) && (to_move[1]+60.0 < sizes[1] / 3.0 * 2.0)){
            cur.position = to_move;
            }
        } else {

        }
    }
    pub fn tick(&mut self, dt: f64) {
        use graphics::math::translate;
        use graphics::Transformed;
        let ref mut walls = self.walls;
        for wall in walls.iter_mut() {
            for brick in wall.bricks.iter_mut() {
                if !brick.dead {
                    let mut i = 0;
                    let mut isdead = false;
                    let mut bx =(brick.position[0] * 12.0 - 10.0);
                    let mut by = (brick.position[1] * 12.0 - 10.0);
                    let both = translate([0.0,0.0]).trans(wall.position[0], wall.position[1]).rot_rad(::std::f64::consts::PI * wall.dir).trans(bx, by);
                    bx = both[0][2];
                    by = both[1][2];
                    while i < self.shots.len() {
                        if collidesWith(self.shots[i].position, [bx,by], [15.0, 15.0], [10.0,10.0]) {
                            println!("Collide with brick!");
                            brick.dead = true;
                            self.shots.remove(i);
                            isdead = true;
                        }
                        i+=1;
                    }
                    if brick.position[1] > 15.0 {
                        brick.position[1] = 0.0;
                    } else {
                        brick.position[1] += 3.0 * dt;
                    }
                    if !isdead {
                        for mut user in &mut self.users {
                            if collidesWith(user.position, [bx,by], [30.0,60.0], [10.0,10.0]) {
                                println!("Collide with user!");
                                brick.dead = true;
                                user.dead = true;
                                isdead = true;
                            }
                        }
                    }
                }
            }
        }
        for mut user in &mut self.users {
            let mut i = 0;
            while i < self.shots.len() {
                if collidesWith(self.shots[i].position, user.position, [10.0,10.0],[30.0,60.0]) {
                    if self.shots[i].owner != user.id{
                    println!("Collide with shot!");
                    user.dead = true;
                    self.shots.remove(i);
                    }
                }
                i += 1;
            }
        }
        let mv = SHOT_SPEED * dt * 10.0;
        for mut cur in &mut self.shots {
            cur.position = match cur.dir {
                0.0 => [cur.position[0], cur.position[1] - mv],
                0.5 => [cur.position[0] + mv, cur.position[1] - mv],
                1.0 => [cur.position[0] + mv, cur.position[1]],
                1.5 => [cur.position[0] + mv, cur.position[1] + mv],
                2.0 => [cur.position[0], cur.position[1] + mv],
                2.5 => [cur.position[0] - mv, cur.position[1] + mv],
                3.0 => [cur.position[0] - mv, cur.position[1]],
                3.5 => [cur.position[0] - mv, cur.position[1] - mv],
                _ => cur.position
            };

        }
        for mut u in &mut self.users {
            u.shot_cooldown-=  dt;
        }
    }

    pub fn check_win(&mut self, sizes: [f64;2]) -> i32{
        let alive = self.users.iter().filter(|x| {!x.dead}).collect::<Vec<&Player>>();
        let num = alive.len();
        if num == 0 {
            println!("Tie achieved!");
            return -2;
        } else if num == 1 {
            println!("Victory!");
            return alive[0].id;
        }
        return -1;
    }
    pub fn reset (&mut self, sizes:[f64;2]){
        let ref sx = sizes[0];
        let ref sy = sizes[1];
        self.users = vec![Player::new([sx/2.0,60.0], 0), Player::new([sx - 60.0, sy/2.0], 1)];
        let mut i = 0;
        self.walls = Vec::new();
        while i < PLAYERS {
            self.walls.push(Wall::new(i as f64, sx, sy));
            i += 1;
        }

    }
}
