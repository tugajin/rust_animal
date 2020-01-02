use rand::Rng;
use crate::position::*;
use crate::common::*;

pub fn eval(pos : &Position) -> Score {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let mut i: i32 = rng.gen();           // genはRng traitに定義されている
    i %= 100;
    Score(i)
    //Score::SCORE_NONE
}