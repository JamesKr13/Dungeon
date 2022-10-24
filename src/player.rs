// use std::collections::HashMap;
use super::interaction::Item;
extern crate rand;
use rand::Rng;
use macroquad::prelude::*;
use super::map::CELL_SIZE;
use std::fmt;
use macroquad::ui::{hash, root_ui, widgets, Skin,Style};
use super::map::AdvanceTileTypes;

const CC_RANGE: f32 = 1.;
pub fn character(char_type:&Character) -> [String; 4]{
    match char_type {
         Character::Priest => ["priest1_v2_1".to_string(),"priest1_v2_2".to_string(),"priest1_v2_3".to_string(),"priest1_v2_4".to_string()],
    }
}
#[derive(Default,Clone,Copy)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T
}
impl Coordinates<i16> {
    pub fn distance(&self, other: &Coordinates<i16>) -> f32{
        return (((self.x-other.x).pow(2)+(self.y-other.y).pow(2)) as f32).sqrt();
    }
}
#[derive(Clone)]
pub struct Health {
    pub points: i16,
    base_health: i16
}
pub struct Inventory {
    pub storage: Vec<Item>,
    skin: Style
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            storage: Vec::new(),
            skin: root_ui()
            .style_builder()
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0)).build(),
        }
    }
}
impl Inventory {
    pub fn display_inventory(&self) {
        let button_skin = {let button_style = self.skin.clone();
            Skin {
                button_style,
                ..root_ui().default_skin()
            }};
        root_ui().push_skin(&button_skin);
            root_ui().window(hash!(), vec2(0., screen_height() / 15.), vec2(screen_width() / 6., screen_height() * 13.0 / 15.), |ui| {
                for each_item_index in 0..self.storage.len() {
                    if widgets::Button::new(&self.storage[each_item_index].item_type.to_string()[..])
                    .position(vec2(0.,0.+each_item_index as f32 * 25.))
                    .ui(ui) {
                        println!("Pushed", );
                    }
                    ui.separator();
                }
            });
    }

}

impl Health {
    pub fn adjust(&mut self, increment:i16) -> Option<bool>{
        self.points += increment;
        if self.points+increment >= 0 {
            return None;
        }
        return Some(false)
    }
    pub fn new(base_health:i16) -> Self {
        Self {
            points: base_health,
            base_health: base_health,
        }
    }
    pub fn draw_health(&self, texture: Texture2D,x:f32,y:f32, dest_size: Vec2, is_player: bool) {
        let mut shift = 0;
        if is_player {

            draw_texture_ex(texture,x+dest_size[0],y,WHITE, DrawTextureParams {
                dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
            });
            draw_texture_ex(texture,x+2.*dest_size[0],y,WHITE, DrawTextureParams {
                dest_size: Some(vec2(dest_size[0]*2.,dest_size[1])), source: Some(Rect {x:60.,y:44.,w: 16.,h:16.}), ..Default::default()
            });
            draw_texture_ex(texture,x+4.*dest_size[0],y,WHITE, DrawTextureParams {
                dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
            });
            shift = 4
        }
        draw_bar(x,y,texture,dest_size,self.base_health as usize,shift);
        // draw_texture_ex(texture,x,y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:96.,y:52.,w: 8.,h:16.}), ..Default::default()
        // });
        // draw_texture_ex(texture,x+dest_size[0],y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
        // });
        // draw_texture_ex(texture,x+2.*dest_size[0],y,WHITE, DrawTextureParams {
        //     dest_size: Some(vec2(dest_size[0]*2.,dest_size[1])), source: Some(Rect {x:60.,y:44.,w: 16.,h:16.}), ..Default::default()
        // });
        // draw_texture_ex(texture,x+4.*dest_size[0],y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
        // });
        // for i in 0..self.base_health {
        // draw_texture_ex(texture,x+5.*dest_size[0]+dest_size[0]*i as f32,y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:96.,y:12.,w: 8.,h:16.}), ..Default::default()
        //     });
        // }
       
        for i in 0..self.points-1 {
            draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0]+dest_size[0]*i as f32,y,WHITE, DrawTextureParams {
                dest_size: Some(dest_size), source: Some(Rect {x:104.,y:32.,w: 8.,h:16.}), ..Default::default()
                });
            }
        if self.base_health-self.points != 0 &&  self.points != 0{
            draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0]+dest_size[0]*(self.points-1) as f32,y,WHITE, DrawTextureParams {
                dest_size: Some(dest_size), source: Some(Rect {x:112.,y:32.,w: 8.,h:16.}), ..Default::default()
                });
            }
            else {
                if self.points != 0 {
                    draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0]+dest_size[0]*(self.points-1) as f32,y,WHITE, DrawTextureParams {
                        dest_size: Some(dest_size), source: Some(Rect {x:104.,y:32.,w: 8.,h:16.}), ..Default::default()
                        });
                }
                }
        // draw_texture_ex(texture,x+5.*dest_size[0] +dest_size[0]*(self.base_health-1) as f32,y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
        // });
        // draw_texture_ex(texture,x+5.*dest_size[0]+dest_size[0]*(self.base_health) as f32,y,WHITE, DrawTextureParams {
        //     dest_size: Some(dest_size), source: Some(Rect {x:96.,y:52.,w: 8.,h:16.}), flip_x: true, ..Default::default()
        // });
    }
}
#[derive(Clone)]
pub struct Damage {
    ranged_damage: Option<i16>,
    cc_damage: i16,
    accuracy: f32,
}
impl Damage {
    pub fn deal(&self, distance: Option<i16>) -> i16 {
        if distance.is_none() {
            return self.cc_damage
        }
        let mut rng = rand::thread_rng();
        let chance: i16 = rng.gen_range(0..=100);
        let hit = (self.accuracy >= chance as f32) as i16;
        return self.ranged_damage.unwrap()*hit
    }
    pub fn new( base_cc:i16, base_range:i16, base_accuracy:f32) -> Self {
        Self {
            ranged_damage: Some(base_range),
            cc_damage: base_cc,
            accuracy: base_accuracy,
        }
    }
}
pub enum Character {
    Priest
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Character::Priest => write!(f,"Priest"),
        }
    }
}

