use macroquad::ui::{hash, root_ui, widgets, Skin,Style};
use macroquad::prelude::*;
use super::Control::*;
use std::collections::HashMap;
use regex::Regex;
use std::fmt;
use super::player::{Coordinates,PlayerCharacter};
extern crate rand;
use ::rand::Rng;



// pub struct Action {
//     pub key: KeyCode,
//     pub state: States
// }

// impl Action {
//     fn new(inter_type: InteractionTypes) -> Option<Self> {
//         let params = Some(vec!(0,0));
//         if !params.is_none() {
//             params = params.unwrap();
//             Some(Self {
//                 key: params.0,
//                 state: params.1
//             })
//         } else {
//             None
//         }
//     }
// }
const COL_ROW_SIZE: usize = 5;
const INVENTORY_SPACE: usize = 25;


#[derive(Clone,Copy)]
pub enum Item {
    Sword, Bow, Empty
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Sword => write!(f,"Sword"),
            Item::Bow => write!(f,"Bow"),
            Item::Empty => write!(f,"Nothing")
        }
    }
}
struct CFG {
    prod: HashMap<String,Vec<String>>,
    proccess_input: Regex,
    final_process_input: Regex,
}
// impl CFG {
//     fn init() -> Self {
//         Self {
//             prod: HashMap::new(),
//             proccess_input: Regex::new(r"(:)").unwrap(),
//             final_process_input: Regex::new(r"()")
//         }
//     }
//     fn add_prod(&mut self, lhs: String, rhs: String){
//         let prods: Vec<String> = self.proccess_input.find_iter(&(&rhs)[..]).filter_map(|words| words.as_str().parse().ok()).collect();
//         for prod in prods {
//             if self.prod.contains_key(&lhs){
//             self.prod.get_mut(&prod).expect("Shouldd not be empty").push(prod.chars().collect());
//             }
//         }
//     }
//     fn gen_random(&self,symbol) {
//         let mut rng = rand::thread_rng();
//         let rand_index: usize = rng.gen_range(0..self.prod.get(&symbol).expect("Invalid type").len());
//         let rand_prod = self.prod.get(&symbol).expect("Invalid type")[rand_index];
//         let sentence = String::new();
//         for sym in rand_prod.chars() {
//             if self.prod.contains_key(&sym.to_string()) {
//                 sentence.push_str(self.gen_random(sym))
//             }
//             else {
//                 sentence.push_str();
//             }
//         }
//     }
// }
#[derive(Default,Clone)]
struct InventorySkins {
    skins: HashMap<String,Style>
}
impl InventorySkins {
    fn create_inventory_skins(&mut self,items: &Vec<Item>) {
        for item in items {
            self.skins.insert(item.to_string(),root_ui()
                .style_builder().
                background(Image::from_file_with_format(include_bytes!("../lib/arrow_1.png"),None))
                .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
                .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0)).build());
                };
        }
    }
#[derive(Default,Clone)]
pub struct ClickActions {
    pub run_state: bool,
    pub on_set_mouse_position: Coordinates<f32>,
}
impl ClickActions {
    pub fn walk_menu(&mut self, set_pos: bool) {
        // root_ui().window(hash!(),vec2(pos.x,pos.y),vec2(50.,50.), |ui| {
        //         if (widgets::Button::new("Move Here")
        //         .position(vec2(mouse_position().0,mouse_position().1))
        //         .ui(ui)) {
        //             println!("Pushed", );
        //         } 
        //         if (widgets::Button::new("Take Item")
        //         .position(vec2(0.,25.))
        //         .ui(ui)) {
        //             println!("Pushed", );
        //         } 
        // });
        // println!("still fine", );
        
        root_ui().window(hash!("Menu"), vec2(mouse_position().0,mouse_position().1), vec2(65., 40.), |ui| {
            if widgets::Button::new("Move Here")
            .position(vec2(0.,0.))
            .ui(ui) {
                self.run_state = false;
                println!("Pushed |.................................................................................................................|", );
                // self.click_action.run_state = true;
            }
            if widgets::Button::new("Take Item")
            .position(vec2(0.,20.))
            .ui(ui) {
                self.run_state = false;
                println!("Pushed |.................................................................................................................|", );
                // self.click_action.run_state = true;
            }
    });
    if set_pos {
        self.on_set_mouse_position = Coordinates {x:mouse_position().0,y:mouse_position().1};
        root_ui().move_window(hash!("Menu"),vec2(mouse_position().0,mouse_position().1));
    }
    }
}
#[derive(Clone)]
pub struct Storage {
    pub items: Vec<Item>,
    pub key: KeyCode,
    pub alt_state: States,
    inventory_skins: InventorySkins,
    pub used: bool,
}
impl Default for Storage {
    fn default() -> Self {
        let mut ivsk = InventorySkins::default();
        let all_items = vec!(Item::Bow,Item::Sword,Item::Sword);
        ivsk.create_inventory_skins(&all_items);
        Self {
            items: all_items,
            key: KeyCode::E,
            alt_state: States::Storage,
            inventory_skins: ivsk,
            used: false,
        }
    }
}
impl Storage {
    pub fn display(&mut self) -> Option<Item>{
            // // let button_skin = {let button_style = (self.inventory_skins.skins.get(&self.items[each_item_index].to_string()).unwrap()).clone();
            // Skin {
            //     button_style,
            //     ..root_ui().default_skin()
            // }};
            // root_ui().push_skin(&button_skin);
            let mut item_removed = 0;
            let mut item:Option<Item> = None;
            let window_size = vec2(150., 3.*screen_height()/4.);
            root_ui().window(hash!("Chest"),vec2(screen_width()-150.,screen_height()/8.),window_size, |ui| {
                if widgets::Button::new(self.items[0].to_string())
                    .position(vec2(0.,0.))
                    .ui(ui){
                        item = Some(self.items[0]);
                        self.used = true;
                    }
                    ui.separator();
                if widgets::Button::new(self.items[1].to_string())
                    .position(vec2(0.,25.))
                    .ui(ui){
                        item = Some(self.items[1]);
                        self.used = true;
                    }
                    ui.separator();
                if widgets::Button::new(self.items[2].to_string())
                    .position(vec2(0.,50.))
                    .ui(ui) {
                        item = Some(self.items[2]);
                        self.used = true;
                    }
                    ui.separator();
            });
            item
        } 
    }

pub struct Inventory {
    content: [Item; INVENTORY_SPACE],
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            content: [Item::Empty; INVENTORY_SPACE],
        }
    }
}
impl Inventory {
    pub fn display_inventory(&mut self) {
        for row in 0..COL_ROW_SIZE{
            for col in 0..COL_ROW_SIZE {
                draw_rectangle_lines(screen_width()/3.- col as f32 * 25.,100.- row as f32 *25., 25.,25.,5., BLUE)
            }
        }
 
    }
}

