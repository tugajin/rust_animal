use std::sync::Mutex;
use crate::position::*;
use crate::search::*;
use crate::gen::*;
use crate::common::*;

lazy_static! {
    pub static ref G_THREAD : Mutex<Thread> = {
        let pos = Position::new();
        let th = Thread::new(pos);
        Mutex::new(th)
    };
}

pub fn game() {

    let mut result = (0,0,0);
    
    for _ in 0..10000 {
        let mut pos = Position::init_sfen(START_SFEN);
        let mut th_lock = G_THREAD.lock().unwrap();
        th_lock.stack_sp = 0;
        loop {
            println!("{}",pos);
            if th_lock.stack_sp > 300 { 
                result.2 += 1;
                break; 
            }
            //th_lock.pos = pos;
            th_lock.nodes = 0;
            th_lock.best_move = MoveSc::MOVE_NONE;
            th_lock.pos = pos.clone();
            th_lock.think();
            println!("result {} vs {} : {}",result.0,result.1,result.2);
            println!("move : {} nodes: {}",th_lock.best_move.mv,th_lock.nodes);
            if th_lock.best_move.mv == Move::MOVE_NONE {
                println!("end");
                if pos.turn() == Color::BLACK {
                    result.1 += 1;
                } else {
                    result.0 += 1;
                }
                break;
            }
            pos = pos.do_move(th_lock.best_move.mv);
            th_lock.stack_sp += 1;
        }
        drop(th_lock);
    }
}