pub struct PlayerCharacter {
    pub health: Health,
    pub damage: Damage,
    pub storage: Inventory,
    pub cor: Coordinates<i16>,
    pub frame: i16,
    pub character: Character
}
impl PlayerCharacter {
    pub fn intialise(base_health: i16, base_cc:i16, base_range:i16, base_accuracy:f32,spawn: Coordinates<i16>) -> Self{
        Self {
            health: Health::new(base_health),
            damage: Damage::new(base_cc,base_range,base_accuracy),
            storage: Inventory::default(),
            cor: spawn,
            frame: 0,
            character: Character::Priest,
        }
    }
    pub fn draw_player(&self, texture: Texture2D) {
        draw_texture_ex(texture,self.cor.x as f32 * CELL_SIZE, self.cor.y as f32 * CELL_SIZE, WHITE, DrawTextureParams {
            source: Some(Rect {
               x: 0.,y: 0.,w: CELL_SIZE ,h:CELL_SIZE
            }),
            ..Default::default()
        });
    }
    pub fn update_player_frame(&mut self) -> i16 {
        let frame = match self.frame {
         0 => 1,
         1 => 2,
         2 => 3,
         3 => 0,
         _ => 0,
         
        };
        self.frame = frame;
        return frame
    }
}
#[derive(Clone)]
pub enum EntityType {
    Vampire,Skelly,Skelly2,Skull
}
#[derive(Clone)]
enum EntityStatus {
    Passive,
    Violent,
    Neutral
}
#[derive(Clone)]
pub struct Entity {
    pub health: Health,
    damage: Damage,
    pub cor: Coordinates<i16>,
    pub entity_type: EntityType,
    entity_status: EntityStatus,
    pub frame: usize,

}
impl Entity {
    pub fn intialise(base_health: i16, base_cc:i16, base_range:i16, base_accuracy:f32,spawn:Coordinates<i16>) -> Self {
        
        let mut rng = rand::thread_rng();
        let random_mob = rng.gen_range(0..4);
        let e_type =  match random_mob {
            0 => EntityType::Vampire,
            1 => EntityType::Skelly,
            2 => EntityType::Skelly2,
            3 => EntityType::Skull,
            _ => EntityType::Skelly2,
        };
        let e_status = match &e_type {
            _ => EntityStatus::Violent,
        };
        Self {
            health: Health::new(base_health),
            damage: Damage::new(base_cc,base_range,base_accuracy),
            cor: spawn,
            entity_type: e_type,
            entity_status: e_status,
            frame: 0,
        }
    }
    fn _attack(&self, target_pos: Coordinates<i16>) -> Option<i16> {
        if !matches!(self.entity_status,EntityStatus::Passive) {
            return Some(match (((self.cor.x-target_pos.x).pow(2)+(self.cor.y-target_pos.y).pow(2)) as f32).sqrt() >= (2.0 as f32).sqrt() {
                true => self.damage.cc_damage,
                false => self.damage.ranged_damage.unwrap()
            })
        }
        return None
    }
    pub fn _update_entity_frame(&mut self) {
        // Match and shift entity frame for which texture is need to be load, for animations
        let frame = match self.frame {
         0 => 1,
         1 => 2,
         2 => 3,
         3 => 0,
         _ => 0,
         
        };
        self.frame = frame;
    }
    pub fn consider_action(&mut self,tile_placement: &Vec<Vec<AdvanceTileTypes>>,player: Coordinates<i16>, others: &Vec<Coordinates<i16>>) -> Option<i16>{
        // Creates vector of all possible moves for entities
        let moves = [(self.cor.x,self.cor.y+1),(self.cor.x,self.cor.y-1),(self.cor.x-1,self.cor.y),(self.cor.x+1,self.cor.y )];
        // Iterates through all moves to find all moves which a valid e.g. no other entities coordiantes or is a valid tiles shown in the match statement
        let possible_moves: Vec<(i16,i16)> = moves.into_iter().filter(|&x| true == match tile_placement[x.1 as usize][x.0 as usize] {
            AdvanceTileTypes::GenericFloor => true,
            AdvanceTileTypes::SmallCHest => true,
            AdvanceTileTypes::Bones => true,
            AdvanceTileTypes::Skull => true,
            AdvanceTileTypes::Chest => true,
            _ => false,
        // Iterates through other to find any point where the possible move is invalid
        } && !others.iter().any(|cor| cor.x == x.0 && cor.y == x.1)).collect();
        
        let mut rng = rand::thread_rng();
        // As long as there is a move
        if possible_moves.len() != 0 {
            // If player is within detection range e.g. there coordiantes are less then detection range
            if distance(player.x,player.y,self.cor.x,self.cor.y) <=  15. {
                // creates a vector of the cost for all moves as the distance betweeen two points
                let cost = possible_moves.iter().map(|x| distance(player.x,player.y,x.0,x.1)).collect::<Vec<f32>>();
                // creates an iterator object
                let mut proximity_cost = cost.iter();
                // If an entity's health drops below a certain value it would attempt to run away
                if self.health.points <= (self.health.base_health/5){
                    // find the highest cost value for a move e.g. move which takes them further away from the player
                    let max = proximity_cost.clone().max_by(|x, y| x.partial_cmp(&y).unwrap()).unwrap();
                    // is the max value is greater then a certain value it will adjust health/heal
                    if *max >= 15. {
                        self.health.adjust(rng.gen_range(0..=1));
                    } else {
                        // if not far enough away from the player entity will decide to run away by picking the max cost move
                        let new_cor = possible_moves[proximity_cost.position(|&x| x == *max).unwrap()];
                        self.cor = Coordinates {x:new_cor.0,y:new_cor.1};
                    }
                } else {
                    // Find the smallest cost, which brings it closer to the player
                    let min = proximity_cost.clone().min_by(|x, y| x.partial_cmp(&y).unwrap()).unwrap();
                    //if the smallest value is within close combat range the function will return a damage value
                    if min <= &CC_RANGE {
                        return Some(self.damage.deal(Some(1)));
                        
                    }
                    else {
                        // Gets the new coordinates from the smallest cost value
                        let new_cor = possible_moves[proximity_cost.position(|&x| x == *min).unwrap()];
                        // sets new_cor to mob coordinates
                        self.cor = Coordinates {x:new_cor.0,y:new_cor.1};
                    }
                }
            }
            else {
                // If entity is not within detection range moves randomly
                let random_move = rng.gen_range(0..possible_moves.len());
                self.cor = Coordinates {x:possible_moves[random_move].0,y:possible_moves[random_move].1};
            }
            None
    } else {
        None
    }
}
    pub fn draw_entity(&self, texture: Texture2D, hud_texture: Texture2D) {
        // Takes textures and use macroquad draw texture function to display the entity
        self.health.draw_health(hud_texture,self.cor.x as f32*CELL_SIZE,self.cor.y as f32 * CELL_SIZE -4. ,vec2(2.,4.),false);
        draw_texture_ex(texture,self.cor.x as f32 * CELL_SIZE, self.cor.y as f32 * CELL_SIZE, WHITE, DrawTextureParams {
            source: Some(Rect {
               x: 0.,y: 0.,w: CELL_SIZE ,h:CELL_SIZE
            }),
            ..Default::default()
        });
    }  
}
fn distance(x: i16,y:i16,x2:i16,y2:i16) -> f32 {
    // Calculates the distant between two points
    (((x2-x).pow(2)+(y2-y).pow(2)) as f32).sqrt() as f32
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_distance() {
//         assert_eq!(distance(2,3,4,5),2.*(2 as f32).sqrt())
//     }
// }
pub fn draw_bar(x:f32,y:f32, texture: Texture2D, dest_size: Vec2, inner_length: usize, shift: i8) {
    draw_texture_ex(texture,x,y,WHITE, DrawTextureParams {
        dest_size: Some(dest_size), source: Some(Rect {x:96.,y:52.,w: 8.,h:16.}), ..Default::default()
    });
    
    for i in 0..inner_length {
        draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0]+dest_size[0]*i as f32,y,WHITE, DrawTextureParams {
            dest_size: Some(dest_size), source: Some(Rect {x:96.,y:12.,w: 8.,h:16.}), ..Default::default()
            });
        }
    draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0] +dest_size[0]*(inner_length-1) as f32,y,WHITE, DrawTextureParams {
        dest_size: Some(dest_size), source: Some(Rect {x:104.,y:52.,w: 8.,h:16.}), ..Default::default()
    });
    draw_texture_ex(texture,x+(shift as f32 + 1.)*dest_size[0]+dest_size[0]*(inner_length) as f32,y,WHITE, DrawTextureParams {
        dest_size: Some(dest_size), source: Some(Rect {x:96.,y:52.,w: 8.,h:16.}), flip_x: true, ..Default::default()
    });
}