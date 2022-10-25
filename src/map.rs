use super::bit_masking::BitMaskMap;
use super::bsp_tree_map_generation::{TileType, WORLD_SIZE};
pub const CELL_SIZE: f32 = 16.;
extern crate rand;
use macroquad::prelude::*;
use rand::Rng;
use std::fmt;
use enum_assoc::Assoc;

const NUMBER_OF_DECORATIONS: i16 = 7;
const FLOORCOOR: [(i16, i16); 12] = [
    (6, 0),
    (7, 0),
    (8, 0),
    (9, 0),
    (6, 1),
    (7, 1),
    (8, 1),
    (9, 1),
    (6, 2),
    (7, 2),
    (8, 2),
    (9, 2),
];
#[derive(Clone, Copy, Debug)]
pub enum AdvanceTileTypes {
    GenericFloor,
    Void,
    BLCorner,
    BRCorner,
    TLCorner,
    TRCorner,
    LEdge,
    REdge,
    TEdge,
    BEdge,
    OTLCorner,
    OTRCorner,
    OBLCorner,
    OBRCorner,
    Skull,
    Chest,
    Bones,
    CandleStick,
    SmallCHest,
    Rock,
    SmallRock,
}
impl fmt::Display for AdvanceTileTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdvanceTileTypes::GenericFloor => write!(f, "Floor"),
            AdvanceTileTypes::Void => write!(f, "Nothing"),
            AdvanceTileTypes::Skull => write!(f, "Skull"),
            AdvanceTileTypes::Chest => write!(f, "Chest"),
            AdvanceTileTypes::Bones => write!(f, "Bones"),
            AdvanceTileTypes::SmallCHest => write!(f, "Small Chest"),
            AdvanceTileTypes::CandleStick => write!(f, "Candle"),
            AdvanceTileTypes::Rock => write!(f, "Rock"),
            AdvanceTileTypes::SmallRock => write!(f, "Small Rock"),
            _ => write!(f, "Wall"),
        }
    }
}
#[derive(Clone, Debug)]
pub struct MapLayout {
    exit: (i32, i32),
    pub other_map: Vec<Vec<TileType>>,
    pub tile_placement: Vec<Vec<AdvanceTileTypes>>,
    frame: usize,
    // x: Vec<[(i16,i16);2]>
}

impl Default for MapLayout {
    fn default() -> Self {
        let mut map = BitMaskMap::default();
        map.tile_map();
        Self {
            exit: map.exit,
            other_map: map.map,
            tile_placement: map.tile_map,
            frame: 0,
        }
    }
}
impl MapLayout {
    pub fn frame_up(&mut self) {
        self.frame = match self.frame {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => 1,
        };
    }
    pub fn draw_map(&mut self, texture: Texture2D,torch_texture:[Texture2D;4]) {
        for row in 0..WORLD_SIZE.1 {
            for column in 0..WORLD_SIZE.0 {
                match &self.tile_placement[row][column] {
                    AdvanceTileTypes::GenericFloor => _draw_floor(row, column, texture),
                    AdvanceTileTypes::Void => (),
                    AdvanceTileTypes::BLCorner => _draw_tile(row, column, texture, (0, 4)),
                    AdvanceTileTypes::BRCorner => _draw_tile(row, column, texture, (5, 4)),
                    AdvanceTileTypes::TLCorner => _draw_tile(row, column, texture, (0, 0)),
                    AdvanceTileTypes::TRCorner => _draw_tile(row, column, texture, (0, 5)),
                    AdvanceTileTypes::LEdge => _draw_tile(row, column, texture, (0, 0)),
                    AdvanceTileTypes::REdge => _draw_tile(row, column, texture, (5, 0)),
                    AdvanceTileTypes::TEdge => _draw_tile(row, column, texture, (3, 0)),
                    AdvanceTileTypes::BEdge => _draw_tile(row, column, texture, (3, 4)),
                    AdvanceTileTypes::OBRCorner => _draw_tile(row, column, texture, (0, 6)),
                    AdvanceTileTypes::OBLCorner => _draw_tile(row, column, texture, (0, 6)),
                    AdvanceTileTypes::OTLCorner => _draw_tile(row, column, texture, (5, 5)),
                    AdvanceTileTypes::OTRCorner => _draw_tile(row, column, texture, (0, 5)),
                    AdvanceTileTypes::Chest => draw_decor(row, column, texture, (2, 8)),
                    AdvanceTileTypes::SmallCHest => draw_decor(row, column, texture, (1, 8)),
                    AdvanceTileTypes::Bones => draw_decor(row, column, texture, (8, 6)),
                    AdvanceTileTypes::Skull => draw_decor(row, column, texture, (7, 7)),
                    AdvanceTileTypes::CandleStick => draw_animated_decor(row, column, texture,torch_texture[self.frame]),
                    AdvanceTileTypes::Rock => draw_decor(row, column, texture, (9, 4)),
                    AdvanceTileTypes::SmallRock => draw_decor(row, column, texture, (9, 5)),
                };
            }
        }
        self.draw_exit(texture);
    }
    fn _draw_floor(&mut self, row: usize, column: usize, texture: Texture2D) {
        let index = (f32::from((row as i16 | (CELL_SIZE as i16 - 1)) + 1) / CELL_SIZE) as usize;
        self._draw_tile(row, column, texture, FLOORCOOR[index]);
    }
    fn draw_exit(&mut self, texture: Texture2D) {
        self._draw_tile(
            (self.exit.1) as usize,
            (self.exit.0) as usize,
            texture,
            (9, 3),
        )
    }
    fn _draw_tile(&mut self, row: usize, column: usize, texture: Texture2D, pos: (i16, i16)) {
        draw_texture_ex(
            texture,
            column as f32 * CELL_SIZE,
            row as f32 * CELL_SIZE,
            WHITE,
            DrawTextureParams {
                source: Some(Rect {
                    x: f32::from(pos.0) * CELL_SIZE,
                    y: f32::from(pos.1) * CELL_SIZE,
                    w: CELL_SIZE,
                    h: CELL_SIZE,
                }),
                ..Default::default()
            },
        );
    }

