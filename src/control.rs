use super::map::AdvanceTileTypes;
use macroquad::prelude::*;
pub const SPEED: f32 = 16.;
use enum_assoc::Assoc;
// use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
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
    pub fn vector_movement(&mut self) -> (i16, i16) {
        // println!("{} {} {} {}", self.up, self.down, self.left, self.right);
        (
            match (self.up, self.down) {
                (false, true) => -1,
                (true, false) => 1,
                _ => 0,
            },
            match (self.right, self.left) {
                (false, true) => -1,
                (true, false) => 1,
                _ => 0,
            },
        )
    }
    #[must_use]
    pub fn movement_character(
        self,
        current_map: &Vec<Vec<AdvanceTileTypes>>,
        current_pos: (usize, usize),
    ) -> bool {
        match current_map[current_pos.1][current_pos.0] {
            AdvanceTileTypes::GenericFloor => true,
            AdvanceTileTypes::SmallCHest => true,
            AdvanceTileTypes::Bones => true,
            AdvanceTileTypes::Skull => true,
            AdvanceTileTypes::CandleStick => true,
            AdvanceTileTypes::Chest => true,
            _ => false,
        }
    }
}

pub struct ScreenMovement {
    mouse_position: (f32, f32),
}
impl Default for ScreenMovement {
    fn default() -> Self {
        Self {
            mouse_position: mouse_position(),
        }
    }
}
impl ScreenMovement {
    #[must_use]
    pub fn mouse_follow(self) -> (f32, f32) {
        let pre_cast = (
            i8::from(self.mouse_position.0 >= screen_width() - 125.),
            i8::from(self.mouse_position.1 >= screen_height() - 125.),
            i8::from(125. >= self.mouse_position.0),
            i8::from(125. >= self.mouse_position.1),
        );
        (
            -4. * f32::from(pre_cast.0) + 4. * f32::from(pre_cast.2),
            4. * f32::from(pre_cast.1) - 4. * f32::from(pre_cast.3),
        )
    }
    pub fn mouse_pos() {
        println!("{:#?}", mouse_position());
    }
}
