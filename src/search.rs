use std::fmt;
use crate::common::*;
use crate::position::*;
use crate::gen::*;
use crate::attack::*;
use crate::eval::*;
use crate::game::*;

pub struct Thread {
    pub pos : Position,
    pub best_move : MoveSc,
    pub nodes : u64,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Not,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    PartialOrd,
    Ord,
)]
pub struct Depth(i32);
impl Depth {
    const MAX_PLY : i32 = 128;
    const DEPTH_INC : i32 = 1;
    pub const DEPTH_ZERO : Depth = Depth(0);
    pub const DEPTH_ONE  : Depth = Depth(Depth::DEPTH_INC);
    pub fn new(d : i32) -> Depth {
        Depth(d * Depth::DEPTH_INC)
    }
    pub fn val(self) -> i32 {
        self.0 / Depth::DEPTH_INC
    }
    pub fn org(self) -> i32 {
        self.0
    }

}

#[derive(Debug, Clone)]
pub struct PV(pub Vec<Move>);

impl PV {
    pub fn new() -> PV {
        PV(Vec::new())
    }
    pub fn add(&mut self, m : Move, pv : &PV) {
        self.0.clear();
        self.0.push(m);
        self.0.extend(&pv.0);
    }
}

impl fmt::Display for PV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s : String = "".to_string();
        for m in &self.0 {
            s = s + &m.to_string() + "\n";
        }
        write!(f,"{}",s)
    }  
}

impl Thread {
    pub fn new(pos : Position) -> Thread {
        let th = Thread {
            pos : pos,
            best_move : MoveSc::MOVE_NONE,
            nodes : 0,
        };
        th
    }

}

pub fn think(pos : &Position) {
    let mut ml = MoveList::new();
    ml.gen_legal(pos);
    let mut new_pv : PV  = PV::new();
    search_root(&mut ml, pos,  Depth::new(7), Score::SCORE_MIN, Score::SCORE_MAX, &mut new_pv);
    let mut th_lock = G_THREAD.lock().unwrap();
    th_lock.best_move.sc = Score::SCORE_MIN;
    for index in 0..ml.pos {
        let mc = &mut ml.mv[index];
        if mc.sc > th_lock.best_move.sc {
            th_lock.best_move = mc.clone();
        }
    }
}

pub fn search_root(ml : &mut MoveList, pos : &Position, depth : Depth, alpha : Score , beta : Score, pv : &mut PV ) {
    let mut best_sc = Score::SCORE_MIN;
    let mut alpha = alpha;
    let mut new_pv : PV  = PV::new();
    for index in 0..ml.pos {
        let mc = &mut ml.mv[index];
        let new_pos = pos.do_move(mc.mv);
        let new_depth = depth - Depth(1);
        let sc = -search(&new_pos, -beta, -alpha, new_depth, 1, &mut new_pv);
        println!("root : {} {}",sc.0,mc.mv);
        mc.sc = sc;
        if sc > best_sc {
            best_sc = sc;
            pv.add(mc.mv,&mut new_pv);
            println!("PV");
            println!("{}",pv);
            //to front
            if sc > alpha {
                alpha = sc;
            }
        }
    }
}

pub fn search(pos : &Position, alpha : Score, beta : Score, depth : Depth, ply : i32, mut pv : &mut PV) -> Score {

    if is_win(pos) {
        //println!("win");
        //println!("{}",pos);
        return Score::SCORE_MAX;
    }
    if depth < Depth::DEPTH_ZERO {
        return quies_search(pos, alpha, beta, ply+1, &mut pv);
        //return Score::SCORE_NONE;
    }

    if ply > Depth::MAX_PLY {
        return Score::SCORE_NONE;
    }

    let mut th_lock = G_THREAD.lock().unwrap();
    th_lock.nodes += 1;
    drop(th_lock);

    assert!(pos.is_ok());
    assert!(alpha < beta);
    assert!(ply >= 0 && ply <= Depth::MAX_PLY);

    let mut alpha = alpha;
    let mut best_sc = Score::SCORE_MIN;
    let mut ml = MoveList::new();
    let mut new_pv = PV::new();
    ml.gen_all(pos);
    for i in 0..ml.pos {
        let mc = ml.mv[i];
        let new_pos = pos.do_move(mc.mv);
        let new_depth = depth - Depth::DEPTH_ONE;
        let sc = -search(&new_pos, -beta, -alpha, new_depth, ply+1, &mut new_pv);
        //println!("{}", pos);
        //println!("{}", mc.mv);
        //println!("ply:{} score is {}",ply,sc.0);
        if sc > best_sc {
            best_sc = sc;
            pv.add(mc.mv,&mut new_pv);
            if sc >= beta {
                return best_sc;
            }
            if sc > alpha {
                alpha = sc;
            }
        }
    }
    best_sc
}

pub fn quies_search(pos : &Position, alpha : Score, beta : Score, ply : i32, pv : &mut PV) -> Score {

    if is_win(pos) {
        return Score::SCORE_MAX;
    }
    
    assert!(pos.is_ok());

    if ply > Depth::MAX_PLY {
        return Score::SCORE_NONE;
    }

    let mut th_lock = G_THREAD.lock().unwrap();
    th_lock.nodes += 1;
    drop(th_lock);

    let mut best_sc : Score = Score::SCORE_MIN;
    let mut ml = MoveList::new();

    if in_checked(pos) {
        ml.gen_legal(pos);
    } else {
        best_sc = eval(pos);
        // stand-pat
        if best_sc >= beta {
            return best_sc;
        }
        ml.gen_cap(pos);
    }
    let mut alpha = alpha;
    let mut new_pv = PV::new();

    for i in 0..ml.pos {
        let mc = ml.mv[i];
        let new_pos = pos.do_move(mc.mv);
        let sc = -quies_search(&new_pos, -beta, -alpha, ply+1, &mut new_pv);
        if sc > best_sc {
            best_sc = sc;
            pv.add(mc.mv,&mut new_pv);
            if sc >= beta {
                return best_sc;
            }
            if sc > alpha {
                alpha = sc;
            }
        }
    }
    best_sc
}