    pub fn tile_decorate(&mut self) {
        for row in 0..WORLD_SIZE.1 {
            for column in 0..WORLD_SIZE.0 {
                if matches!(
                    self.tile_placement[row][column],
                    AdvanceTileTypes::GenericFloor
                ) {
                    self.place_object_decoration(row, column);
                }
            }
        }
    }
    fn place_object_decoration(&mut self, row: usize, column: usize) {
        let mut rng = rand::thread_rng();
        let random_decoration: i16 = rng.gen_range(0..=NUMBER_OF_DECORATIONS * 20);
        self.tile_placement[row][column] = match random_decoration {
            1 => AdvanceTileTypes::Skull,
            2 => AdvanceTileTypes::Chest,
            3 => AdvanceTileTypes::Skull,
            5 => AdvanceTileTypes::Rock,
            6 => AdvanceTileTypes::Bones,
            7 => AdvanceTileTypes::Bones,
            8 => AdvanceTileTypes::SmallCHest,
            9 => AdvanceTileTypes::CandleStick,
            10 => AdvanceTileTypes::SmallRock,
            _ => AdvanceTileTypes::GenericFloor,
        }
    }
}
pub fn _draw_tile(row: usize, column: usize, texture: Texture2D, pos: (i16, i16)) {
    draw_texture_ex(
        texture,
        column as f32 * CELL_SIZE,
        row as f32 * CELL_SIZE,
        WHITE,
        DrawTextureParams {
            source: Some(Rect {
                x: f32::from(pos.0) * CELL_SIZE,
                y: f32::from(pos.1) * CELL_SIZE,
                w: CELL_SIZE,
                h: CELL_SIZE,
            }),
            ..Default::default()
        },
    );
}
pub fn draw_decor(row: usize, column: usize, texture: Texture2D, pos: (i16, i16)) {
    _draw_floor(row, column, texture);
    draw_texture_ex(
        texture,
        column as f32 * CELL_SIZE,
        row as f32 * CELL_SIZE,
        WHITE,
        DrawTextureParams {
            source: Some(Rect {
                x: f32::from(pos.0) * CELL_SIZE,
                y: f32::from(pos.1) * CELL_SIZE,
                w: CELL_SIZE,
                h: CELL_SIZE,
            }),
            ..Default::default()
        },
    );
}
pub fn draw_animated_decor(row: usize, column: usize, floor_texture:Texture2D, texture: Texture2D) {
    _draw_floor(row, column, floor_texture);
    draw_texture_ex(
        texture,
        column as f32 * CELL_SIZE,
        row as f32 * CELL_SIZE,
        WHITE,
        DrawTextureParams {
            source: Some(Rect {
                x: 0.,
                y: 0.,
                w: CELL_SIZE,
                h: CELL_SIZE,
            }),
            ..Default::default()
        },
    );
}
fn _draw_floor(row: usize, column: usize, texture: Texture2D) {
    let index = (f32::from((row as i16 | (CELL_SIZE as i16 - 1)) + 1) / CELL_SIZE) as usize;
    _draw_tile(row, column, texture, FLOORCOOR[index]);
}
