// use super::BSPMapGeneration::*;
use super::bsp_tree_map_generation::{BSPTree, TileType, WORLD_SIZE};
use super::map::AdvanceTileTypes;

const SEARCH: (i16, i16, i16, i16, i16, i16, i16, i16) = (1, 2, 4, 8, 16, 32, 64, 128);
#[derive(Clone)]
pub struct BitMaskMap {
    pub exit: (i32, i32),
    pub map: Vec<Vec<TileType>>,
    pub tile_map: Vec<Vec<AdvanceTileTypes>>,
    pub x: Vec<[(i16, i16); 2]>,
}
impl Default for BitMaskMap {
    fn default() -> Self {
        let mut map = BSPTree::default();
        println!("BitMask Default",);
        map.generate_level();
        Self {
            exit: (0, 0),
            map: map.level,
            tile_map: vec![vec![AdvanceTileTypes::Void; WORLD_SIZE.0]; WORLD_SIZE.1],
            x: map.x,
        }
    }
}
impl BitMaskMap {
    fn match_bit(&mut self, tile: &TileType) -> i16 {
        match tile {
            TileType::Wall => 0,
            TileType::Floor => 1,
        }
    }
    fn sum(&mut self, value: [i16; 8]) -> i16 {
        let mut total = 0;
        for num in value {
            total += num;
        }
        total
    }
    fn directional_compare(&mut self, row: usize, column: usize) -> AdvanceTileTypes {
        // Note to Self: Dude what the fuck, 8 bit masking, the fuck man, just use 4 bit masking to check and detect corners
        let mut total: [i16; 8] = [0; 8];
        if row != 0 {
            total[1] = SEARCH.1 * self.match_bit(&self.map[row - 1][column].clone());
            if column != 0 {
                total[0] = SEARCH.0 * self.match_bit(&self.map[row - 1][column - 1].clone());
            }
            if column <= WORLD_SIZE.1 - 2 {
                total[2] = SEARCH.2 * self.match_bit(&self.map[row - 1][column + 1].clone());
            }
        }
        if row <= WORLD_SIZE.0 - 2 {
            total[6] = SEARCH.6 * self.match_bit(&self.map[row + 1][column].clone());
            if column != 0 {
                total[5] = SEARCH.5 * self.match_bit(&self.map[row + 1][column - 1].clone());
            }
            if column <= WORLD_SIZE.1 - 2 {
                total[7] = SEARCH.7 * self.match_bit(&self.map[row + 1][column + 1].clone());
            }
        }
        if column != 0 {
            total[3] = SEARCH.3 * self.match_bit(&self.map[row][column - 1].clone());
        }
        if column <= WORLD_SIZE.1 - 2 {
            total[4] = SEARCH.4 * self.match_bit(&self.map[row][column + 1].clone());
        }
        let inital = self.match_bit(&self.map[row][column].clone());
        match self.sum(total) * inital {
            2 => AdvanceTileTypes::GenericFloor,
            8 => AdvanceTileTypes::TEdge,
            10 => AdvanceTileTypes::TEdge,
            11 => AdvanceTileTypes::BRCorner,
            15 => AdvanceTileTypes::BRCorner,
            16 => AdvanceTileTypes::TLCorner,
            18 => AdvanceTileTypes::TEdge,
            22 => AdvanceTileTypes::BLCorner,
            23 => AdvanceTileTypes::BLCorner,
            24 => AdvanceTileTypes::TLCorner,
            26 => AdvanceTileTypes::TRCorner,
            27 => AdvanceTileTypes::TRCorner,
            30 => AdvanceTileTypes::TRCorner,
            31 => AdvanceTileTypes::BEdge,
            43 => AdvanceTileTypes::BRCorner,
            47 => AdvanceTileTypes::BRCorner,
            62 => AdvanceTileTypes::LEdge,
            63 => AdvanceTileTypes::BEdge,
            64 => AdvanceTileTypes::TRCorner,
            66 => AdvanceTileTypes::TEdge,
            72 => AdvanceTileTypes::TEdge,
            75 => AdvanceTileTypes::TEdge,
            80 => AdvanceTileTypes::TEdge,
            82 => AdvanceTileTypes::TEdge,
            86 => AdvanceTileTypes::TEdge,
            88 => AdvanceTileTypes::LEdge,
            90 => AdvanceTileTypes::BLCorner,
            91 => AdvanceTileTypes::LEdge,
            94 => AdvanceTileTypes::TLCorner,
            95 => AdvanceTileTypes::LEdge,
            104 => AdvanceTileTypes::REdge,
            105 => AdvanceTileTypes::REdge,
            106 => AdvanceTileTypes::TEdge,
            107 => AdvanceTileTypes::REdge,
            108 => AdvanceTileTypes::REdge,
            111 => AdvanceTileTypes::REdge,
            118 => AdvanceTileTypes::LEdge,
            120 => AdvanceTileTypes::REdge,
            122 => AdvanceTileTypes::GenericFloor, //?
            123 => AdvanceTileTypes::GenericFloor,
            124 => AdvanceTileTypes::REdge,
            126 => AdvanceTileTypes::BLCorner,
            127 => AdvanceTileTypes::OTRCorner,
            143 => AdvanceTileTypes::BRCorner,
            145 => AdvanceTileTypes::TLCorner,
            150 => AdvanceTileTypes::BLCorner, //Ledge
            151 => AdvanceTileTypes::BLCorner,
            159 => AdvanceTileTypes::BEdge, //is a corner
            191 => AdvanceTileTypes::BEdge,
            201 => AdvanceTileTypes::LEdge,
            203 => AdvanceTileTypes::BRCorner,
            208 => AdvanceTileTypes::TLCorner, // T l Outside corner
            210 => AdvanceTileTypes::REdge,
            211 => AdvanceTileTypes::LEdge,
            214 => AdvanceTileTypes::LEdge,
            212 => AdvanceTileTypes::LEdge,
            215 => AdvanceTileTypes::LEdge,
            216 => AdvanceTileTypes::TEdge,
            218 => AdvanceTileTypes::TEdge,
            219 => AdvanceTileTypes::TEdge,
            222 => AdvanceTileTypes::TEdge,
            223 => AdvanceTileTypes::OTLCorner, //Corner Piece
            232 => AdvanceTileTypes::REdge,
            233 => AdvanceTileTypes::REdge,
            235 => AdvanceTileTypes::REdge,
            239 => AdvanceTileTypes::REdge,
            240 => AdvanceTileTypes::LEdge,
            242 => AdvanceTileTypes::LEdge,
            243 => AdvanceTileTypes::LEdge,
            244 => AdvanceTileTypes::TLCorner,
            246 => AdvanceTileTypes::LEdge,
            247 => AdvanceTileTypes::LEdge,
            248 => AdvanceTileTypes::TEdge,
            249 => AdvanceTileTypes::TEdge,
            250 => AdvanceTileTypes::TEdge,
            251 => AdvanceTileTypes::TEdge,
            252 => AdvanceTileTypes::TEdge,
            253 => AdvanceTileTypes::TEdge,
            254 => AdvanceTileTypes::TEdge,
            255 => AdvanceTileTypes::GenericFloor,
            0 => AdvanceTileTypes::Void,
            _ => AdvanceTileTypes::Void,
        }
    }
    pub fn tile_map(&mut self) {
        for row in 0..WORLD_SIZE.1 {
            for column in 0..WORLD_SIZE.0 {
                self.tile_map[row][column] = self.directional_compare(row, column);
            }
        }
    }
}
