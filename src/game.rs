use std::sync::Mutex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
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
        let mut mv_string = "".to_string();
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
            println!("move : {} nodes: {}",th_lock.best_move.mv.to_usi(),th_lock.nodes);
            if th_lock.best_move.mv == Move::MOVE_NONE {
                println!("end");
                if pos.turn() == Color::BLACK {
                    result.1 += 1;
                    mv_string = mv_string + "WHITE";
                } else {
                    result.0 += 1;
                    mv_string = mv_string + "BLACK";
                }
                break;
            }
            mv_string = mv_string + &th_lock.best_move.mv.to_usi() + " ";
            pos = pos.do_move(th_lock.best_move.mv);
            let sp = th_lock.stack_sp as usize;
            th_lock.stack[sp].mv = th_lock.best_move.mv;
            th_lock.stack_sp += 1;
        }
        if mv_string.len() != 0 {
            let mut file = OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open("record.sfen")
                                .expect("cannot open file");
            write!(file, "{}\n", mv_string).expect("cannot write file");
            file.flush().unwrap();
            println!("{}",mv_string);
        }
        drop(th_lock);
    }
}