use macroquad::prelude::*;
use super::Map::AdvanceTileTypes;
pub const SPEED: f32 = 16.;
// use std::collections::HashMap;

#[derive(Copy,Clone,Debug)]
pub enum States {
    Inventory,
    Pause,
    Menu,
    Storage,
    Empty,
    Play,
    OptionInfo
}

pub struct Movement {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Movement {
    pub fn vector_movement(&mut self) -> (i16,i16) {
        // println!("{} {} {} {}", self.up, self.down, self.left, self.right);
        return (match (self.up,self.down) {
            (false,true) => -1,
            (true,false) => 1,
            _ => 0
        }, match (self.right,self.left) {
            (false,true) => -1,
            (true,false) => 1,
            _ => 0
        })
    }
    pub fn movement_character(self,current_map: &Vec<Vec<AdvanceTileTypes>>, current_pos: (usize,usize)) -> bool {
        match current_map[current_pos.1][current_pos.0] {
                    AdvanceTileTypes::GenericFloor => true,
                    AdvanceTileTypes::SmallCHest => true,
                    AdvanceTileTypes::Bones => true,
                    AdvanceTileTypes::Skull => true,
                    AdvanceTileTypes::CandleStick => true,
                    AdvanceTileTypes::Chest => true,
                    _ => false
                }
    }
}
struct Player {

}

pub struct ScreenMovement {
    mouse_position: (f32,f32)
}
impl Default for ScreenMovement {
    fn default() -> Self {
        Self {
            mouse_position: mouse_position()
        }
    }
}
impl ScreenMovement {
    pub fn mouse_follow(self) -> (f32,f32) {
        let pre_cast = ((self.mouse_position.0 >= screen_width()-125.) as i8, (self.mouse_position.1 >= screen_height()-125.) as i8, (125. >= self.mouse_position.0) as i8, (125. >= self.mouse_position.1) as i8);
        return (-4.*pre_cast.0 as f32 +4.*pre_cast.2 as f32, 4.*pre_cast.1 as f32 -4.*pre_cast.3 as f32)
    }
    pub fn mouse_pos() {
        println!("{:#?}", mouse_position());
    }
}
