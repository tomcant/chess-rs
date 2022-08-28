mod attacks;
mod board;
mod colour;
mod fen;
mod game;
mod r#move;
mod movegen;
mod piece;
mod square;

fn main() {
    let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game_state = start_fen.parse::<game::GameState>().unwrap();

    println!("{game_state:?}");
}
