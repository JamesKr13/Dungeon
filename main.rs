
pub const CELL_SIZE: f32 = 16.;
// pub mod BSPMapGeneration;
// use crate ::BSPMapGeneration::*;
pub mod Map;
use crate ::Map::*;
pub mod BitMasking;
use crate ::BitMasking::*;
pub mod Control;
use crate ::Control::*;
use macroquad::prelude::*;
pub mod Decorate;
use crate ::Decorate::*;
pub mod BSPTreeMapGeneration;
use crate ::BSPTreeMapGeneration::*;
use macroquad::color::*;
use std::collections::HashMap;
pub mod Interaction;
use crate ::Interaction::*;
pub mod Player;
use crate ::Player::*;
extern crate rand;
use ::rand::Rng;

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

#[macroquad::main("differ-geon-11")]
async fn main() {
    let mut target = (0.,0.);
    let mut zoom = 0.005;
    let mut rotation = 0.0;
    let mut smooth_rotation: f32 = 180.;
    let mut offset = (2., 2.);
    let texture: Texture2D = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/sheet.png"), Some(ImageFormat::Png)));
    texture.set_filter(FilterMode::Nearest);
    let mut map2 = MapLayout::default();
    let mut current_player = set_spawn(&map2.tile_placement);
    map2.tile_decorate();
    let background_color = Color::new(36.,18.,26.,1.);
    let screen_world = (WORLD_SIZE.0 as f32*CELL_SIZE*zoom/2.,WORLD_SIZE.1 as f32 *CELL_SIZE*zoom);
    let mut current_state = States::Play;
    let mut sub_states = [States::Play; 3];
    let mut record =false;
    let mut all_storage: HashMap<(i16,i16),Storage> = HashMap::new();
    let mut inventory = Inventory::default();
    let mut action_key: Option<KeyCode>= None;
    let mut alt_state: States =  States::Empty;
    let mut draw_info = false;
    let mut x = 0;
    let  mut y = 0;
    let mut selected = (0,0);
    println!("{},{}", current_player.0,current_player.1);
    loop {
       


        if matches!(current_state,States::Menu) {
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
            if !matches!(current_state,States::Pause) {
                clear_background(GREEN);
            }
        if is_key_pressed(KeyCode::A) {

        }
        if !matches!(current_state,States::Play) {

            map2.draw_map(texture);
            draw_rectangle_lines(abs(x) as f32 *CELL_SIZE,abs(y) as f32 * CELL_SIZE,CELL_SIZE,CELL_SIZE,3.,GOLD);
            if matches!(sub_states[2],States::OptionInfo) {
                draw_rectangle_lines(abs(x) as f32 *CELL_SIZE + CELL_SIZE,abs(y) as f32 * CELL_SIZE,CELL_SIZE,CELL_SIZE,3.,BLACK);
            }
            if matches!(sub_states[0],States::Play) && matches!(sub_states[1],States::Play) && matches!(sub_states[2],States::Play) {
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
                        offset.1 = ((current_player.1) as f32 * CELL_SIZE * zoom);
                    }
                }
            }
            draw_rectangle(current_player.0 as f32 *CELL_SIZE,current_player.1 as f32 *CELL_SIZE,CELL_SIZE,CELL_SIZE,RED);
        }
        set_default_camera(); 
        let store_check = match map2.tile_placement[current_player.1 as usize][current_player.0 as usize] {
            AdvanceTileTypes::SmallCHest => 1,
            AdvanceTileTypes::Chest => 0,
            _ => 2
        };
        if store_check != 2 {
            if !all_storage.contains_key(&current_player){
                let storage = Storage::default();
                all_storage.insert(current_player,storage);
            }
            let alt_state = all_storage[&current_player].alt_state;
            if is_key_pressed(all_storage[&current_player].key) {
                sub_states[0] = match sub_states[0] {
                    States::Storage => States::Play,
                    _ => States::Storage,
                }
            }
            if matches!(sub_states[0],States::Storage) {
                all_storage[&current_player].clone().display();
            }
        }
        if is_key_pressed(KeyCode::I) {
            sub_states[1] = match sub_states[1] {
                States::Inventory => States::Play,
                _ => States::Inventory,
            }
        }
        if matches!(sub_states[1],States::Inventory) {
            inventory.display_inventory();
        }
    }   if record {
        println!("{:#?}", mouse_position());
    }
    if is_key_pressed(KeyCode::R) {
        record = !record;
    }
    // if is_key_pressed(KeyCode::D) {
    //     let mouse_position
    //     draw_rectangle()
    // }
    //0.5444 shifts the local y position of the cursor to is canvas value e.g. the index in the map placement
    //3.92 is the target value produced by minusing the offset from (0,0) or in this case (0,600) mutliply it by the zoom
    //value and using a bitwise operator to round up to the nearest multiple of 16 then dividing and shaving the value
    //to give an accurate mouse position.
    // Note: functioning const of 0.5444 is only functional when screen size is (1366,784)
    //       5.4444 only function due to the margin of error created by rounding up.
    y = (((((mouse_position_local()[1] + offset.1)*0.544)/zoom) as i16 | 15)+1)/16 -1;
    x = -((((mouse_position_local()[0] - offset.0)/zoom) as i16 | 15)+1)/16 ;
    if is_mouse_button_down(MouseButton::Left) {
        if match  map2.tile_placement[abs(y) as usize][abs(x) as usize] {
            AdvanceTileTypes::Skull => true,
            _ => false,
        } {
            sub_states[2] = States::OptionInfo;
            selected = (x,y)
        }
    }
    if selected != (x,y) {
        sub_states[2] = States::Play;
    }
        if is_key_pressed(KeyCode::Space) {
            println!("{:#?}", mouse_position_local());
            println!("{},{}", screen_height(),screen_width());
            println!("off set is {:#?}", offset);
            
            println!("{},{}", x,y);
            // current_player = (((((offset.0/zoom) as i16 |15) +1)/16) as i16,((((offset.1/zoom) as i16 |15) +1)/16) as i16);
            // println!("{},{}", current_player.0 as f32 * CELL_SIZE, current_player.1 as f32 * CELL_SIZE);
            // println!("{:#?}", map2.tile_placement[((((offset.1/zoom) as i16 |15) +1)/16) as usize][((((offset.0/zoom) as i16 |15) +1)/16) as usize]);
            // println!("cor are {:#?},{:#?}",((((offset.1/zoom) as i16 |15) +1)/16) as usize, ((((offset.0/zoom) as i16 |15) +1)/16) as usize);
            // draw_rectangle(((((offset.0/zoom) as i16 |15) +1)) as f32, ((((offset.1/zoom) as i16 |15) +1)) as f32, 10.,10.,GREEN);
        }
        draw_text(&(&map2.tile_placement[abs(y) as usize][abs(x) as usize].to_string())[..], 40., screen_height()-40.,40., BLUE);
        draw_rectangle(screen_width()/2.,screen_height()/2.,50.,50.,GREEN);
        next_frame().await
        }
    }
