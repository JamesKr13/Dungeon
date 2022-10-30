pub const CELL_SIZE: f32 = 16.;
// pub mod BSPMapGeneration;
// use crate ::BSPMapGeneration::*;
pub mod map;
use crate::map::{AdvanceTileTypes, MapLayout};
pub mod bit_masking;
pub mod control;
use crate::control::{Movement, ScreenMovement, States};
use macroquad::prelude::*;
pub mod bsp_tree_map_generation;
use crate::bsp_tree_map_generation::{abs, WORLD_SIZE};
use std::collections::HashMap;
pub mod interaction;
use crate::interaction::{Storage, CFG};
pub mod player;
use crate::player::{Coordinates, Entity, EntityType, PlayerCharacter};
extern crate rand;
use ::rand::Rng;
pub mod question_gen;
use crate::question_gen::{ask_question, Question};
use macroquad::ui::{hash, root_ui, widgets, Skin, Style};
use std::time::SystemTime;
pub mod traits;
use crate::traits::RemoveLast;
pub mod tunelling;
use crate ::tunelling::*;
use std::fs;

const MAX_LEVEL: usize = 1;
// use pathfinding::prelude::dijkstra;

fn draw_mobs(mobs: &Vec<Entity>, textures: [[Texture2D; 4]; 4], bar_texture: Texture2D) {
    for mob in mobs {
        let mob_index = match mob.entity_type {
            EntityType::Vampire => 0,
            EntityType::Skelly => 1,
            EntityType::Skelly2 => 2,
            EntityType::Skull => 3,
        };
        mob.draw_entity(textures[mob_index][mob.frame], bar_texture);
    }
}
fn set_ui() {
    let skin = {
        let window_style = root_ui()
        .style_builder()
        // .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .color(Color::from_rgba(43, 44, 51,255))
        .color_inactive(Color::from_rgba(43, 44, 51,255))
        // .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .build();
        let label_style = root_ui()
        .style_builder().text_color(Color::from_rgba(176, 179, 194,255))
        // .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .color(Color::from_rgba(43, 44, 51,255))
        .font_size(25)
        // .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .build();
        let button_style = root_ui()
        .style_builder()
        .font_size(25)
        // .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .color_clicked(Color::from_rgba(43, 44, 51,255))
        .text_color_clicked(Color::from_rgba(176, 179, 194,255))
        .text_color(Color::from_rgba(43, 44, 51,255))
        .color(Color::from_rgba(176, 179, 194,255))
        // .text_color_clicked(Color::from_rgba(43, 44, 51,255))
        // .color_selected(Color::from_rgba(176, 179, 194,255))
        // .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .build();
        Skin {
            button_style,
            window_style,
            label_style,
            ..root_ui().default_skin()
        }
    };
    root_ui().push_skin(&skin);
}
fn set_spawn(map: &Vec<Vec<AdvanceTileTypes>>) -> (i16, i16) {
    let mut rng = rand::thread_rng();
    let mut x: usize = rng.gen_range(0..WORLD_SIZE.0);
    let mut y: usize = rng.gen_range(0..WORLD_SIZE.1);
    let mut spawn = map[y][x];
    while !matches!(spawn, AdvanceTileTypes::GenericFloor) {
        x = rng.gen_range(0..WORLD_SIZE.0);
        y = rng.gen_range(0..WORLD_SIZE.1);
        spawn = map[y][x];
    }
    (x as i16, y as i16)
}

