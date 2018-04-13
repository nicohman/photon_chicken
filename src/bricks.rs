const PLAYERS: i32 = 2;
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
        let mut pos = [((sx / 2.0) - (sx / 2.0) * ((id % 2.0)-2.0)), sy / 2.0 * (id % 2.0) * (id - (1.0 - (id % 3.0))) ];
        let mut bricks = Vec::new();
        while i < 18 {
            bricks.push(Brick {
                position: [(i as f64 / 6.0).floor(), (i % 6) as f64]
            });
        }
        let dir = id / 2.0;
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
    pub players : Vec<Player>,
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
            players:Vec::new(),
            shots:Vec::new(),
            walls:Vec::new(),
        }
    }
    pub fn reset (&mut self, sizes:[f64;2]){
        let ref sx = sizes[0];
        let ref sy = sizes[1];
        self.players = vec![Player::new([sx-60.0,sy-60.0], 0), Player::new([60.0;2], 1)];
        let mut i = 0;
        while i < PLAYERS {
            self.walls.push(Wall::new(i as f64, sx, sy))
        }

    }
}
