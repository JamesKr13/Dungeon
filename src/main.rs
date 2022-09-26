
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

#[macroquad::main("differ-geon-9")]
async fn main() {
    let mut target = (20.*CELL_SIZE,20.*CELL_SIZE);
    let mut zoom = 0.005;
    let mut rotation = 0.0;
    let mut smooth_rotation: f32 = 180.;
    let mut offset = (0., 0.);
    let texture: Texture2D = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../lib/sheet.png"), Some(ImageFormat::Png)));
    texture.set_filter(FilterMode::Nearest);
    let mut map2 = MapLayout::default();
    map2.tile_decorate();
    // for each in &map2.other_map {
    //     for tile in each {
    //         match tile {
    //             TileType::Floor => print!("0", ),
    //             TileType::Wall => print!("1", ),
    //         };
    //     }
    //     println!("", );
    // }
    let background_color = Color::new(36.,18.,26.,1.);
    loop {
        // clear_background(background_color);
        set_camera(&Camera2D {
            target: vec2(target.0, target.1),
            rotation: smooth_rotation,
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            offset: vec2(offset.0, offset.1),
            ..Default::default()
        });
    if is_key_pressed(KeyCode::Equal) {
        zoom *= 1.5;
    }
    if is_key_pressed(KeyCode::Minus) {
        zoom /= 1.5;
    }
    map2.draw_map(texture);
    let mut movement = Movement {
        up: is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        down: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
        left: is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
        right: is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
    };
    let shift = movement.vector_movement();
    println!("{}, {}", shift.0, shift.1);
    // if map2.check_future_move(((abs((((offset.1/zoom/16.).ceil()-1.)) as i16) + shift.0) as usize,(abs(((offset.0/zoom/16.).ceil() as i16) + shift.1) as usize))) {

    offset.1 += shift.0 as f32 * SPEED * zoom;
    offset.0 += shift.1 as f32 * SPEED * zoom;
    // }
    set_default_camera(); 
    draw_rectangle(screen_width()/2., screen_height()/2.,CELL_SIZE,CELL_SIZE,RED);
    next_frame().await
    }
    

}
