use raylib::prelude::*;
const NROW: i32 = 6;
const NCOL: i32 = 7;
fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let _rust_orange = Color::new(222, 165, 132, 255);
    let _ray_white = Color::new(255, 255, 255, 255);
    let board_image = Image::load_image("resouces/board.png").expect("WHAT DA HAILLL");
    let _ = rl
        .load_texture(&thread, "resouces/board.png")
        .expect("couldn't load texture :(");

    let board_texture = rl
        .load_texture_from_image(&thread, &board_image)
        .expect("WHAT DA HAILL");
    println!(
        "width: {} \n height: {}.",
        board_texture.width, board_texture.height
    );
    let square_widf = board_texture.width / NROW;
    let square_heif = board_texture.height / NCOL;
    let square_wewant = (square_widf * NROW / 2, square_heif * 3 / 2);
    let square_center = square_widf / 2;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let pressed_key = rl.get_key_pressed();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        //d.draw_circle_gradient(240, 240, 100 as f32, Color::ORANGE, Color::RED);
        d.draw_circle(square_wewant.0, square_wewant.1, 55.0, Color::YELLOW);
        d.draw_texture(&board_texture, 0, 0, Color::VIOLET);
        if let Some(pressed_key) = pressed_key {
            // Certain keyboards may have keys raylib does not expect. Uncomment this line if so.
            // let pressed_key: u32 = unsafe { std::mem::transmute(pressed_key) };
            d.draw_text(&format!("{:?}", pressed_key), 100, 12, 10, Color::BLACK);
        }
    }
}
