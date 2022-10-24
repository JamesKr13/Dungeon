use macroquad::prelude::*;
use super::map::AdvanceTileTypes;
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
    OptionInfo,
    Question,
    Dead,
    StartScreen,
}
// pub struct ManageStates {
//     pub main_state: States,
//     pub question: States,
//     pub sub_state_one: States,
//     pub sub_state_two: States,
// }
// impl ManageStates {
//     fn inventory_change_state(&mut self) {
//         self.sub_state_one = match self.sub_state_one {
//             States::Inventory => States::Play,
//             _ => States::Inventory,
//         }
//     }
//     fn storage_change_state(&mut self) {
//         self.sub_state_two = match self.sub_state_two {
//             States::Storage => States::Play,
//             _ => States::Storage,
//         }
//     }
//     fn match_substate_calls(&mut self) {
//         if matches!(self.question,States::Question) {
//             match get_char_pressed {
//                 "i" => self.inventory_change_state(),
//                 "e" => 
//             }
//         }
//     }
// }
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
