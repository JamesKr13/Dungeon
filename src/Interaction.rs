use macroquad::ui::{hash, root_ui, widgets, Style};
use macroquad::prelude::*;
use super::control::*;
use std::collections::HashMap;
use regex::Regex;
use std::fmt;
use super::player::Coordinates;
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
pub struct CFG {
    rules: HashMap<String,Vec<String>>,
}
impl Default  for CFG {
    fn default() -> Self {
        Self {
            rules: HashMap::from([
                ("S".to_string(),vec!["The Adj Type of Pl U".to_string(),"A Type of Adj, U".to_string(), "The Type of Pl , U".to_string()]),
                ("U".to_string(),vec!["weild by N to V the Adj N".to_string(), "used to V N at the Nu of Pl to V N of Pl".to_string()]),
                ("Pl".to_string(),vec!["Moose Yarmtoon".to_string(),"Port Welchbaw".to_string(),"Rooplains".to_string(),"West Bol".to_string(),"Oakquet".to_string(),"Branwo Hills".to_string(),"East-West Kenwick".to_string(),"Sense Springs".to_string(),"Chamkam Ferry".to_string(),"New Mutthag".to_string()]),
                ("V".to_string(),vec!["ask".to_string(), "remove".to_string(), "play".to_string(), "decimate".to_string(), "pack".to_string(), "appreciate".to_string(), "research".to_string(), "render".to_string(), "resemble".to_string(), "handle".to_string(), "top".to_string(), "stress".to_string(), "diminish".to_string(), "submit".to_string(), "mark".to_string(), "change".to_string(), "cry".to_string(), "double".to_string(), "convey".to_string(), "part".to_string(), "manage".to_string(), "excuse".to_string(), "damage".to_string(), "lie".to_string(), "score".to_string(), "concede".to_string(), "round".to_string(), "dip".to_string(), "manipulate".to_string(), "view".to_string(), "cross".to_string(), "lock".to_string(), "illustrate".to_string(), "attach".to_string(), "promote".to_string(), "occur".to_string(), "guarantee".to_string(), "locate".to_string(), "decrease".to_string(), "set".to_string(), "build".to_string(), "combine".to_string(), "lower".to_string(), "entitle".to_string(), "conceive".to_string(), "break".to_string(), "trap".to_string(), "park".to_string(), "ruin".to_string(), "assist".to_string()]),
                ("Adj".to_string(), vec!["unaccountable".to_string(), "male".to_string(), "wrong".to_string(), "dreary".to_string(), "blue-eyed".to_string(), "muddled".to_string(), "undesirable".to_string(), "productive".to_string(), "bouncy".to_string(), "solid".to_string(), "direful".to_string(), "dark".to_string(), "bizarre".to_string(), "sordid".to_string(), "wakeful".to_string(), "material".to_string(), "gaping".to_string(), "curious".to_string(), "frightened".to_string(), "pink".to_string(), "irate".to_string(), "narrow".to_string(), "ten".to_string(), "null".to_string(), "voiceless".to_string(), "well-made".to_string(), "opposite".to_string(), "sturdy".to_string(), "black".to_string(), "irritating".to_string(), "peaceful".to_string(), "soggy".to_string(), "finicky".to_string(), "obviously".to_string(), "marvelous".to_string(), "scrawny".to_string(), "fearless".to_string(), "busy".to_string(), "chief".to_string(), "steady".to_string(), "motionless".to_string(), "scary".to_string(), "ubiquitous".to_string(), "expensive".to_string(), "physical".to_string(), "kaput".to_string(), "tiny".to_string(), "unknown".to_string(), "alive".to_string(), "red".to_string()]),
                ("Nu".to_string(), vec!["Battle".to_string(),"Party".to_string(),"war".to_string(),"Meeting".to_string(),"county".to_string(),"trade".to_string(),"division".to_string(),"hill".to_string(),"valley".to_string()]),
                ("N".to_string(), vec!("MN".to_string(),"MN LN".to_string(),"MN MN MN LN the Adj".to_string(), "MN MN LN of Pl".to_string(), "MN MN MN MN MN MN LN the RN".to_string())),
                ("MN".to_string(),vec!["".to_string(), "Leonie".to_string(), "Louis".to_string(), "Viola".to_string(), "Noah".to_string(), "Jane".to_string(), "Sean".to_string(), "Orlando".to_string(), "Hollyn".to_string(), "Benjamin".to_string(), "Gwendolen".to_string(), "Lucinda".to_string(), "Annabel".to_string(), "Daniel".to_string(), "Elijah".to_string(), "Devon".to_string(), "Robert".to_string(), "Ryder".to_string(), "Grey".to_string(), "Miriam".to_string(), "Riley".to_string(), "Abraham".to_string(), "Anise".to_string(), "Ellory".to_string(), "Sutton".to_string(), "Ray".to_string(), "Karilyn".to_string(), "Sue".to_string(), "Blayne".to_string(), "Lilibeth".to_string(), "Rhett".to_string(), "Naomi".to_string(), "Carleen".to_string(), "Robin".to_string(), "Zane".to_string(), "Dezi".to_string(), "Fawn".to_string(), "Kylie".to_string(), "Chase".to_string(), "Timothy".to_string(), "Isaiah".to_string(), "Amelia".to_string(), "Jude".to_string(), "Nicolas".to_string(), "Marcellus".to_string(), "Jackson".to_string(), "Jasper".to_string(), "Sharon".to_string(), "Trey".to_string(), "Dante".to_string(), "Finn".to_string()]),
                ("LN".to_string(),vec!["".to_string(), "Franco".to_string(), "Vazquez".to_string(), "Cummings".to_string(), "Dennis".to_string(), "Noble".to_string(), "Harrell".to_string(), "Hickman".to_string(), "Munoz".to_string(), "Perez".to_string(), "Sandoval".to_string(), "Rasmussen".to_string(), "Kaufman".to_string(), "Cox".to_string(), "Hart".to_string(), "Robles".to_string(), "Carson".to_string(), "Hunt".to_string(), "Stuart".to_string(), "Frank".to_string(), "Leach".to_string(), "Garrett".to_string(), "Gross".to_string(), "Reeves".to_string(), "Brady".to_string(), "Gonzales".to_string(), "Bradshaw".to_string(), "Hartman".to_string(), "Rivera".to_string(), "Shaw".to_string(), "Livingston".to_string(), "Mcbride".to_string(), "Zimmerman".to_string(), "Bates".to_string(), "Vaughan".to_string(), "Schmitt".to_string(), "Tanner".to_string(), "Elliott".to_string(), "Hayes".to_string(), "Ritter".to_string(), "Foster".to_string(), "Gay".to_string(), "Osborne".to_string(), "Butler".to_string(), "Clements".to_string(), "Williams".to_string(), "Brown".to_string(), "Hensley".to_string(), "Kirby".to_string(), "Wilkinson".to_string(), "Moody".to_string()]),
                ("RN".to_string(), vec!["I".to_string(),"II".to_string(), "III".to_string(),"IV".to_string(),"V".to_string(),"VI".to_string(),"VII".to_string(),"VIII".to_string(),"IX".to_string(),"X".to_string(),"XI".to_string()])

            ])
        }
    }
}
impl CFG {
    pub fn create_sentence(&self, symbol: String) -> String {
        let rule = self.rules.get(&symbol).expect("Incorrect Symbol");
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(0..rule.len());
        let start = rule[random_choice].clone().to_string();
        let mut cont: Vec<String> = start.split_whitespace().map(str::to_string).collect();
        // let mut new_sentence: String = cont.join(" ");
        let mut continue_loop = true;
        while continue_loop {
            // println!("call", );
            // for i in &cont {
            //     println!("{}={}", self.rules.contains_key(&i.to_string()));
            // }
            for index in 0..cont.len() {
                if self.rules.contains_key(&cont[index][..]) {
                    let options = self.rules.get(&cont[index][..]).unwrap();
                    let random_choice:usize = rng.gen_range(0..options.len());
                    cont[index] = options[random_choice].clone();
                    continue_loop = true

                } else {
                    continue_loop = false
                }
            }
            cont = cont.join(" ").split_whitespace().map(str::to_string).collect();

        }
        return cont.join(" ")
    }
}
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
        //             println!("Pushed".to_string() );
        //         } 
        //         if (widgets::Button::new("Take Item")
        //         .position(vec2(0.,25.))
        //         .ui(ui)) {
        //             println!("Pushed".to_string(), );
        //         } 
        // });
        // println!("still fine".to_string(), );
        
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
    _content: [Item; INVENTORY_SPACE],
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            _content: [Item::Empty; INVENTORY_SPACE],
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

