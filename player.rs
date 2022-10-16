use std::collections::HashMap;
use super::Interaction::Item;
extern crate rand;
use rand::Rng;

const INVENTORY_SPACE: i8 = 5;

struct Coordinates<T> {
    x: T,
    y: T
}
struct Health {
    points: i8
}
impl Health {
    fn adjust(&mut self, increment:i8) -> Option<bool>{
        self.points += increment;
        if self.points >= 0 {
            return Some(false);
        }
        None
    }
    fn new(base_health:i8) -> Self {
        Self {
            points: base_health
        }
    }
}
struct Damage {
    ranged_damage: Option<i8>,
    cc_damage: i8,
    accuracy: f32,
}
impl Damage {
    fn deal(&self, distance: Option<i16>) -> i8 {
        if distance.is_none() {
            return self.cc_damage
        }
        let mut rng = rand::thread_rng();
        let chance: i16 = rng.gen_range(0..=100);
        let hit = (self.accuracy >= chance as f32) as i8;
        return self.ranged_damage.unwrap()*hit
    }
    fn new( base_cc:i8, base_range:i8, base_accuracy:f32) -> Self {
        Self {
            ranged_damage: Some(base_range),
            cc_damage: base_cc,
            accuracy: base_accuracy,
        }
    }
}

struct Player {
    health: Health,
    damage: Damage,
    inventory: [Item;25],
    cor: Coordinates<i8>
}
impl Player {
    fn intialise(base_health: i8, base_cc:i8, base_range:i8, base_accuracy:f32,spawn: Coordinates<i8>) -> Self{
        // let mut inventory_initial = [];
        // for row in 0..inventory_size.1 {
        //     for col in 0..inventory_size.0 {
        //          inventory_initial[0][1] =Item::Empty;
        //     }
        // }
        Self {
            health: Health::new(base_health),
            damage: Damage::new(base_cc,base_range,base_accuracy),
            inventory: [Item::Empty;25],
            cor: spawn
        }
    }
}
enum EntityType {
    Vampire
}
enum EntityStatus {
    Passive,Violent, Neutral
}
struct Entity {
    health: Health,
    damage: Damage,
    cor: Coordinates<i8>,
    entity_type: EntityType,
    entity_status: EntityStatus,
    movement_points: i8

}
enum AIOptions {
    Move, Attack, Heal, LookAround, Panic, AvoidPlayer, Group
}
struct AIOption {
    aioid: i32,
    option: AIOptions,
}
impl PartialEq for AIOption {
    fn eq(&self, other: &Self) -> bool {
        self.aioid == other.aioid
    }
}
impl Eq for AIOption {}
impl Entity {
    fn intialise(base_health: i8, base_cc:i8, base_range:i8, base_accuracy:f32,spawn:Coordinates<i8>, e_type: EntityType,base_movement_points: i8) -> Self {
        let e_status = match e_type {
            EntityType::Vampire => EntityStatus::Violent,
        };
        Self {
            health: Health::new(base_health),
            damage: Damage::new(base_cc,base_range,base_accuracy),
            cor: spawn,
            entity_type: e_type,
            entity_status: e_status,
            movement_points: base_movement_points
        }
    }
    fn attack(&self, target_pos: Coordinates<i8>) -> Option<i8> {
        if !matches!(self.entity_status,EntityStatus::Passive) {
            return Some(match (((self.cor.x-target_pos.x).pow(2)+(self.cor.y-target_pos.y).pow(2)) as f32).sqrt() >= (2.0 as f32).sqrt() {
                true => self.damage.cc_damage,
                false => self.damage.ranged_damage.unwrap()
            })
        }
        return None
    }
}

// struct EntityAI {
//     probability_of_actions: HashMap<AIOptions,f32>,
// }
// impl Default for EntityAI {
//     fn default() -> Self {
//         Self {

//         }
//     }
// }