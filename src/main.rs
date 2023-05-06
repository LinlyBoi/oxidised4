use oxidised4::{
    bored::{GameState, MenuState, PlayState, Strategy},
    gamedata::{
        algorithms::{minimax_decision, minimax_decision_pruning},
        Disk,
    },
};
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
    let mut state: GameState = GameState::MainMenu(MenuState::default());
    let mut strategy = Strategy::MiniMax;
    let mut difficulty = 3;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        match &mut state {
            GameState::MainMenu(ref mut mstate) => {
                if mstate.init(&d) {
                    strategy = mstate.strategy.clone();
                    difficulty = mstate.difficulty.clone();
                    state = GameState::Play(PlayState::default());
                }
            }
            GameState::Play(ref mut state) => {
                match state.player_turn {
                    true => {
                        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                            if let Some(column) = get_mouse_column(&d, square_widf) {
                                state.play_human(column);
                            }
                        }
                    }
                    false => match strategy {
                        Strategy::MiniMax => state.play_cpu(minimax_decision),
                        Strategy::AlphaBeta => state.play_cpu(minimax_decision_pruning),
                    },
                }
                if state.board.game_over() {
                    let scores = state.board.getscore();
                    println!("Player score: {} \n CPU Score: {}", scores.0, scores.1);
                }

                for circle in state.clone().circles {
                    let (x, y, disk) = circle;
                    let color = match disk {
                        Disk::P1 => Color::RED,
                        Disk::P2 => Color::YELLOW,
                        Disk::EMPTY => Color::WHITE,
                    };
                    d.draw_texture(&circle_texture, x, y, color);
                }
                d.clear_background(Color::WHITE);
                d.draw_texture(&board_texture, BOARDSTART.0, BOARDSTART.1, Color::VIOLET);
            }
        }
    }
}

const STARTY: i32 = 9;
fn get_mouse_column(rl: &RaylibHandle, sw: i32) -> Option<i32> {
    //row,col return
    let mouse_pos = rl.get_mouse_x();
    dbg!(mouse_pos);
    for num in 1..NCOL + 1 {
        dbg!(mouse_pos < sw * (num) - STARTY);
        if (mouse_pos > sw * (num - 1) + STARTY) && (mouse_pos < sw * (num) - STARTY) {
            dbg!(num);
            return Some(num - 1);
        }
    }
    None
}
