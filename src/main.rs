
pub const CELL_SIZE: f32 = 16.;
// pub mod BSPMapGeneration;
// use crate ::BSPMapGeneration::*;
pub mod map;
use crate ::map::*;
pub mod bit_masking;
pub mod control;
use crate ::control::*;
use macroquad::prelude::*;
pub mod decorate;
pub mod bsp_tree_map_generation;
use crate ::bsp_tree_map_generation::*;
use std::collections::HashMap;
pub mod interaction;
use crate ::interaction::*;
pub mod player;
use crate ::player::*;
extern crate rand;
use ::rand::Rng;
pub mod question_gen;
use crate ::question_gen::*;
// use std::thread;
use std::time::SystemTime;
pub mod traits;
use crate ::traits::*;
// use pathfinding::prelude::dijkstra;

fn draw_mobs(mobs: &Vec<Entity>, textures: [[Texture2D;4];4], bar_texture: Texture2D) {
    for mob in mobs{
        let mob_index = match mob.entity_type {
            EntityType::Vampire => 0,
            EntityType::Skelly => 1,
            EntityType::Skelly2 => 2,
            EntityType::Skull => 3,
        };
        mob.draw_entity(textures[mob_index][mob.frame],bar_texture);
    }
}



fn set_spawn(map: &Vec<Vec<AdvanceTileTypes>>) -> (i16,i16) {
    let mut rng = rand::thread_rng();
    let mut x: usize = rng.gen_range(0..WORLD_SIZE.0);
    let mut y: usize = rng.gen_range(0..WORLD_SIZE.1);
    let mut spawn = map[y][x];
    while !matches!(spawn,AdvanceTileTypes::GenericFloor) {
        x = rng.gen_range(0..WORLD_SIZE.0); 
        y = rng.gen_range(0..WORLD_SIZE.1);
        spawn = map[y][x];
    }
    return (x as i16,y as i16)
}
    
