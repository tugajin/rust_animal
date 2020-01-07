use rand::Rng;
use crate::position::*;
use crate::common::*;

pub static PIECE_SCORE : [Score ; Piece::PIECE_SIZE] = [
    Score(0),Score(100), Score(300), Score(300), Score(500) ,Score(16000)
];
pub static PIECE_EX_SCORE : [Score ; Piece::PIECE_SIZE] = [
    Score(PIECE_SCORE[0].0 * 2), Score(PIECE_SCORE[1].0 * 2), Score(PIECE_SCORE[2].0 * 2),
    Score(PIECE_SCORE[3].0 * 2), Score(PIECE_SCORE[4].0 + PIECE_SCORE[1].0), Score(PIECE_SCORE[5].0 * 1)]; 

pub fn eval(pos : &Position) -> Score {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let mut i: i32 = rng.gen();           // genはRng traitに定義されている
    i %= 10;
    //i = 0;
    let mut sc = pos.material() + Score(i);
    if pos.turn() == Color::WHITE {
        sc = -sc;
    }
    sc
    //Score::SCORE_NONE
}