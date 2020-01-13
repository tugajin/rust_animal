use crate::learn2::*;
use crate::position::*;
use crate::common::*;
use tch::{Tensor,nn, nn::Module, nn::OptimizerConfig, Device};

const THREAD_NUM : usize = 1;

pub struct LearnData {
    pub thread_id : usize,
    pos : Position,
    mv_list : Vec<Move>,
    r : f64,
}

const KIF_PATH : &str = "./record.sfen";

impl LearnData {
    
    
    pub fn new(thread_id : usize, pos : Position) -> LearnData {
        let ld = LearnData {
            thread_id : thread_id,
            pos : pos.clone(),
            mv_list : Vec::new(),
            r : 0.5,
        };
        ld
    }
    fn load_kif(&mut self, sfen : &str) {
        self.pos = Position::init_sfen(START_SFEN);
        let mv_vector: Vec<&str> = sfen.split(' ').collect();
        for mv_str in mv_vector.iter() {
            let mut next_move = Move::MOVE_NONE;
            if mv_str == &"BLACK" {
                self.r = 1.0;
            } else if mv_str == &"WHITE" {
                self.r = 0.0;
            } else {
                //println!("{}",mv_str.trim());
                let prefix = &mv_str[0..1];
                if prefix == "*" {//drop
                    let to_str = &mv_str[1..3];
                    let p_str = &mv_str[3..4];
                    let to = Square::from_str(to_str);
                    let p = Piece::from_str(p_str);
                    //println!("str |{}| |{}| : |{}|",prefix,to_str,p_str);
                    //println!("sq  |{}| |{}| : |{}|",prefix,to,p);
                    next_move = make_drop_move(to,p);
                } else {//move
                    let from_str = &mv_str[0..2];
                    let to_str = &mv_str[2..4];
                    let p_str = &mv_str[4..5]; 
                    let from = Square::from_str(from_str);
                    let to = Square::from_str(to_str);
                    let p = Piece::from_str(p_str);
                    let cap = self.pos.square(to).to_piece();
                    let prom = mv_str.len() >= 6;
                    //println!("{} len:{}",mv_str,mv_str.len());
                    //println!("str |{}| |{}| : |{}|",from_str,to_str,p_str);
                    //println!("sq  |{}| |{}| : |{}|",from,to,p);
                    next_move = make_move(from, to, p, cap, prom);
                }
            }
            if next_move == Move::MOVE_NONE {
                break;
            } else {
                self.mv_list.push(next_move);
                self.pos = self.pos.do_move(next_move);
                println!("mv:{}",next_move);
                println!("{}",self.pos);
            }
        }
    }
}

pub fn learn() {
    let pos = Position::init_sfen(START_SFEN);
    let mut learn_data : [LearnData ; THREAD_NUM] = [LearnData::new(0,pos.clone()) ; THREAD_NUM];
    for i in 0..THREAD_NUM {
        learn_data[i].thread_id = i;
    }
    learn_data[0].load_kif("B3B2H B1B2R *A2H A1A2K C4C3K *A1H C3C2K B2B1R C2C1K B1C1R A4B3Z *B1K *C2Z B1B2K B3A2Z A1A2H *B1K C1C2R B1B2K C2B2R *A1K *B1K A1A2K B2A2R *C1H A2B2R B4A4R *A1K A4B4R B1C1K B4A4R A1B1K A4B4R *A3H B4C4R A3A4H+ WHITE");
}