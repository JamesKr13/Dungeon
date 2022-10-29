// use std::collections::HashMap;
use super::interaction::{Item,DrawText,Items};
extern crate rand;
use super::map::AdvanceTileTypes;
use super::map::CELL_SIZE;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin, Style};
use rand::Rng;
use std::fmt;

const CC_RANGE: f32 = 1.;
#[derive(Default, Clone, Copy)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
}
impl Coordinates<i16> {
    #[must_use]
    pub fn distance(&self, other: &Coordinates<i16>) -> f32 {
        f32::from((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).sqrt()
    }
}
#[derive(Clone)]
pub struct Health {
    pub points: i16,
    base_health: i16,
}
#[derive(Clone)]
pub struct Points {
    pub points: i16,
    base_points: i16,
}
impl Points {
    pub fn adjust(&mut self, increment: i16) -> Option<bool> {
        if self.points + increment <= self.base_points && self.points + increment >= 0 {
            self.points += increment;
            return None;
        };
        Some(false)
    }
    pub fn base_adjust(&mut self, increment: i16) {
        if self.base_points + increment >= 0{
            self.base_points += increment;
        };
    }
    #[must_use]
    pub fn new(base_points: i16) -> Self {
        Self {
            points: base_points,
            base_points:base_points,
        }
    }
    pub fn draw_points(
        &self,
        texture: Texture2D,
        x: f32,
        y: f32,
        dest_size: Vec2,
        is_player: bool,
        is_health: bool,
    ) {
        let var_source: [[f32;2];2];
        if is_health {
            var_source = [[60.,44.],[104.,32.]];
        } else {
            var_source = [[76.,44.],[128.,92.]];
        }
        let mut shift = 0;
        if is_player {
            draw_texture_ex(
                texture,
                x + dest_size[0],
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(Rect {
                        x: 104.,
                        y: 52.,
                        w: 8.,
                        h: 16.,
                    }),
                    ..Default::default()
                },
            );
            // Bar time, Draw the identifer for bar type
            draw_texture_ex(
                    texture,
                    x + 2. * dest_size[0],
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(dest_size[0] * 2., dest_size[1])),
                        source: Some(Rect {
                            x: var_source[0][0],
                            y: var_source[0][1],
                            w: 16.,
                            h: 16.,
                        }),
                        ..Default::default()
                    },
                );
            draw_texture_ex(
                texture,
                x + 4. * dest_size[0],
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(Rect {
                        x: 104.,
                        y: 52.,
                        w: 8.,
                        h: 16.,
                    }),
                    ..Default::default()
                },
            );
            shift = 4
        }
        draw_bar(x, y, texture, dest_size, self.base_points as usize, shift);
        for i in 0..self.points - 1 {
            draw_texture_ex(
                texture,
                x + (f32::from(shift) + 1.) * dest_size[0] + dest_size[0] * f32::from(i),
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(Rect {
                        x: var_source[1][0],
                        y: var_source[1][1],
                        w: 8.,
                        h: 16.,
                    }),
                    ..Default::default()
                },
            );
        }
        if self.base_points - self.points != 0 && self.points != 0 {
            draw_texture_ex(
                texture,
                x + (f32::from(shift) + 1.) * dest_size[0]
                    + dest_size[0] * f32::from(self.points - 1),
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(Rect {
                        x: var_source[1][0]+8.,
                        y: var_source[1][1],
                        w: 8.,
                        h: 16.,
                    }),
                    ..Default::default()
                },
            );
        } else if self.points != 0 {
            draw_texture_ex(
                texture,
                x + (f32::from(shift) + 1.) * dest_size[0]
                    + dest_size[0] * f32::from(self.points - 1),
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: Some(Rect {
                        x: var_source[1][0],
                        y: var_source[1][1],
                        w: 8.,
                        h: 16.,
                    }),
                    ..Default::default()
                },
            );
        }
    }
}
pub struct Inventory {
    pub storage: Vec<Item>,
    skin: Style,
    state: bool,
    display_index: Option<usize>,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            storage: Vec::new(),
            skin: root_ui()
                .style_builder()
                .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
                .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
                .build(),
            state: false,
            display_index: None,
        }
    }
}
impl Inventory {
    pub fn display_inventory(&mut self) -> Option<[i16;5]>{
        let button_skin = {
            let button_style = self.skin.clone();
            Skin {
                button_style,
                ..root_ui().default_skin()
            }
            
        };
        root_ui().push_skin(&button_skin);
        root_ui().window(
            hash!(),
            vec2(0., screen_height() / 15.),
            vec2(screen_width() / 6., screen_height() * 13.0 / 15.),
            |ui| {
                for each_item_index in 0..self.storage.len() {
                    if widgets::Button::new(
                        format!("{}: {}",&self.storage[each_item_index].name,match self.storage[each_item_index].equip {
                            false => "Not".to_string(),
                            true => self.storage[each_item_index].slot.to_string(),
                        }),
                    )
                    .position(vec2(0., 0. + each_item_index as f32 * 25.))
                    .ui(ui)
                    {   
                        self.display_index = Some(each_item_index);
                    }
                    ui.separator();
                }                
            },
            
        );
        let mut output_effect: Option<[i16;5]> = None;
        if self.display_index.is_some(){
            let effect: [i16;5];
            let effect_text: String;
            let item_type = self.storage[self.display_index.unwrap()].item_type;
            let items_uses_text: (&'static str,&'static str);
            if item_type.consumable() {
                effect_text = format!("{}",match item_type {
                    Items::BigHealth => "Heals 10 health",
                    Items::BigStamina => "Regens 10 Stamina",
                    Items::Stamina => "Regens 5 Stamina",
                    Items::Health => "Heals 5 Health",
                    Items::Mystery => "I don't know what it does but drink it.",
                    _ => "No effects"
                } );
                effect = match item_type {
                    Items::BigHealth => [0,0,0,10,0],
                    Items::BigStamina => [0,0,0,0,10],
                    Items::Stamina => [0,0,0,0,5],
                    Items::Health => [0,0,0,5,0],
                    Items::Mystery => self.storage[self.display_index.unwrap()].effect,
                    _ => [0,0,0,0,0]
                };
                items_uses_text = ("Consume it","Consumed");
            } else {
                let temp = self.storage[self.display_index.unwrap()].effect;
                effect = [temp[0],temp[1],temp[2],0,0];
                effect_text = format!("Health {}, Damage {}, Stamina {}",effect[0],effect[1],effect[2]);
                items_uses_text = ("Equip","Unequip");
            }
            self.storage.sort_by(|a,b| b.equip.cmp(&a.equip));
        root_ui().window(
            hash!("InfoMenu"),
            vec2(screen_width() / 6., screen_height() / 15. + self.display_index.unwrap() as f32 * 25.),
            vec2(screen_width() / 3., screen_height() * 3.0 / 15.),
            |ui| {
                    let description = self.storage[self.display_index.unwrap()].description.clone();
                    let mut vector: Vec<String> = Vec::new();
                    let each_word: Vec<String> = description.split(' ').map(str::to_string).collect();
                    let mut tally = 0;
                    for word in each_word {
                        if !vector.is_empty() && tally + word.len() <= 20 {
                            tally += word.len();
                            let index = vector.len() - 1;
                            vector[index].push_str(&format!(" {}", word)[..]);
                        } else {
                            tally = 0;
                            vector.push(word);
                        }
                    }
                    for line in 0..vector.len() {
                        widgets::Button::new(vector[line].clone()).position(vec2(0.,line as f32 *25.))
                    .ui(ui);
                    }
                    
                    widgets::Button::new(effect_text).position(vec2(0.,(vector.len()+2) as f32 *25.))
                        .ui(ui);
                    if self.storage[self.display_index.unwrap()].equip {
                        if widgets::Button::new(items_uses_text.1).position(vec2(0.,(vector.len()+1) as f32 *25.))
                        .ui(ui) {
                            self.storage[self.display_index.unwrap()].equip = false;
                            let all_effects = self.storage[self.display_index.unwrap()].effect;
                            output_effect = Some([-all_effects[0],-all_effects[1],-all_effects[2],0,0]);
                        }
                    } else {
                        if widgets::Button::new(items_uses_text.0).position(vec2(0.,(vector.len()+1) as f32 *25.))
                        .ui(ui) {
                            let if_any: Vec<Item> = self.storage.clone().into_iter().filter(|item| item.equip == true && item.slot.to_string() == self.storage[self.display_index.unwrap()].slot.to_string()).collect();
                            if if_any.len() == 0 {
                            if item_type.consumable() {
                                self.storage.remove(self.display_index.unwrap());
                                self.display_index = None;
                            } else {
                                self.storage[self.display_index.unwrap()].equip = true;
                            }
                            output_effect = Some(effect);

                            }
                        }
                    }
                    if is_mouse_button_pressed(MouseButton::Right) && self.state{
                        println!("runn because {} {}", self.state, self.display_index.unwrap());
                        self.display_index = None;
                        self.state = false;
                    }
                    self.state = true;
                
            },
            
        );
        
    }
    output_effect
}
}

