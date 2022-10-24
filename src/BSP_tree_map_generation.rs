extern crate rand;
use rand::Rng;
use std::collections::HashMap;

pub const WORLD_SIZE: (usize,usize) = (40,40);
pub const CELL_SIZE: f32 = 16.;
const MAX_LEAF_SIZE: i16 = 20;
const MIN_LEAF_SIZE: i16 = 5;
#[derive(Clone,Debug,Copy)]
pub enum TileType{
    Wall, Floor
}

#[derive(Clone,Copy,Debug,Default)]
pub struct Rectangle {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16
}
impl Rectangle {
    pub fn center(self) -> (i16,i16) {
        let center_x = (self.x1 + self.x2)/2;
        let center_y = (self.y1 + self.y2)/2;
        return (center_x,center_y)
    }
}
#[derive(Clone,Debug,Default)]
pub struct Leaf {
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    room: Rectangle,
}
pub fn new_leaf(x: i16, y:i16, width: i16, height: i16) -> Leaf {
    Leaf {
        x: x,
        y: y,
        width: width,
        height: height,
        ..Default::default()
    }
}
impl Leaf {
    pub fn inital(&mut self,x: i16, y:i16, width: i16, height: i16) -> Self {
        Self {
            x: x,
            y: y,
            width: width,
            height: height,
            ..Default::default()

        }
    }
    fn split(&mut self) -> Option<(Leaf,Leaf)> {
            let mut rng = rand::thread_rng();
            let random: i8 = rng.gen_range(0..=1); 
            let mut horizon_split: bool = random == 0;
            if self.width > self.height && (self.width/self.width) as f32 >= 1.25 {
                horizon_split = false;
            } else if self.height > self.width && (self.height/self.width) as f32 >= 1.25 {
                horizon_split = true;
            }

            let max: i16 = match horizon_split {
                true => self.height - MIN_LEAF_SIZE,
                false => self.width - MIN_LEAF_SIZE
            };
            if max <= MAX_LEAF_SIZE {
                return None;
            }
            let split: i16 = rng.gen_range(MIN_LEAF_SIZE..max);
            return match horizon_split {
               true => Some((self.inital(self.x,self.y,self.width,split),self.inital(self.x, self.y + split,self.width, self.height - split))),
               false => Some((self.inital(self.x,self.y,split,self.height), self.inital(self.x + split, self.y,self.width -split, self.height)))
            }
        }
    fn create_rooms(&mut self) {
        let mut rng = rand::thread_rng();
        if self.width >=6 && self.height >= 6 {
        let roomsize: [i16; 2] = [rng.gen_range(5..self.width), rng.gen_range(5..self.height)];
        let room_pos: [i16; 2] = [rng.gen_range(0..self.width-roomsize[0]), rng.gen_range(0..self.height-roomsize[1])];
        self.room = Rectangle { 
            x1: self.x + room_pos[0],
            y1: self.y + room_pos[1], 
            x2: roomsize[0],
            y2: roomsize[1],
        };
    }
    }
}
#[derive(Clone,Debug)]
pub struct BSPTree {
    pub leafs: Vec<Leaf>,
    pub root: Leaf,
    pub level: Vec<Vec<TileType>>,
    pub x: Vec<[(i16,i16);2]>
}
impl Default for BSPTree {
    fn default() -> Self {
        Self {
            leafs: Vec::new(), root: new_leaf(0,0,WORLD_SIZE.0 as i16,WORLD_SIZE.1 as i16), level: vec![vec![TileType::Wall; WORLD_SIZE.0]; WORLD_SIZE.1], x: Vec::new()
        }
    }
}
impl BSPTree {
    pub fn generate_level(&mut self) {
        self.root = Leaf {
            x: 0,
            y: 0,
            width: WORLD_SIZE.0 as i16,
            height: WORLD_SIZE.1 as i16,
            ..Default::default()
        };
        self.leafs.push(self.root.clone());
        let mut split = true;
        while split {
            split = false;
            for index in 0..self.leafs.len() {
                let old_len = self.leafs.len();
                let mut l = self.leafs[index].clone();
                if l.width > MIN_LEAF_SIZE || l.height > MIN_LEAF_SIZE {
                    let x = l.split();
                    if !x.is_none() {
                        let mut children = x.unwrap();
                        children.0.create_rooms();
                        children.1.create_rooms();
                        self.leafs.push(children.0);
                        self.leafs.push(children.1);
                        if self.leafs.len()-2 == old_len {
                            self.leafs.remove(index);
                        }
                        split = true
                    }
                }
                }
                }
            self.create_binary_map();
            self.create_hallway();
            }
    fn create_binary_map(&mut self) {
        for leaf in &self.leafs {
            for y in 0..leaf.room.y2 {
                for x in 0..leaf.room.x2 {
                    self.level[(x+leaf.room.x1) as usize][(y+leaf.room.y1) as usize] = TileType::Floor;

                }
            }
        }
    }
    fn create_hallway(&mut self) {
        let mut linked_rooms: HashMap<(i16,i16),(i16,i16)> = HashMap::new();
        let mut closes_room: (i16,i16) = (0,0);
        for room_index in 0..self.leafs.len() {
            let main_room = (self.leafs[room_index].room.x1+(self.leafs[room_index].room.x2)/2,self.leafs[room_index].room.y1+(self.leafs[room_index].room.y2)/2);
            let mut closes_distance: f32= f32::INFINITY;
            for other_room_index in 0..self.leafs.len() {
                if other_room_index != room_index {
                let compare_room = (self.leafs[other_room_index].room.x1+(self.leafs[other_room_index].room.x2)/2,self.leafs[other_room_index].room.y1+(self.leafs[other_room_index].room.y2)/2);
                let distance = pythogoras(main_room.0-compare_room.0,main_room.1-compare_room.1);
                if distance <= closes_distance && main_room !=match linked_rooms.get(&compare_room) {
                        Some(_) => linked_rooms[&compare_room],
                        None => (WORLD_SIZE.0 as i16,WORLD_SIZE.1 as i16),
                    } {
                    closes_room= compare_room;
                    closes_distance = distance;
                } 
            }
        }
        linked_rooms.insert(main_room,closes_room);
        self.create_passage(main_room,closes_room);
        }
    }
    // Problem with gen is in remains here
    fn create_passage(&mut self, point: (i16,i16), second_point: (i16,i16)) {
        let mut destination_point = *vec!(point.0,second_point.0).iter().min().unwrap();
        for x_distance in *vec!(point.1,second_point.1).iter().min().unwrap()..=*vec!(point.1,second_point.1).iter().max().unwrap()+2 {
            self.level[destination_point as usize][x_distance as usize] = TileType::Floor;
            if destination_point >= 1 {
            self.level[(destination_point-1) as usize][x_distance as usize] = TileType::Floor;
            }
            if destination_point <= WORLD_SIZE.1 as i16 -1 {
                self.level[(destination_point+1) as usize][x_distance as usize] = TileType::Floor;
            }
            if destination_point >= 2 {
                self.level[(destination_point-2) as usize][x_distance as usize] = TileType::Floor;
            } else if destination_point <= WORLD_SIZE.1 as i16 -2 {
                self.level[(destination_point+2) as usize][x_distance as usize] = TileType::Floor;
            }
        }
        destination_point = *vec!(point.1,second_point.1).iter().max().unwrap();
        for y_distance in *vec!(point.0,second_point.0).iter().min().unwrap()..=*vec!(point.0,second_point.0).iter().max().unwrap()+2 {
            self.level[y_distance as usize][destination_point as usize] = TileType::Floor;
            if destination_point <= WORLD_SIZE.0 as i16-1 {
                self.level[y_distance as usize][(destination_point+1) as usize] = TileType::Floor;
            }
            if destination_point >= 1 {
            self.level[y_distance as usize][(destination_point-1) as usize] = TileType::Floor;
            }
            if destination_point <= WORLD_SIZE.0 as i16 -2 {
                self.level[y_distance as usize][(destination_point+2) as usize] = TileType::Floor;
            } else if destination_point >= 2 {
                self.level[y_distance as usize][(destination_point-2) as usize] = TileType::Floor;
            }
        }

    }
}
fn pythogoras(opp: i16,adj: i16) -> f32 {
    (opp.pow(2)+adj.pow(2)) as f32
}
pub fn abs(value: i16) -> i16 {
    let new_value = (value.pow(2) as f32).sqrt() as i16;
    new_value
}