pub fn create_mobs(number: i8,map: &Vec<Vec<AdvanceTileTypes>>) -> Vec<Entity>{
    let mut all_entity = Vec::new();
    for _ in 0..number {
        let spawn = set_spawn(map);
        all_entity.push(Entity::intialise(5, 5, 5, 5., Coordinates {x:spawn.0,y:spawn.1}));
    } 
    return all_entity
}    
#[macroquad::main("differ-geon-11")]
async fn main() {
    let target = (0.,0.);
    let zoom = 0.005;
    let smooth_rotation: f32 = 180.;
    let texture: Texture2D = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/sheet.png"), Some(ImageFormat::Png)));
    texture.set_filter(FilterMode::Nearest);
    let mut map2 = MapLayout::default();
    let mut current_player = set_spawn(&map2.tile_placement);
    let mut offset = (current_player.0 as f32 *zoom, current_player.1 as f32*zoom);
    map2.tile_decorate();
    let mut current_state = States::Play;
    let mut sub_states = [States::Play; 3];
    let mut all_storage: HashMap<(i16,i16),Storage> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut selected = (0,0);
    let mut player = PlayerCharacter::intialise(20,10,10,30.,Coordinates {
        x:current_player.0,
        y:current_player.1
    });
    let mut black_list: Vec<(i16,i16)> = Vec::new();
    let found_path: Option<(Vec<(i32, i32)>, u32)> = None;
    let mut mobs = create_mobs(7,&map2.tile_placement);
    let hud = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/hud-pieces.png"),Some(ImageFormat::Png)));
    hud.set_filter(FilterMode::Nearest);
    // let menu = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/cave.png"),Some(ImageFormat::Png)));
    // menu.set_filter(FilterMode::Nearest);
    let mob_textures: [[Texture2D;4];4] = [[Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/vampire_v1_1.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/vampire_v1_2.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/vampire_v1_3.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/vampire_v1_4.png"),Some(ImageFormat::Png)))],
    [Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton_v1_1.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton_v1_2.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton_v1_3.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton_v1_4.png"),Some(ImageFormat::Png)))],
    [Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton2_v1_1.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton2_v1_2.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton2_v1_3.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skeleton2_v1_4.png"),Some(ImageFormat::Png)))],
    [Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skull_v1_1.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skull_v1_2.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skull_v1_3.png"),Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Mob/skull_v1_4.png"),Some(ImageFormat::Png)))]];
    for mob_texture in mob_textures {
        for texture in mob_texture {
            texture.set_filter(FilterMode::Nearest);
        }
    }
    let player_texutres: [Texture2D;4] = [Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_1.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_2.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_3.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_4.png"), Some(ImageFormat::Png)))];
    for each in player_texutres {
        each.set_filter(FilterMode::Nearest);
    }
    let _selection_hand: [Texture2D;4] = [Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_1.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_2.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_3.png"), Some(ImageFormat::Png))),
    Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/Priest/priest1_v2_4.png"), Some(ImageFormat::Png)))];
    for each in player_texutres {
        each.set_filter(FilterMode::Nearest);
    }
    let mut period = SystemTime::now();
    let mut question = Question::default();
    question.create("eigen value");
    let cfg = CFG::default();
    let mut mob_shift_count = 0;
    loop {        
        if !matches!(current_state,States::Menu) {
            
            
            if is_key_pressed(KeyCode::P) {
                println!("{}",cfg.create_sentence("S".to_string()));
            }
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
                    _ => States::Empty
                };
                
                if !matches!(state,States::Empty) {
                    current_state = state;
                }
            }
        

        //Main loop
        if matches!(current_state,States::Play) {
            if 0.5 < (period.elapsed().unwrap().subsec_nanos() as f32/100000000.) {
                player.update_player_frame();
                mob_shift_count += 1;
                for mob in &mut mobs {
                    mob._update_entity_frame()
                }
                if mob_shift_count == 4 {
                    for mob_index in 0..mobs.len() {
                        let exculde_self:  Vec<Coordinates<i16>> = mobs.clone().into_iter().map(|mob| mob.cor).collect();
                        if !mobs[mob_index].consider_action(&map2.tile_placement,player.cor,&exculde_self).is_none(){
                            let life_state = player.health.adjust(0);
                            if !life_state.is_none() {
                                current_state = States::Menu;
                            }
                        }
                        // println!("NM = {},{}", mob.cor.x,mob.cor.y);
                    }
                    mob_shift_count = 0
                }
                period = SystemTime::now();
            }
            map2.draw_map(texture);
            draw_mobs(&mobs,mob_textures,hud);
            draw_rectangle_lines(abs(x) as f32 *CELL_SIZE,abs(y) as f32 * CELL_SIZE,CELL_SIZE,CELL_SIZE,3.,GOLD);
            if matches!(sub_states[0],States::Play) && matches!(sub_states[1],States::Play) {
                let mut movement = Movement {
                    up: is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up),
                    down: is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down),
                    left: is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left),
                    right: is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right),
                };
                let shift = movement.vector_movement();
                let screen_shift = ScreenMovement::default();
                let screen_shift = screen_shift.mouse_follow();
                    offset.0 += (screen_shift.0) * zoom;
                    offset.1 += (screen_shift.1) * zoom;
                if shift.0 != 0 || shift.1 != 0 {
                    if movement.movement_character(&map2.tile_placement,((current_player.0-shift.1) as usize,(current_player.1-shift.0) as usize)) {
                        current_player.0 -= shift.1;
                        current_player.1 -= shift.0;
                        offset.0 = (current_player.0) as f32 * CELL_SIZE * zoom;
                        offset.1 = (current_player.1) as f32 * CELL_SIZE * zoom * screen_width()/screen_height();
                    }
                }
            }
            // draw_rectangle(current_player.0 as f32 *CELL_SIZE,current_player.1 as f32 *CELL_SIZE,CELL_SIZE,CELL_SIZE,RED);
        player.draw_player(player_texutres[player.frame as usize]);
        if !&found_path.is_none() {
            for tile_move in &found_path.clone().unwrap().0 {
                draw_rectangle(tile_move.0 as f32 * CELL_SIZE, tile_move.1 as f32 * CELL_SIZE,CELL_SIZE,CELL_SIZE, RED);
            }
        }



        set_default_camera(); 





        let store_check = match map2.tile_placement[current_player.1 as usize][current_player.0 as usize] {
            AdvanceTileTypes::SmallCHest => 1,
            AdvanceTileTypes::Chest => 0,
            _ => 2
        };
        let _sub_state_one = sub_states[1];
        if !matches!(States::Question, _sub_state_one) {
            question.user_answer = ask_question(&question,&question.user_answer);
            println!("{}", question.user_answer);
            if is_key_released(KeyCode::Backspace) {
                question.user_answer = question.user_answer.remove_last();
            } 
        }
       

        if question.user_answer.eq("true") {
            println!("correct", );
        }
        //load store for player: store is any storage
        if store_check != 2 {
            if !all_storage.contains_key(&current_player){
                let storage = Storage::default();
                all_storage.insert(current_player,storage);
            }
            // let alt_state = all_storage[&current_player].alt_state;
            if is_key_pressed(all_storage[&current_player].key) {
                sub_states[0] = match sub_states[0] {
                    States::Storage => States::Play,
                    _ => States::Storage,
                }
            }
            if matches!(sub_states[0],States::Storage) {
                if !black_list.iter().any(|&i| i==current_player) {
                    println!("{}", &all_storage[&current_player].used);
                let pull_item = all_storage[&current_player].clone().display();
                if !pull_item.is_none(){
                    player.storage.storage.push(pull_item.unwrap());
                    black_list.push(current_player);
                    sub_states[0] = States::Play;
                }
            }
            }
        }
        // match states of inventory
        if is_key_pressed(KeyCode::I){
            sub_states[1] = match sub_states[1] {
                States::Inventory => States::Play,
                _ => States::Inventory,
            }
        }
        // display inventory
        if matches!(sub_states[1],States::Inventory) {
            player.storage.display_inventory();
        }
        player.health.draw_health(hud,screen_width()/50.,screen_height()/50., vec2(20.,40.),true);
    //Mouse Movement and finding of mouse postion to cell
    y = (((((mouse_position_local()[1] + offset.1)*(screen_height()/screen_width()))/zoom) as i16 | 15)+1)/16 -1;
    x = -((((mouse_position_local()[0] - offset.0)/zoom) as i16 | 15)+1)/16 ;
    player.cor.x = current_player.0;
    player.cor.y = current_player.1;
    if is_mouse_button_down(MouseButton::Left) {
        let position_of_mob = mobs.iter().position(|mob| mob.cor.x == x && mob.cor.y == y);
        if !position_of_mob.is_none() {
            // As question Function goes in here
            if !mobs[position_of_mob.unwrap()].health.adjust(-1).is_none() {
                mobs.remove(position_of_mob.unwrap());
            }
        }
        if match  map2.tile_placement[abs(y) as usize][abs(x) as usize] {
            AdvanceTileTypes::Void => true,
            _ => true,
        } {
            sub_states[2] = States::OptionInfo;
            selected = (x,y)
            
        }
        
    }
    if selected != (x,y) {
        sub_states[2] = States::Play;
    }
        if x < WORLD_SIZE.0 as i16 && y < WORLD_SIZE.1 as i16{
            draw_text(&(&map2.tile_placement[abs(y) as usize][abs(x) as usize].to_string())[..], 40., screen_height()-40.,40., BLUE);
        }
    }
    } else {
        // draw_texture_ex(menu,0.,0.,WHITE, DrawTextureParams {
        //     dest_size: Some(vec2(screen_width(),screen_height())),
        //     ..Default::default()
        // })
        
    }
        
        next_frame().await
        }
    }
