use std::sync::Mutex;
use crate::position::*;
use crate::search::*;

lazy_static! {
    pub static ref G_THREAD : Mutex<Thread> = {
        let pos = Position::new();
        let th = Thread::new(pos);
        Mutex::new(th)
    };
}

pub fn game() {
    
    for _ in 0..1 {
        let pos = Position::init_sfen(START_SFEN);
        println!("{}",pos);
        let mut th_lock = G_THREAD.lock().unwrap();
        //th_lock.pos = pos;
        th_lock.nodes = 0;
        drop(th_lock);
        think(&pos);
        let th_lock = G_THREAD.lock().unwrap();
        println!("move : {} nodes: {}",th_lock.best_move.mv,th_lock.nodes);
        //th.pos.do_move(th.best_move.mv);
    }
}