#[must_use]
pub fn create_mobs(number: i8, map: &Vec<Vec<AdvanceTileTypes>>, level: usize) -> Vec<Entity> {
    let mut all_entity = Vec::new();
    let mut rng = rand::thread_rng();
    let min = (((4*(level+1))/2) as f32).ceil() as i16;
    let max = (((8*(level+1))/2) as f32).ceil() as i16;
    for _ in 0..number {
        let spawn = set_spawn(map);
        all_entity.push(Entity::intialise(
            rng.gen_range(min..max),
            rng.gen_range(min-1..max-2),
            rng.gen_range(min-1..max-2),
            5.,
            Coordinates {
                x: spawn.0,
                y: spawn.1,
            },
        ));
    }
    all_entity
}
#[macroquad::main("differ-geon-11")]
async fn main() {
    let target = (0., 0.);
    let zoom = 0.005;
    let smooth_rotation: f32 = 180.;
    let texture: Texture2D = Texture2D::from_image(&Image::from_file_with_format(
        include_bytes!("../lib/sheet.png"),
        Some(ImageFormat::Png),
    ));
    texture.set_filter(FilterMode::Nearest);
    let mut map2 = MapLayout::default();
    let mut current_player = set_spawn(&map2.tile_placement);
    map2.tile_decorate();
    let mut offset = (
        f32::from(current_player.0) * zoom,
        f32::from(current_player.1) * zoom,
    );
   
    let mut current_state = States::Play;
    let mut sub_states = [States::Play; 3];
    
    let mut x = 0;
    let mut y = 0;
    let mut selected = (0, 0);
    let mut player = PlayerCharacter::intialise(
        20,
        5,
        5,
        30.,
        Coordinates {
            x: current_player.0,
            y: current_player.1,
        },
        15,
    );
    let mut kill_count = 0;
    let mut level_count = 0;
    let mut question_count = 0;
    let mut position_of_mob: Option<usize> = None;
    let mut black_list: Vec<(i16, i16)> = Vec::new();
    let mut all_storage: HashMap<(i16, i16), Storage> = HashMap::new();
    let mut mobs = create_mobs(12, &map2.tile_placement,level_count);
    // let mut hight_scores: String = fs::read("../lib/Score.txt").expect("Unable to read file").into_iter().map(|x| x.to_string()).collect();
    // let high_score_data: Vec<&str> = hight_scores.split("|").collect();
    let hud = Texture2D::from_image(&Image::from_file_with_format(
        include_bytes!("../lib/hud-pieces.png"),
        Some(ImageFormat::Png),
    ));
    hud.set_filter(FilterMode::Nearest);
    // let menu = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/cave.png"),Some(ImageFormat::Png)));
    // menu.set_filter(FilterMode::Nearest);
    
    let mut user_answer = String::new();
    let mob_textures: [[Texture2D; 4]; 4] = [
        [
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/vampire_v1_1.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/vampire_v1_2.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/vampire_v1_3.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/vampire_v1_4.png"),
                Some(ImageFormat::Png),
            )),
        ],
        [
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton_v1_1.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton_v1_2.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton_v1_3.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton_v1_4.png"),
                Some(ImageFormat::Png),
            )),
        ],
        [
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton2_v1_1.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton2_v1_2.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton2_v1_3.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skeleton2_v1_4.png"),
                Some(ImageFormat::Png),
            )),
        ],
        [
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skull_v1_1.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skull_v1_2.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skull_v1_3.png"),
                Some(ImageFormat::Png),
            )),
            Texture2D::from_image(&Image::from_file_with_format(
                include_bytes!("../lib/Mob/skull_v1_4.png"),
                Some(ImageFormat::Png),
            )),
        ],
    ];
    for mob_texture in mob_textures {
        for texture in mob_texture {
            texture.set_filter(FilterMode::Nearest);
        }
    }
    let player_texutres: [Texture2D; 4] = [
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/Priest/priest1_v2_1.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/Priest/priest1_v2_2.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/Priest/priest1_v2_3.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/Priest/priest1_v2_4.png"),
            Some(ImageFormat::Png),
        )),
    ];
    for each in player_texutres {
        each.set_filter(FilterMode::Nearest);
    }
    let torch_texture: [Texture2D; 4] = [
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/torch/candlestick_1_1.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/torch/candlestick_1_2.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/torch/candlestick_1_3.png"),
            Some(ImageFormat::Png),
        )),
        Texture2D::from_image(&Image::from_file_with_format(
            include_bytes!("../lib/torch/candlestick_1_4.png"),
            Some(ImageFormat::Png),
        )),
    ];
    for each in player_texutres {
        each.set_filter(FilterMode::Nearest);
    }
    for each in torch_texture {
        each.set_filter(FilterMode::Nearest);
    }
    let mut period = SystemTime::now();
    let mut movement_period = SystemTime::now();
    let mut question = Question::default();
    question.create("eigen value");
    let cfg = CFG::default();
    let mut mob_shift_count = 0;
    let mut moving = false;
    let exit = set_spawn(&map2.tile_placement);
    map2.exit = (exit.0 as i32,exit.1 as i32);
    set_ui();
    loop {
        // println!("{}", cfg.create_sentence("S".to_string()));
        if !matches!(current_state, States::Menu) {
            set_camera(&Camera2D {
                target: vec2(target.0, target.1),
                rotation: smooth_rotation,
                zoom: vec2(zoom, zoom * screen_width() / screen_height()),
                offset: vec2(offset.0, offset.1),
                ..Default::default()
            });
            if is_key_pressed(KeyCode::Escape) {
                let state = match current_state {
                    States::Play => States::Pause,
                    States::Pause => States::Play,
                    _ => States::Empty,
                };

                if !matches!(state, States::Empty) {
                    current_state = state;
                }
            }

            //Main loop
            if matches!(current_state, States::Play) && level_count != MAX_LEVEL{
                map2.draw_map(texture,torch_texture);
                draw_mobs(&mobs, mob_textures, hud);
                if is_key_pressed(KeyCode::I) {
                    sub_states[1] = match sub_states[1] {
                        States::Inventory => States::Play,
                        _ => States::Inventory,
                    }
                }
                
                player.draw_player(player_texutres[player.frame as usize]);
                if matches!(sub_states[1],States::Play) {
                if 1. < (period.elapsed().unwrap().subsec_nanos() as f32 / 100_000_000.) {
                    player.update_player_frame();
                    mob_shift_count += 1;
                    for mob in &mut mobs {
                        mob._update_entity_frame()
                    }
                    if mob_shift_count == 4 {
                        map2.frame_up();
                        if !moving {
                            player.stamina.adjust(1);
                        }
                        for mob_index in 0..mobs.len() {
                            let exculde_self: Vec<Coordinates<i16>> =
                                mobs.clone().into_iter().map(|mob| mob.cor).collect();
                                
                            if mobs[mob_index]
                                .consider_action(&map2.tile_placement, player.cor, &exculde_self)
                                .is_some()
                            {
                                let life_state = player.health.adjust(0);
                                
                                if life_state.is_some() {
                                    current_state = States::Menu;
                                }
                            }
                        }
                        mob_shift_count = 0
                    }
                    period = SystemTime::now();
                }
                
                draw_rectangle_lines(
                    f32::from(abs(x)) * CELL_SIZE,
                    f32::from(abs(y)) * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    3.,
                    GOLD,
                );
                if matches!(sub_states[0], States::Play) && matches!(sub_states[1], States::Play) && 1. < (movement_period.elapsed().unwrap().subsec_nanos() as f32 / 100_000_000.){
                    movement_period = SystemTime::now();
                    let mut movement = Movement {
                        up: is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
                        down: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
                        left: is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
                        right: is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
                    };
                    let shift = movement.vector_movement();
                    if (shift.0 != 0 || shift.1 != 0)
                        && movement.movement_character(
                            &map2.tile_placement,
                            (
                                (current_player.0 - shift.1) as usize,
                                (current_player.1 - shift.0) as usize,
                            ),
                        ) && player.stamina.points != 0
                    {
                        current_player.0 -= shift.1;
                        current_player.1 -= shift.0;
                        offset.0 = f32::from(current_player.0) * CELL_SIZE * zoom;
                        offset.1 = f32::from(current_player.1) * CELL_SIZE * zoom * screen_width()
                            / screen_height();
                        player.stamina.adjust(-1);
                        moving = true
                    } else {
                        moving = false;
                    }
                }
                let screen_shift = ScreenMovement::default();
                let screen_shift = screen_shift.mouse_follow();
                offset.0 += (screen_shift.0) * zoom;
                offset.1 += (screen_shift.1) * zoom;
                // draw_rectangle(current_player.0 as f32 *CELL_SIZE,current_player.1 as f32 *CELL_SIZE,CELL_SIZE,CELL_SIZE,RED);
            }
                set_default_camera();

                let store_check = match map2.tile_placement[current_player.1 as usize]
                    [current_player.0 as usize]
                {
                    AdvanceTileTypes::SmallCHest => 1,
                    AdvanceTileTypes::Chest => 0,
                    _ => 2,
                };
                
                //load store for player: store is any storage
                
                    // let alt_state = all_storage[&current_player].alt_state;
                    let interact_key = is_key_pressed(KeyCode::E);
                    if interact_key {
                        //Creates a new level
                        if player.cor.x == map2.exit.0 as i16 && player.cor.y == map2.exit.1 as i16  {
                            position_of_mob = None;
                            black_list= Vec::new();
                            all_storage= HashMap::new();
                            texture.set_filter(FilterMode::Nearest);
                            map2 = MapLayout::default();
                            current_player = set_spawn(&map2.tile_placement);
                            map2.tile_decorate();
                            mobs = create_mobs(12, &map2.tile_placement, level_count);
                            current_player = set_spawn(&map2.tile_placement);
                            let exit = set_spawn(&map2.tile_placement);
                            map2.exit = (exit.0 as i32,exit.1 as i32);
                            level_count += 1;
                            current_state = States::LevelScreen;
                            period = SystemTime::now();
                        } else {
                            if store_check != 2 {
                                all_storage
                                    .entry(current_player)
                                    .or_insert_with(|| Storage::default());
                            sub_states[0] = match sub_states[0] {
                                States::Storage => States::Play,
                                _ => States::Storage,
                            };
                        }
                    }
                }
                    if matches!(sub_states[0], States::Storage)
                    && !black_list.iter().any(|&i| i == current_player)
                    {
                        let pull_item = all_storage[&current_player].clone().display();
                        if pull_item.is_some() {
                            player.storage.storage.push(pull_item.unwrap());
                            black_list.push(current_player);
                            sub_states[0] = States::Play;
                            
                        }
                    }
                // match states of inventory
                
                // display inventory
                if matches!(sub_states[1], States::Inventory) {
                    let effects = player.storage.display_inventory();
                    if effects.is_some() {
                        let stat_effects = effects.unwrap();
                        player.health.base_adjust(stat_effects[0]);
                        player.health.adjust(stat_effects[0]);
                        player.damage.adjust(stat_effects[1]);
                        player.stamina.adjust(stat_effects[2]);
                        player.stamina.base_adjust(stat_effects[2]);
                        player.health.adjust(stat_effects[3]);
                        player.stamina.adjust(stat_effects[4]);
                    }
                }
                player.health.draw_points(
                    hud,
                    screen_width() / 50.,
                    screen_height() / 50.,
                    vec2(20., 40.),
                    true,
                    true,
                );
                player.stamina.draw_points(
                    hud,
                    screen_width() / 50.,
                    screen_height() / 50.+40.,
                    vec2(20., 40.),
                    true,
                    false,
                );
                //Mouse Movement and finding of mouse postion to cell
                y = (((((mouse_position_local()[1] + offset.1)
                    * (screen_height() / screen_width()))
                    / zoom) as i16
                    | 15)
                    + 1)
                    / 16
                    - 1;
                x = -((((mouse_position_local()[0] - offset.0) / zoom) as i16 | 15) + 1) / 16;
                player.cor.x = current_player.0;
                player.cor.y = current_player.1;
                if is_mouse_button_down(MouseButton::Left) && abs(x) < WORLD_SIZE.0 as i16 && abs(y) < WORLD_SIZE.1 as i16{
                    if position_of_mob.is_none() {
                    position_of_mob =
                        mobs.iter().position(|mob| mob.cor.x == x && mob.cor.y == y);
                    }
                    if match map2.tile_placement[abs(y) as usize][abs(x) as usize] {
                        AdvanceTileTypes::Void => true,
                        _ => true,
                    } {
                        sub_states[2] = States::OptionInfo;
                        selected = (x, y)
                    }
                }
                if position_of_mob.is_some() || matches!(sub_states[1],States::Question){
                    if !matches!(sub_states[1],States::Question) {
                        question.user_answer = String::new();
                        println!("running", );
                    }
                    sub_states[1] = States::Question;
                   
                    if question.user_answer.eq("true") {
                        if mobs[position_of_mob.unwrap()].health.adjust(player.damage.deal(None)).is_some() {
                            mobs.remove(position_of_mob.unwrap());
                            kill_count += 1;
                        }
                        question.create("eigen value");
                        sub_states[1] = States::Play;
                        position_of_mob = None;
                        question.user_answer = String::new();
                        question_count += 1;
                    }
                    if question.user_answer.eq("false") {
                        question.create("eigen value");
                        sub_states[1] = States::Play;
                        position_of_mob = None;
                        question.user_answer = String::new();

                    }
                }
                let _sub_state_one = sub_states[1];
                if matches!(_sub_state_one,States::Question) {
                    question.user_answer = ask_question(&question, &question.user_answer);
                    if is_key_pressed(KeyCode::Backspace) {
                        if question.user_answer.len() != 0 {
                        question.user_answer = question.user_answer[0..question.user_answer.len()-1].to_string();
                        }
                    }
            }
                if selected != (x, y) {
                    sub_states[2] = States::Play;
                }
                if x < WORLD_SIZE.0 as i16 -1 && y < WORLD_SIZE.1 as i16 -1 {
                    draw_text(
                        &(&map2.tile_placement[abs(y) as usize][abs(x) as usize].to_string())[..],
                        40.,
                        screen_height() - 40.,
                        40.,
                        BLUE,
                    );
                }
            } else {
                 if level_count == MAX_LEVEL {

                    current_state = States::Victory;
                 }
                //  fs::write("../lib/Score", format!("{}|",MAX_LEVEL*(kill_count*5+question_count*10))).expect("Unable to write file");

            }
        } 
        if matches!(current_state,States::LevelScreen) {
            set_default_camera();

            draw_text(&format!("Floor {}/{}",level_count,MAX_LEVEL),screen_width()/2.-200.,screen_height()/2.,100.,WHITE);
            if period.elapsed().unwrap().as_secs() > 1 {
                draw_text("- Complete -",screen_width()/2.-150.,screen_height()/2.+100.,50.,WHITE);
            }
            if period.elapsed().unwrap().as_secs() > 3 {
                current_state = States::Play;
            }
        }
        if matches!(current_state,States::Pause) {
            set_default_camera();

            draw_text("- Paused -",screen_width()/2.-200.,screen_height()/2.,100.,WHITE);
        }
        if matches!(current_state,States::Victory) {
            set_default_camera();

            draw_text("- Victory -",screen_width()/2.-200.,screen_height()/2.,100.,WHITE);
            draw_text(&format!("- Score {} -", MAX_LEVEL*(kill_count*5+question_count*10)),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // if high_score_data.len() != 0 {
            //     draw_text(&format!("- 1st Score {} -", high_score_data[0]),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // } else {
            //     draw_text(&format!("- 1st Score - -"),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // }
            
            // if high_score_data.len() > 1 {
            //     draw_text(&format!("- 2nd Score {} -", high_score_data[1]),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // } else {
            //     draw_text(&format!("- 2nd Score - -"),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // }
            // if high_score_data.len() > 2 {
            //     draw_text(&format!("- 3nd Score {} -", high_score_data[2]),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // } else {
            //     draw_text(&format!("- 3nd Score - -"),screen_width()/2.-100.,screen_height()/2.+100.,50.,WHITE);
            // }
            

        }
        next_frame().await
    }
}
