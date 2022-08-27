mod attacks;
mod board;
mod colour;
mod game_state;
mod r#move;
mod move_generator;
mod piece;
mod square;

fn main() {
    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game_state = start_fen.parse::<game_state::GameState>().unwrap();

    println!("{game_state:?}");
}
