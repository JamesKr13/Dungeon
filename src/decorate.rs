use super::BSP_tree_map_generation::WORLD_SIZE;
use super::map::*;
extern crate rand;
use rand::Rng;
use macroquad::prelude::*;
const NUMBER_OF_DECORATIONS: i16 = 7;
#[derive(Clone,Debug,Copy)]
pub enum Objects {
    Skull, Chest, Empty, Bones, CandleStick, SmallCHest, Rock, SmallRock
}


pub struct PlaceObjects {
    pub object_map: Vec<Vec<Objects>>,
    pub map: Vec<Vec<AdvanceTileTypes>>,
}
impl PlaceObjects {
    pub fn tile_decorate(&mut self) {
        for row in 0..WORLD_SIZE.1 {
            for column in 0..WORLD_SIZE.0 {
                if matches!(self.map[row][column], AdvanceTileTypes::GenericFloor) {
                    self.place_object_decoration(row,column);
            }
        }
    }
}
    fn place_object_decoration(&mut self,row:usize,column: usize) {
        if row != 0 || column != 0 || row != WORLD_SIZE.1 || column != WORLD_SIZE.0  {
        let mut rng = rand::thread_rng();
        let random_decoration: i16 = rng.gen_range(0..=NUMBER_OF_DECORATIONS*20);
        // if self.object_map[row][column] {
        self.object_map[row][column] = match random_decoration {
            1 => Objects::Skull,
            2 => Objects::Chest,
            3 => Objects::Skull,
            4 => Objects::Skull,
            5 => Objects::Rock,
            6 => Objects::Bones,
            7 => Objects::Bones,
            8 => Objects::SmallCHest,
            9 => Objects::CandleStick,
            10 => Objects::SmallRock,
            _ => Objects::Empty,
        }
    }

    }
    pub fn draw_decor(&mut self, texture: Texture2D) {
        for row in 0..WORLD_SIZE.1 {
            for column in 0..WORLD_SIZE.0 {
                let object: Objects = self.object_map[row][column];
                if matches!(object, Objects::Chest) {
                    _draw_tile(row,column, texture, (2,8))
                    }
                else if matches!(object, Objects::Skull){
                    _draw_tile(row,column, texture, (7,7))
                    }
                else if matches!(object, Objects::SmallCHest){
                    _draw_tile(row,column, texture, (1,8))
                    }

                else if matches!(object, Objects::CandleStick){
                    draw_rectangle((column as f32)* CELL_SIZE, (row as f32)*CELL_SIZE, CELL_SIZE,CELL_SIZE, ORANGE)
                    }
                else if matches!(object, Objects::Bones){
                    _draw_tile(row,column, texture, (8,6))
                    }
                else if matches!(object, Objects::SmallRock){
                    _draw_tile(row,column, texture, (9,5))
                    }
                else if matches!(object, Objects::Rock){
                    _draw_tile(row,column, texture, (9,4))
                    }
            }
        }
    }
}
pub fn new_object_map(map: Vec<Vec<AdvanceTileTypes>>) -> PlaceObjects {
    PlaceObjects {
        object_map: vec![vec![Objects::Empty; WORLD_SIZE.0]; WORLD_SIZE.1],
        map: map,
    }
}