#[derive(Clone)]
pub struct Damage {
    ranged_damage: Option<i16>,
    cc_damage: i16,
    accuracy: f32,
}
impl Damage {
    #[must_use]
    pub fn deal(&self, distance: Option<i16>) -> i16 {
        if distance.is_none() {
            return self.cc_damage;
        }
        let mut rng = rand::thread_rng();
        let chance: i16 = rng.gen_range(0..=100);
        let hit = i16::from(self.accuracy >= f32::from(chance));
        self.ranged_damage.unwrap() * hit
    }
    #[must_use]
    pub fn new(base_cc: i16, base_range: i16, base_accuracy: f32) -> Self {
        Self {
            ranged_damage: Some(base_range),
            cc_damage: base_cc,
            accuracy: base_accuracy,
        }
    }
    pub fn adjust(&mut self, value: i16) {
        self.cc_damage += value;
    }
}
pub enum Character {
    Priest,
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Character::Priest => write!(f, "Priest"),
        }
    }
}

pub struct PlayerCharacter {
    pub health: Points,
    pub damage: Damage,
    pub stamina: Points,
    pub storage: Inventory,
    pub cor: Coordinates<i16>,
    pub frame: i16,
    pub character: Character,
}
impl PlayerCharacter {
    #[must_use]
    pub fn intialise(
        base_health: i16,
        base_cc: i16,
        base_range: i16,
        base_accuracy: f32,
        spawn: Coordinates<i16>,
        base_stamina: i16
    ) -> Self {
        Self {
            health: Points::new(base_health),
            damage: Damage::new(base_cc, base_range, base_accuracy),
            storage: Inventory::default(),
            cor: spawn,
            frame: 0,
            character: Character::Priest,
            stamina: Points::new(base_stamina)
        }
    }
    pub fn draw_player(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            f32::from(self.cor.x) * CELL_SIZE,
            f32::from(self.cor.y) * CELL_SIZE,
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
    pub fn update_player_frame(&mut self) -> i16 {
        let frame = match self.frame {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            _ => 0,
        };
        self.frame = frame;
        frame
    }
}
#[derive(Clone)]
pub enum EntityType {
    Vampire,
    Skelly,
    Skelly2,
    Skull,
}
#[derive(Clone)]
enum EntityStatus {
    Passive,
    Violent,
    Neutral,
}
#[derive(Clone)]
pub struct Entity {
    pub health: Points,
    damage: Damage,
    pub cor: Coordinates<i16>,
    pub entity_type: EntityType,
    entity_status: EntityStatus,
    pub frame: usize,
    heal_percent: f32,
}
impl Entity {
    #[must_use]
    pub fn intialise(
        base_health: i16,
        base_cc: i16,
        base_range: i16,
        base_accuracy: f32,
        spawn: Coordinates<i16>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let random_mob = rng.gen_range(0..4);
        let e_type = match random_mob {
            0 => EntityType::Vampire,
            1 => EntityType::Skelly,
            2 => EntityType::Skelly2,
            3 => EntityType::Skull,
            _ => EntityType::Skelly2,
        };
        let e_status = EntityStatus::Violent;
        Self {
            health: Points::new(base_health),
            damage: Damage::new(base_cc, base_range, base_accuracy),
            cor: spawn,
            entity_type: e_type,
            entity_status: e_status,
            frame: 0,
            heal_percent: 5.,
        }
    }
    fn _attack(&self, target_pos: Coordinates<i16>) -> Option<i16> {
        if !matches!(self.entity_status, EntityStatus::Passive) {
            return Some(
                match f32::from(
                    (self.cor.x - target_pos.x).pow(2) + (self.cor.y - target_pos.y).pow(2),
                )
                .sqrt()
                    >= 2.0_f32.sqrt()
                {
                    true => self.damage.cc_damage,
                    false => self.damage.ranged_damage.unwrap(),
                },
            );
        }
        None
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
    pub fn consider_action(
        &mut self,
        tile_placement: &Vec<Vec<AdvanceTileTypes>>,
        player: Coordinates<i16>,
        others: &Vec<Coordinates<i16>>,
    ) -> Option<i16> {
        // Creates vector of all possible moves for entities
        let moves = [
            (self.cor.x, self.cor.y + 1),
            (self.cor.x, self.cor.y - 1),
            (self.cor.x - 1, self.cor.y),
            (self.cor.x + 1, self.cor.y),
        ];
        // Iterates through all moves to find all moves which a valid e.g. no other entities coordiantes or is a valid tiles shown in the match statement
        let possible_moves: Vec<(i16,i16)> = moves.into_iter().filter(|&x| match tile_placement[x.1 as usize][x.0 as usize] {
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
        if !possible_moves.is_empty() {
            // If player is within detection range e.g. there coordiantes are less then detection range
            if distance(player.x, player.y, self.cor.x, self.cor.y) <= 10. {
                // creates a vector of the cost for all moves as the distance betweeen two points
                let cost = possible_moves
                    .iter()
                    .map(|x| distance(player.x, player.y, x.0, x.1))
                    .collect::<Vec<f32>>();
                // creates an iterator object
                let mut proximity_cost = cost.iter();
                // If an entity's health drops below a certain value it would attempt to run away
                if self.health.points
                    <= (f32::from(self.health.base_points) * self.heal_percent) as i16
                {
                    // find the highest cost value for a move e.g. move which takes them further away from the player
                    let max = proximity_cost
                        .clone()
                        .max_by(|x, y| x.partial_cmp(y).unwrap())
                        .unwrap();
                    // is the max value is greater then a certain value it will adjust health/heal
                    if *max >= 10. {
                        let chance = rng.gen_range(0..=1);
                        self.health.adjust(1*i16::from(chance <= 3));
                        self.heal_percent = 0.7;
                    } else {
                        // if not far enough away from the player entity will decide to run away by picking the max cost move
                        let new_cor =
                            possible_moves[proximity_cost.position(|&x| x == *max).unwrap()];
                        self.cor = Coordinates {
                            x: new_cor.0,
                            y: new_cor.1,
                        };
                    }
                } else {
                    self.heal_percent = 0.5;
                    // Find the smallest cost, which brings it closer to the player
                    let min = proximity_cost
                        .clone()
                        .min_by(|x, y| x.partial_cmp(y).unwrap())
                        .unwrap();
                    //if the smallest value is within close combat range the function will return a damage value
                    if min <= &0. {
                        return Some(self.damage.deal(Some(1)));
                    } else {
                        // Gets the new coordinates from the smallest cost value
                        let new_cor =
                            possible_moves[proximity_cost.position(|&x| x == *min).unwrap()];
                        // sets new_cor to mob coordinates
                        self.cor = Coordinates {
                            x: new_cor.0,
                            y: new_cor.1,
                        };
                    }
                }
            } else {
                // If entity is not within detection range moves randomly
                let random_move = rng.gen_range(0..possible_moves.len());
                self.cor = Coordinates {
                    x: possible_moves[random_move].0,
                    y: possible_moves[random_move].1,
                };
            }
            None
        } else {
            None
        }
    }
    pub fn draw_entity(&self, texture: Texture2D, hud_texture: Texture2D) {
        // Takes textures and use macroquad draw texture function to display the entity
        self.health.draw_points(
            hud_texture,
            f32::from(self.cor.x) * CELL_SIZE,
            f32::from(self.cor.y) * CELL_SIZE - 4.,
            vec2(2., 4.),
            false,
            true,
        );
        draw_texture_ex(
            texture,
            f32::from(self.cor.x) * CELL_SIZE,
            f32::from(self.cor.y) * CELL_SIZE,
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
}
fn distance(x: i16, y: i16, x2: i16, y2: i16) -> f32 {
    // Calculates the distant between two points
    f32::from((x2 - x).pow(2) + (y2 - y).pow(2)).sqrt()
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_distance() {
//         assert_eq!(distance(2,3,4,5),2.*(2 as f32).sqrt())
//     }
// }
pub fn draw_bar(
    x: f32,
    y: f32,
    texture: Texture2D,
    dest_size: Vec2,
    inner_length: usize,
    shift: i8,
) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            source: Some(Rect {
                x: 96.,
                y: 52.,     
                w: 8.,
                h: 16.,
            }),
            ..Default::default()
        },
    );

    for i in 0..inner_length {
        draw_texture_ex(
            texture,
            x + (f32::from(shift) + 1.) * dest_size[0] + dest_size[0] * i as f32,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                source: Some(Rect {
                    x: 96.,
                    y: 12.,
                    w: 8.,
                    h: 16.,
                }),
                ..Default::default()
            },
        );
    }
    draw_texture_ex(
        texture,
        x + (f32::from(shift) + 2.) * dest_size[0] + dest_size[0] * (inner_length - 1) as f32,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            source: Some(Rect {
                x: 104.,
                y: 52.,
                w: 8.,
                h: 16.,
            }),
            ..Default::default()
        },
    );
    draw_texture_ex(
        texture,
        x + (f32::from(shift) + 2.) * dest_size[0] + dest_size[0] * (inner_length) as f32,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(dest_size),
            source: Some(Rect {
                x: 96.,
                y: 52.,
                w: 8.,
                h: 16.,
            }),
            flip_x: true,
            ..Default::default()
        },
    );
}
