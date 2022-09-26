use macroquad::prelude::*;

pub const SPEED: f32 = 2.;

pub struct Movement {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Movement {
    pub fn vector_movement(&mut self) -> (i16,i16) {
        println!("{} {} {} {}", self.up, self.down, self.left, self.right);
        return (match (self.up,self.down) {
            (false,true) => 1,
            (true,false) => -1,
            _ => 0
        }, match (self.right,self.left) {
            (false,true) => 1,
            (true,false) => -1,
            _ => 0
        })
    }
}