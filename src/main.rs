use oxidised4::{bored::GameState, gamedata::Disk};
use raylib::prelude::*;
const NROW: i32 = 6;
const NCOL: i32 = 7;
const BOARDSTART: (i32, i32) = (0, 0);

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    //images
    let board_image = Image::load_image("resouces/board.png").expect("WHAT DA HAILLL");
    let _ = rl
        .load_texture(&thread, "resouces/board.png")
        .expect("couldn't load texture :(");
    let _ = rl
        .load_texture(&thread, "resouces/bracc.png")
        .expect("couldn't load texture :(");
    let circle_image = Image::load_image("resouces/bracc.png").expect("WHAT DA HAILLL");
    let _ = rl
        .load_texture(&thread, "resouces/bracc.png")
        .expect("couldn't load texture :(");
    //textures
    let board_texture = rl
        .load_texture_from_image(&thread, &board_image)
        .expect("WHAT DA HAILL");

    let circle_texture = rl
        .load_texture_from_image(&thread, &circle_image)
        .expect("WHAT DA HAILL");
    let square_widf = board_texture.width / NCOL;
    dbg!(square_widf);
    let square_heif = board_texture.height / NROW;
    let _square_wewant = (square_widf * NROW / 2, square_heif * 3 / 2);
    let _square_center = square_widf / 2;
    //7,9 are the values to center the circle
    let mut state: GameState = GameState::default();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let column = get_mouse_column(&d, square_widf);
            let coords = get_circle_coords(1, column);
            state.circles.push((coords.1, coords.0, Disk::RED));
        }
        for circle in &state.circles {
            let (x, y, disk) = circle;
            let color = match disk {
                Disk::RED => Color::RED,
                Disk::BLU => Color::BLUE,
                Disk::EMPTY => Color::WHITE,
            };
            d.draw_texture(&circle_texture, *x, *y, color);
        }
        d.clear_background(Color::WHITE);
        d.draw_texture(&board_texture, BOARDSTART.0, BOARDSTART.1, Color::VIOLET);
    }
}

//TODO move this to a struct
const STARTX: i32 = 9;
const STARTY: i32 = 7;
const WX: i32 = 14;
const WY: i32 = 14;
const CIRCLEWIDTH: i32 = 56;
fn get_circle_coords(x: i32, y: i32) -> (i32, i32) {
    let mut returned: (i32, i32) = (STARTX, STARTY);
    match x {
        1 => {}
        _ => {
            returned.0 = STARTX + (CIRCLEWIDTH * (x - 1)) + (WX * (x - 1));
        }
    };
    match y {
        1 => {}
        _ => {
            returned.1 = STARTY + (CIRCLEWIDTH * (y - 1)) + (WY * (y - 1));
        }
    };
    returned
}
fn get_mouse_column(rl: &RaylibHandle, sw: i32) -> i32 {
    //row,col return
    let mouse_pos = rl.get_mouse_x();
    dbg!(mouse_pos);
    for num in 1..NCOL + 1 {
        dbg!(mouse_pos < sw * (num) - STARTX);
        if (mouse_pos > sw * (num - 1) + STARTX) && (mouse_pos < sw * (num) - STARTX) {
            dbg!(num);
            return num;
        }
    }
    -100
}
