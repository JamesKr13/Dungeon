use super::control::States;
use super::player::Coordinates;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fmt;
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

#[derive(Clone, Copy)]
pub enum Items {
    Sword,
    Bow,
    Empty,
    Staff,
}
impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Items::Sword => write!(f, "Sword"),
            Items::Bow => write!(f, "Bow"),
            Items::Staff => write!(f, "Staff"),
            Items::Empty => write!(f, "Nothing"),
        }
    }
}
pub struct CFG {
    rules: HashMap<String, Vec<String>>,
}
impl Default for CFG {
    fn default() -> Self {
        Self {
            rules: HashMap::from([
                ("S".to_string(), vec!["SS Adj Type US WH".to_string(), "MV Type of Adj US WH Fro".to_string()]),
                ("WH".to_string(), vec!["by NA".to_string(), "for NA C".to_string(), "With the PR of NA C".to_string()]),
                ("PR".to_string(),vec!["blood".to_string(), "bones".to_string(), "eyebrows".to_string(), "facial hair".to_string(), "skeleton".to_string(), "ashes".to_string(), "toe nail clippings".to_string()]),
                ("SS".to_string(), vec!["the".to_string(), "a".to_string(), "there was a".to_string()]),
                ("Fro".to_string(), vec!["from the Pl".to_string(), "of the Pl".to_string(), "Title of Pl".to_string()]),
                ("Title".to_string(), vec!["king".to_string(), "begger".to_string(), "knight".to_string(), "manic".to_string(), "innkeep".to_string(), "banker".to_string(), "confused Age".to_string(), "irritaating Age".to_string(), "frustrated Age".to_string(), "theif".to_string(), "lord".to_string(), "lady".to_string(), "queen".to_string(), "ambigous frog".to_string()]),
                ("Age".to_string(), vec!["old Age".to_string(), "man".to_string(), "women".to_string(), "child".to_string(), "toddle".to_string(), "miiddle aged man".to_string(), "middle aged women".to_string(), "tween".to_string()]),
                ("US".to_string(), vec!["used".to_string(), "wielded".to_string(), "held".to_string(), "brandished".to_string(), "managed".to_string()]),
                ("Adj".to_string(), vec!["the bloodthirsty".to_string(), "desperate".to_string(), "gruesome".to_string(), "knightly".to_string(), "nightly".to_string(), "embrassed".to_string(), "wholesale".to_string(), "mass produced".to_string(), "juicy".to_string(), "scrawny".to_string(), "thin".to_string(), "nondescript".to_string(), "administrative".to_string(), "useless".to_string(), "the irratic Title".to_string(), "sometimes functional".to_string()]),
                ("C".to_string(), vec!["to Act AG the Group".to_string()]),
                ("Act".to_string(), vec!["defend".to_string(), "fend".to_string(), "war".to_string(), "attack".to_string(), "attack unannouced".to_string(), "make false accusations".to_string(), "act tough".to_string()]),
                ("AG".to_string(), vec!["against".to_string(), "for".to_string()]),
                ("Group".to_string(), vec!["GTheory EGroup C".to_string(), "Adj EGroup of Pl".to_string(), "Adj GTheory EGroup of Pl".to_string(), "Adj EGroup of Direc".to_string()]),
                ("GTheory".to_string(), vec!["dirty capatlist".to_string(), "communist".to_string(), "christen".to_string(), "cultist".to_string(), "philospohical".to_string()]),
                ("EGroup".to_string(), vec!["barberians".to_string(), "warrior".to_string(), "soldiers".to_string(), "YAX legions".to_string(), "MV armies".to_string(), "watch".to_string(), "YAX council".to_string()]),
                ("YAX".to_string(), vec!["hight".to_string(), "middle".to_string(), "low".to_string(), "slightly to the left".to_string(), "Direc".to_string(), "".to_string()]),
                ("Direc".to_string(), vec!["east Direc".to_string(),  "west Direc".to_string(), "south Direc".to_string(), "north Direc".to_string(),"".to_string()]),
                ("C".to_string(), vec!["the Adj".to_string()]),
                ("Pl".to_string(), vec!["Direc PlNa".to_string(), "Isles of PlNa".to_string(), "isalnd of PlNa".to_string(), "county of PlNa".to_string(), "suburb of PlNa".to_string(), "YAX city of PlNa".to_string(), "Direc cities of PlNa".to_string(), "towers of PlNa".to_string(), "PlNa forest".to_string(), "desert of PlNa".to_string(), "Adj of PlNa".to_string()]),
                ("PlNa".to_string(), vec!["Waycastle".to_string(), "Starapple".to_string(),  "Dracburgh".to_string(),  "Lagooncall".to_string(),  "Snakewall".to_string(),  "Pineshear".to_string(),  "Glassfalcon".to_string(),  "Bleakden".to_string(),  "Bymallow".to_string(),  "Hazelmount".to_string()]),
                ("NA".to_string(), vec!["FN MN LN".to_string(), "FN MN LN the NUM".to_string()]),
                ("MN".to_string(), vec!["FN".to_string()]),
                ("FN".to_string(),vec!["Leonie".to_string(),
                        "Louis".to_string(),
                        "Viola".to_string(),
                        "Noah".to_string(),
                        "Jane".to_string(),
                        "Sean".to_string(),
                        "Orlando".to_string(),
                        "Hollyn".to_string(),
                        "Benjamin".to_string(),
                        "Gwendolen".to_string(),
                        "Lucinda".to_string(),
                        "Annabel".to_string(),
                        "Daniel".to_string(),
                        "Elijah".to_string(),
                        "Devon".to_string(),
                        "Robert".to_string(),
                        "Ryder".to_string(),
                        "Grey".to_string(),
                        "Miriam".to_string(),
                        "Riley".to_string(),
                        "Abraham".to_string(),
                        "Anise".to_string(),
                        "Ellory".to_string(),
                        "Sutton".to_string(),
                        "Ray".to_string(),
                        "Karilyn".to_string(),
                        "Sue".to_string(),
                        "Blayne".to_string(),
                        "Lilibeth".to_string(),
                        "Rhett".to_string(),
                        "Naomi".to_string(),
                        "Carleen".to_string(),
                        "Robin".to_string(),
                        "Zane".to_string(),
                        "Dezi".to_string(),
                        "Fawn".to_string(),
                        "Kylie".to_string(),
                        "Chase".to_string(),
                        "Timothy".to_string(),
                        "Isaiah".to_string(),
                        "Amelia".to_string(),
                        "Jude".to_string(),
                        "Nicolas".to_string(),
                        "Marcellus".to_string(),
                        "Jackson".to_string(),
                        "Jasper".to_string(),
                        "Sharon".to_string(),
                        "Trey".to_string(),
                        "Dante".to_string(),
                        "Finn".to_string()]),
                    ("LN".to_string(),
                    vec![
                        "".to_string(),
                        "Franco".to_string(),
                        "Vazquez".to_string(),
                        "Cummings".to_string(),
                        "Dennis".to_string(),
                        "Noble MN".to_string(),
                        "Harrell".to_string(),
                        "Hickman".to_string(),
                        "Munoz".to_string(),
                        "Perez".to_string(),
                        "Sandoval".to_string(),
                        "Rasmussen".to_string(),
                        "Kaufman".to_string(),
                        "Cox".to_string(),
                        "Hart".to_string(),
                        "Robles".to_string(),
                        "Carson".to_string(),
                        "Hunt".to_string(),
                        "Stuart".to_string(),
                        "Frank".to_string(),
                        "Leach".to_string(),
                        "Garrett".to_string(),
                        "Gross".to_string(),
                        "Reeves".to_string(),
                        "Brady".to_string(),
                        "Gonzales".to_string(),
                        "Bradshaw".to_string(),
                        "Hartman".to_string(),
                        "Rivera".to_string(),
                        "Shaw".to_string(),
                        "Livingston".to_string(),
                        "Mcbride".to_string(),
                        "Zimmerman".to_string(),
                        "Bates".to_string(),
                        "Vaughan".to_string(),
                        "Schmitt".to_string(),
                        "Tanner".to_string(),
                        "Elliott".to_string(),
                        "Hayes".to_string(),
                        "Ritter".to_string(),
                        "Foster".to_string(),
                        "Gay".to_string(),
                        "Osborne".to_string(),
                        "Butler".to_string(),
                        "Clements".to_string(),
                        "Williams".to_string(),
                        "Brown".to_string(),
                        "Hensley".to_string(),
                        "Kirby".to_string(),
                        "Wilkinson".to_string(),
                        "Moody".to_string(),
                    ],
                ),
                (
                    "NUM".to_string(),
                    vec![
                        "I".to_string(),
                        "II".to_string(),
                        "III".to_string(),
                        "IV".to_string(),
                        "V".to_string(),
                        "VI".to_string(),
                        "VII who P Pl".to_string(),
                        "VIII".to_string(),
                        "IX".to_string(),
                        "X".to_string(),
                        "XI".to_string(),
                    ],
                ),
            ]),
        }
    }
}
impl CFG {
    #[must_use]
    pub fn create_sentence(&self, symbol: String) -> String {
        let rule = self.rules.get(&symbol).expect("Incorrect Symbol");
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(0..rule.len());
        let start = rule[random_choice].clone();
        let mut cont: Vec<String> = start.split_whitespace().map(str::to_string).collect();
        // let mut new_sentence: String = cont.join(" ");
        while cont.iter().any(|e| self.rules.contains_key(e)) {
            // println!("call".to_string(), );
            // for i in &cont {
            //     println!("{}={}".to_string(), self.rules.contains_key(&i.to_string()));
            // }
            for index in 0..cont.len() {
                if self.rules.contains_key(&cont[index][..]) {
                    let options = self.rules.get(&cont[index][..]).unwrap();
                    let random_choice: usize = rng.gen_range(0..options.len());
                    cont[index] = options[random_choice].clone();
                }
            }
            cont = cont
                .join(" ")
                .split_whitespace()
                .map(str::to_string)
                .collect();
        }
        cont.join(" ")
    }
}
#[derive(Default, Clone)]
pub struct ClickActions {
    pub run_state: bool,
    pub on_set_mouse_position: Coordinates<f32>,
}
impl ClickActions {
    pub fn walk_menu(&mut self, _set_pos: bool) {
        // root_ui().window(hash!(),vec2(pos.x,pos.y),vec2(50.,50.), |ui| {
        //         if (widgets::Button::new("Move Here")
        //         .position(vec2(mouse_position().0,mouse_position().1))
        //         .ui(ui)) {
        //             println!("Pushed".to_string(), );
        //         }
        //         if (widgets::Button::new("Take Item")
        //         .position(vec2(0.,25.))
        //         .ui(ui)) {
        //             println!("Pushed".to_string(), );
        //         }
        // });
        // println!("still fine".to_string(), );

        //     root_ui().window(hash!("Menu"), vec2(mouse_position().0,mouse_position().1), vec2(65., 40.), |ui| {
        //         if widgets::Button::new("Move Here")
        //         .position(vec2(0.,0.))
        //         .ui(ui) {
        //             self.run_state = false;
        //             println!("Pushed |.................................................................................................................|", );
        //             // self.click_action.run_state = true;
        //         }
        //         if widgets::Button::new("Take Item")
        //         .position(vec2(0.,20.))
        //         .ui(ui) {
        //             self.run_state = false;
        //             println!("Pushed |.................................................................................................................|", );
        //             // self.click_action.run_state = true;
        //         }
        // });
        // if set_pos {
        //     self.on_set_mouse_position = Coordinates {x:mouse_position().0,y:mouse_position().1};
        //     root_ui().move_window(hash!("Menu"),vec2(mouse_position().0,mouse_position().1));
        // }
        // }
    }
}
trait DrawText {
    fn draw_text_given_space(
        &self,
        x_cor: f32,
        y_cor: f32,
        width: f32,
        height: f32,
        font_size: f32,
        sentence: String,
    );
}
#[derive(Clone)]
pub struct Item {
    pub description: String,
    pub item_type: Items,
}
impl DrawText for Item {
    fn draw_text_given_space(
        &self,
        x_cor: f32,
        y_cor: f32,
        width: f32,
        _height: f32,
        font_size: f32,
        sentence: String,
    ) {
        let all_chars: Vec<char> = sentence.chars().collect();
        let dimensions = measure_text(&sentence[..], None, font_size as u16, 1.);
        let char_number = ((all_chars.len() / (dimensions.width / width).ceil() as usize) as f32)
            .floor() as usize;
        println!("{}", all_chars.len());
        println!(
            "{},{} so {}",
            dimensions.width, dimensions.height, char_number
        );
        let mut each_line: Vec<String> = Vec::new();
        let each_word: Vec<String> = sentence.split(' ').map(str::to_string).collect();
        let mut tally = 0;
        for word in each_word {
            if !each_line.is_empty() && tally + word.len() <= char_number {
                tally += word.len();
                let index = each_line.len() - 1;
                each_line[index].push_str(&format!(" {}", word)[..]);
            } else {
                tally = 0;
                each_line.push(word);
            }
        }
        //     if &each_line.len() != &0 && &each_line.last().expect("No Vec found").len() != &char_number {
        //         let index = each_line.len()-1;
        //         each_line[index].push(c);
        //     }
        //     else {
        //         each_line.push(c.to_string());
        //     }
        // }
        // let index = each_line.len();
        // let last = each_line[index-1].len();
        // if last != 8 {
        //     each_line[index-1].push_str(&(" ").repeat((char_number-last) as usize)[..]);
        // }
        for line_index in 0..each_line.len() {
            draw_text(
                &each_line[line_index][..],
                x_cor,
                y_cor + (font_size + font_size * line_index as f32),
                font_size,
                WHITE,
            );
        }
    }
}
impl Item {
    fn new(description: String) -> Self {
        let mut rng = rand::thread_rng();
        let random_item: usize = rng.gen_range(0..3);
        let d = description.replace(
            "Type",
            &(match random_item {
                0 => Items::Sword,
                1 => Items::Bow,
                2 => Items::Staff,
                _ => Items::Empty,
            })
            .to_string()[..],
        );
        Self {
            description: d,
            item_type: Items::Sword,
        }
    }
}
#[derive(Clone)]
pub struct Storage {
    pub items: Vec<Item>,
    pub key: KeyCode,
    pub alt_state: States,
    // inventory_skins: InventorySkins,
    pub used: bool,
}
impl Default for Storage {
    fn default() -> Self {
        let all_items = vec![
            Item::new(CFG::default().create_sentence("S".to_string())),
            Item::new(CFG::default().create_sentence("S".to_string())),
            Item::new(CFG::default().create_sentence("S".to_string())),
        ];
        // ivsk.create_inventory_skins(&all_items);
        Self {
            items: all_items,
            key: KeyCode::E,
            alt_state: States::Storage,
            // inventory_skins: ivsk,
            used: false,
        }
    }
}
impl Storage {
    pub fn display(&mut self) -> Option<Item> {
        let item: Option<Item> = None;
        let window_size = vec2(screen_width() / 3., 3. * screen_height() / 4.);
        // draw_rectangle(100.,100.,400.,400.,BLUE);
        draw_rectangle(
            screen_width() - window_size[0],
            0.,
            window_size[0],
            screen_height(),
            BLACK,
        );
        self.items[0].draw_text_given_space(
            screen_width() - window_size[0],
            200.,
            window_size[0],
            window_size[1] / 3.,
            20.,
            self.items[0].description.clone(),
        );
        self.items[0].draw_text_given_space(
            screen_width() - window_size[0],
            200. + window_size[1] / 3.,
            400.,
            400.,
            20.,
            self.items[1].description.clone(),
        );
        self.items[0].draw_text_given_space(
            screen_width() - window_size[0],
            200. + 2. * window_size[1] / 3.,
            window_size[0],
            window_size[1] / 3.,
            20.,
            self.items[2].description.clone(),
        );
        item
    }
}

pub struct Inventory {
    _content: [Items; INVENTORY_SPACE],
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            _content: [Items::Empty; INVENTORY_SPACE],
        }
    }
}
impl Inventory {
    pub fn display_inventory(&mut self) {
        for row in 0..COL_ROW_SIZE {
            for col in 0..COL_ROW_SIZE {
                draw_rectangle_lines(
                    screen_width() / 3. - col as f32 * 25.,
                    100. - row as f32 * 25.,
                    25.,
                    25.,
                    5.,
                    BLUE,
                )
            }
        }
    }
}
