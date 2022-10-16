extern crate rand;
use ::rand::Rng;
pub const WORLD_SIZE: (usize,usize) = (25,25);
const MAX_ROOMS: i32 = 15;
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 5;
#[derive(Clone,Debug,Copy)]
pub enum TileType{
    Wall, Floor
}
#[derive(Clone,Copy)]
pub struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}
impl Rectangle {
    pub fn center(self) -> (i32,i32) {
        let center_x = (self.x1 + self.x2)/2;
        let center_y = (self.y1 + self.y2)/2;
        return (center_x,center_y)
    }
    fn intersect(self, other: &Rectangle) -> bool{
        return self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
    fn new(x:i32,y:i32,w:i32, h:i32) -> Self{
        Self {
            x1: x,
            y1: y,
            x2: w+x,
            y2: h+y,
        }
    }
}
#[derive(Clone)]
pub struct TunnelingAlgorithm {
    pub level: Vec<Vec<TileType>>,
    nums_room: i32,
    pub rooms: Vec<Rectangle>

}
impl Default for TunnelingAlgorithm {
    fn default() -> Self {
        Self {
            level: vec![vec![TileType::Wall; WORLD_SIZE.0]; WORLD_SIZE.1],
            nums_room: 0,
            rooms: Vec::new()
        }
    }
}
impl TunnelingAlgorithm {
    pub fn generate_level(&mut self) {
        let mut rooms = Vec::new();
        let mut rng = rand::thread_rng();
        for _range in 0..MAX_ROOMS {
            let mut w: i32 = rng.gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
            let mut h: i32 = rng.gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);
            while w <= 2 {
                w = rng.gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);

            }
            while h <= 2 {
                h = rng.gen_range(ROOM_MIN_SIZE..=ROOM_MAX_SIZE);

            }
            let x = rng.gen_range(0..WORLD_SIZE.0 as i32-w -1);
            let y = rng.gen_range(0..WORLD_SIZE.1 as i32-w -1);

            let new_room = Rectangle::new(x,y,w,h);
            let mut failed = true;
            for other_room in &rooms {
                if self.intersection(new_room,other_room){
                    failed = true;
                } else {
                    failed = false
                }
            }
            if !failed {
                self.create_room(new_room);
                let (new_x,new_y) = new_room.center();
                if &self.nums_room != &0 {
                    let (prev_x,prev_y) = rooms[(self.nums_room -1) as usize].center();
                    let value: i32 = rng.gen_range(0..=1);
                    if value == 1 {
                        self.create_hor_tunnel(prev_x,new_x, prev_y);
                        self.create_ver_tunnel(prev_y,new_y,prev_x)
                    } else {
                            self.create_hor_tunnel(prev_x,new_x, new_y);
                            self.create_ver_tunnel(prev_y,new_y,new_x)
                    }
                }
            }
            rooms.push(new_room);
            self.nums_room += 1;

        }
        self.rooms = rooms
    }
    fn create_hor_tunnel(&mut self, x1:i32,x2:i32,y:i32) {
        for x in *vec!(x1,x2).iter().min().unwrap()-1..=y {
            self.level[x as usize][y as usize] = TileType::Floor;
            self.level[x as usize][(y+1) as usize] = TileType::Floor;
            self.level[x as usize][(y-1) as usize] = TileType::Floor;
            self.level[x as usize][(y+2) as usize] = TileType::Floor;


        }
    }
    fn create_ver_tunnel(&mut self, y1:i32,y2:i32,x:i32) {
        for y in x-1..=*vec!(y1,y2).iter().min().unwrap() {
            self.level[x as usize][y as usize] = TileType::Floor;
            self.level[(x-1) as usize][y as usize] = TileType::Floor;
            self.level[(x-1) as usize][y as usize] = TileType::Floor;
            
        }
    }
    fn create_room(&mut self, room: Rectangle){
        for x_perimeter in room.x1..room.x2 {
            for y_perimeter in room.y1..room.y2 {
                self.level[(x_perimeter) as usize][(y_perimeter) as usize] = TileType::Floor;
            }
        }
    }
    fn intersection(&self,new_room: Rectangle, other_room: &Rectangle) -> bool {
        if new_room.intersect(other_room) {
            true
        } else {
            false
        }
    }
}