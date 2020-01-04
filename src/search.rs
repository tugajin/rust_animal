use std::fmt;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::common::*;
use crate::position::*;
use crate::gen::*;
use crate::attack::*;
use crate::eval::*;
use crate::game::*;

const MAX_SP : usize = 2048;

#[derive(Debug, Copy, Clone)]
struct Stack {
    pub key : Key,
    pub hand_b : Key,
}

pub struct Thread {
    pub pos : Position,
    pub best_move : MoveSc,
    pub nodes : u64,
    root_turn : Color,
    pub stack_sp : usize,
    stack : Vec<Stack>,
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
        let mut th = Thread {
            pos : pos,
            best_move : MoveSc::MOVE_NONE,
            nodes : 0,
            stack : Vec::new(),
            root_turn : Color::BLACK,
            stack_sp : 0,
        };
        let stc = Stack {key : Key(0), hand_b : Key(0)};
        th.stack = vec![stc; MAX_SP];
        th.root_turn = th.pos.turn();
        th
    }

    fn check_rep(&self, ply :usize) -> bool {
        let mut index : i32 = (ply as i32) - 4 ;
        let mut count = 0;
        while index >= 0 && count < 2 {
            if self.pos.key() == self.stack[index as usize].key 
            && self.pos.hand_b() == self.stack[index as usize].hand_b {
                //println!("same {} {}!",ply,index);
                return true;
            }
            index -= 2;
            count += 1;
        }
        false
    }

    pub fn think(&mut self) {
        let mut ml = MoveList::new();
        ml.gen_legal(&self.pos);

        //let mut rng = rand::thread_rng();
        //ml.mv.shuffle(&mut rng);

        let mut new_pv : PV  = PV::new();
        self.search_root(&mut ml,  Depth::new(4), Score::EVAL_MIN, Score::EVAL_MAX, &mut new_pv);
        self.best_move.sc = Score::EVAL_MIN;
        for index in 0..ml.size {
            let mc = &mut ml.mv[index];
            if mc.sc > self.best_move.sc {
                self.best_move = mc.clone();
            }
        }
    }
    
    pub fn search_root(&mut self, ml : &mut MoveList, depth : Depth, alpha : Score , beta : Score, pv : &mut PV ) {
        let mut best_sc = Score::EVAL_MIN;
        let mut alpha = alpha;
        let mut new_pv : PV  = PV::new();
        let org_pos = self.pos.clone();
        
        self.stack.insert(self.stack_sp,Stack{ key:self.pos.key(), hand_b:self.pos.hand_b() });

        for index in 0..ml.size {
            let mc = &mut ml.mv[index];
            self.pos = self.pos.do_move(mc.mv);
            let new_depth = depth - Depth(1);
            let sc = -self.search(-beta, -alpha, new_depth, 1, &mut new_pv);
            self.pos = org_pos.clone();
            //println!("root : {} {}",sc.0,mc.mv);
            mc.sc = sc;
            if sc > best_sc {
                best_sc = sc;
                pv.add(mc.mv,&mut new_pv);
                //println!("PV");
                //println!("{}",pv);
                //to front
                if sc > alpha {
                    alpha = sc;
                }
            }
        }
    }
    
    pub fn search(&mut self, alpha : Score, beta : Score, depth : Depth, ply : i32, mut pv : &mut PV) -> Score {
    
        if is_win(&self.pos) {
            //println!("win");
            //println!("{}",pos);
            return Score::in_mate(ply);
        }
        if depth < Depth::DEPTH_ZERO {
            return self.quies_search(alpha, beta, ply+1, &mut pv);
            //return Score::SCORE_NONE;
        }
    
        if ply > Depth::MAX_PLY {
            return Score::SCORE_NONE;
        }

        assert!(&self.pos.is_ok());
        debug_assert!(alpha < beta,"alpha {} : beta {}",alpha.0,beta.0);
        assert!(ply >= 0 && ply <= Depth::MAX_PLY);

        self.nodes += 1;

        self.stack.insert(self.stack_sp + (ply as usize), Stack{ key:self.pos.key(), hand_b:self.pos.hand_b() });

        if self.check_rep(self.stack_sp + (ply as usize)) {
            /*if self.root_turn != self.pos.turn() {
                return Score::SCORE_REP;
            } else {
                return Score::SCORE_REP;
            }*/
            return Score::SCORE_NONE;
        }
        
        let mut alpha = alpha;
        let mut best_sc = Score::EVAL_MIN;
        let mut ml = MoveList::new();
        let mut new_pv = PV::new();
        let org_pos = self.pos.clone();
        ml.gen_all(&self.pos);
        ml.insersion_sort();
        //let mut rng = rand::thread_rng();
        //ml.mv.shuffle(&mut rng);


        for i in 0..ml.size {
            let mc = ml.mv[i];
            self.pos = self.pos.do_move(mc.mv);

            let new_depth = depth - Depth::DEPTH_ONE;
            let sc = -self.search(-beta, -alpha, new_depth, ply+1, &mut new_pv);
            self.pos = org_pos.clone();
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
    
    pub fn quies_search(&mut self, alpha : Score, beta : Score, ply : i32, pv : &mut PV) -> Score {
    
        if is_win(&self.pos) {
            return Score::SCORE_MAX;
        }
        
        assert!(&self.pos.is_ok());
    
        if ply > Depth::MAX_PLY {
            return Score::SCORE_NONE;
        }
        self.nodes += 1;
    
        let mut best_sc : Score = Score::EVAL_MIN;
        let mut ml = MoveList::new();
    
        if in_checked(&self.pos) {
            ml.gen_legal(&self.pos);
        } else {
            best_sc = eval(&self.pos);
            // stand-pat
            if best_sc >= beta {
                return best_sc;
            }
            ml.gen_cap(&self.pos);
        }
        ml.insersion_sort();
        let mut alpha = alpha;
        let mut new_pv = PV::new();
        let org_pos = self.pos.clone();

        //let mut rng = rand::thread_rng();
        //ml.mv.shuffle(&mut rng);
    
        for i in 0..ml.size {
            let mc = ml.mv[i];
            
            self.pos = self.pos.do_move(mc.mv);
            let sc = -self.quies_search(-beta, -alpha, ply+1, &mut new_pv);
            self.pos = org_pos.clone();
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
}


