use std::fmt;
use crate::common::*;
use crate::position::*;
use crate::attack::*;
use crate::eval::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MoveSc {
    pub mv : Move,
    pub sc : Score,
}
impl MoveSc {
    pub fn new(mv : Move, sc : Score) -> MoveSc {
        MoveSc{ mv :mv, sc : sc }
    }
    pub fn mv(mv : Move) -> MoveSc {
        MoveSc { mv:mv, sc : Score::SCORE_NONE }
    }
    pub const MOVE_NONE : MoveSc = MoveSc {
        mv : Move::MOVE_NONE,
        sc : Score::SCORE_NONE
    };
}
pub struct MoveList {
    pub mv: [MoveSc; MoveList::MAX_LEGAL_MOVE],
    pub size : usize,
}
impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s : String = "".to_string();
        for i in 0..self.size {
            s = s + &i.to_string() + " : " +  &self.mv[i].mv.to_string() + " : " + &self.mv[i].sc.0.to_string() + "\n";
        }
        write!(f,"{}",s)
    }  
}

impl MoveList {
    //TODO skip init mv 
    pub fn new() -> MoveList {
        MoveList {
            mv : [ MoveSc::MOVE_NONE; MoveList::MAX_LEGAL_MOVE],
            size : 0,
        }
    }
    pub fn begin(&self) -> &[MoveSc] {
        &self.mv[0..self.size]
    }
    pub fn begin_mut(&mut self) -> &mut [MoveSc] {
        &mut self.mv[0..self.size]
    }
    pub fn clear(&mut self) {
        self.size = 0;
    }
    pub fn add(&mut self, mv : MoveSc) {
        self.mv[self.size] = mv;
        self.size += 1;
    }
    pub fn insersion_sort(&mut self) {
        let size = self.size;
        if size <= 1 { return; }
        self.mv[size].sc = Score::SCORE_MIN;
        for i in (0..size-1).rev() {
            let tmp = self.mv[i];
            let mut j = i+1;
            while tmp.sc < self.mv[j].sc {
                self.mv[j-1] = self.mv[j];
                j += 1;
            }
            self.mv[j-1] = tmp;
        }
    }
    pub fn note_move_score(&mut self, pos : &Position) {
        for mc in self.begin_mut() {
            if mc.mv.cap() != Piece::EMPTY || mc.mv.piece() == Piece::RAION {
                let sc = see(mc.mv,Score::SCORE_MIN, Score::SCORE_MAX, &pos);
                if sc < Score::SCORE_NONE {
                    mc.sc = Score::SCORE_MIN;
                }
            } else {

            }
        }
    }
    const MAX_LEGAL_MOVE : usize = 400;
    pub fn gen_all(&mut self, pos : &Position) {
        self.add_all(pos);
    }
    pub fn gen_cap(&mut self, pos : &Position) {
        self.add_cap(pos);
    }
    pub fn gen_legal(&mut self, pos : &Position) {
        debug_assert!(pos.is_ok());
        let tmp_pos = pos;
        let mut tmp_ml = MoveList::new();
        tmp_ml.add_all(&pos);
        for i in 0..tmp_ml.size {
            let ms = tmp_ml.mv[i as usize];
            let tmp_pos2 = tmp_pos.do_move(ms.mv);
            if is_win(&tmp_pos2) { continue; }
            self.add(ms);
        }
    }
    fn add_all(&mut self, pos : &Position) {
        debug_assert!(pos.is_ok());
        let me = pos.turn();
        let opp = Color::flip(me);
        for sq in Square::SQ_INDEX.iter() {
            let pc = pos.square(*sq);
            if pc.is_us(me) {
                let p = pc.to_piece();

                macro_rules! add_noprom_move {
                    ($inc:expr, $p:expr) => {
                        let to = *sq + $inc;
                        let cap = pos.square(to);
                        if (cap == PieceColor::EMPTY) || (cap.is_us(opp)) {
                            let cap_p = cap.to_piece();
                            let mv = make_noprom_move(*sq, to, $p, cap_p);
                            let sc = if cap != PieceColor::EMPTY { PIECE_SCORE[cap_p.0 as usize] - PIECE_SCORE[$p.0 as usize] } else { Score::SCORE_NONE };
                            self.add(MoveSc::new(mv,sc));                        
                        }
                    };
                }
                
                match p {
                    Piece::HIYOKO => {
                        let to = if me == Color::BLACK { *sq + Square::INC_N } else { *sq + Square::INC_S };
                        let cap = pos.square(to);
                        if (cap == PieceColor::EMPTY) || (cap.is_us(opp)) {
                            let cap_p = cap.to_piece();
                            let prom = if me == Color::BLACK { to <= Square::C1 } else { to >= Square::A4 };
                            let mv = if prom { make_prom_move(*sq, to, cap.to_piece()) } 
                                     else    { make_noprom_move(*sq, to, Piece::HIYOKO, cap.to_piece()) };
                            let mut sc = Score::SCORE_NONE;
                            if cap != PieceColor::EMPTY {
                                sc = PIECE_SCORE[cap_p.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize];
                            }
                            if prom {
                                sc += PIECE_SCORE[Piece::NIWATORI.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize];
                            }
                            self.add(MoveSc::new(mv,sc));                        
                        }
                    },
                    Piece::KIRIN => {
                        add_noprom_move!(Square::INC_N, Piece::KIRIN);
                        add_noprom_move!(Square::INC_S, Piece::KIRIN);
                        add_noprom_move!(Square::INC_W, Piece::KIRIN);
                        add_noprom_move!(Square::INC_E, Piece::KIRIN);
                    },
                    Piece::ZOU => {
                        add_noprom_move!(Square::INC_NW, Piece::ZOU);
                        add_noprom_move!(Square::INC_NE, Piece::ZOU);
                        add_noprom_move!(Square::INC_SW, Piece::ZOU);
                        add_noprom_move!(Square::INC_SE, Piece::ZOU);
                    },
                    Piece::RAION => {
                        add_noprom_move!(Square::INC_N,  Piece::RAION);
                        add_noprom_move!(Square::INC_S,  Piece::RAION);
                        add_noprom_move!(Square::INC_W,  Piece::RAION);
                        add_noprom_move!(Square::INC_E,  Piece::RAION);
                        add_noprom_move!(Square::INC_NW, Piece::RAION);
                        add_noprom_move!(Square::INC_NE, Piece::RAION);
                        add_noprom_move!(Square::INC_SW, Piece::RAION);
                        add_noprom_move!(Square::INC_SE, Piece::RAION);
                    },
                    Piece::NIWATORI => {
                        
                        if me == Color::BLACK {
                            add_noprom_move!(Square::INC_NE, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_N,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_NW, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_E,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_S,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_W,  Piece::NIWATORI);
                        } else {
                            add_noprom_move!(Square::INC_SW, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_S,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_SE, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_E,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_N,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_W,  Piece::NIWATORI);
                        }
                    }
                    _ => {  unreachable!() }
                }    
            } else if pc == PieceColor::EMPTY {
                macro_rules! add_drop_move {
                    ($p:expr) => {
                        if pos.has(me,$p) {
                            let to = *sq;
                            let mv = make_drop_move(to, $p);
                            self.add(MoveSc::new(mv,Score::QUIET_MOVE_SCORE));                            
                        }
                    };
                }
                add_drop_move!(Piece::KIRIN);
                add_drop_move!(Piece::ZOU);
                add_drop_move!(Piece::HIYOKO);
            }
        }
    }
    fn add_cap(&mut self, pos : &Position) {
        debug_assert!(pos.is_ok());
        let me = pos.turn();
        let opp = Color::flip(me);
        for sq in Square::SQ_INDEX.iter() {
            let pc = pos.square(*sq);
            if pc.is_us(me) {
                let p = pc.to_piece();

                macro_rules! add_noprom_move {
                    ($inc:expr, $p:expr) => {
                        let to = *sq + $inc;
                        let cap = pos.square(to);
                        if cap.is_us(opp) {
                            let cap_p = cap.to_piece();
                            let mv = make_noprom_move(*sq, to, $p, cap_p);
                            let sc = PIECE_SCORE[cap_p.0 as usize] - PIECE_SCORE[$p.0 as usize];
                            self.add(MoveSc::new(mv,sc));                        
                        }
                    };
                }
                
                match p {
                    Piece::HIYOKO => {

                        let to = if me == Color::BLACK { *sq + Square::INC_N } else { *sq + Square::INC_S };
                        let cap = pos.square(to);
                        if cap.is_us(opp) {
                            let cap_p = cap.to_piece();
                            let prom = if me == Color::BLACK { to <= Square::C1 } else { to >= Square::A4 };
                            let mv = if prom { make_prom_move(*sq, to, cap.to_piece()) } 
                                     else    { make_noprom_move(*sq, to, Piece::HIYOKO, cap.to_piece()) };
                            let mut sc = PIECE_SCORE[cap_p.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize];
                            if prom {
                                sc += PIECE_SCORE[Piece::NIWATORI.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize];
                            }
                            self.add(MoveSc::new(mv,sc));                        
                        }
                    },
                    Piece::KIRIN => {
                        add_noprom_move!(Square::INC_N, Piece::KIRIN);
                        add_noprom_move!(Square::INC_S, Piece::KIRIN);
                        add_noprom_move!(Square::INC_W, Piece::KIRIN);
                        add_noprom_move!(Square::INC_E, Piece::KIRIN);
                    },
                    Piece::ZOU => {
                        add_noprom_move!(Square::INC_NW, Piece::ZOU);
                        add_noprom_move!(Square::INC_NE, Piece::ZOU);
                        add_noprom_move!(Square::INC_SW, Piece::ZOU);
                        add_noprom_move!(Square::INC_SE, Piece::ZOU);
                    },
                    Piece::RAION => {
                        add_noprom_move!(Square::INC_N,  Piece::RAION);
                        add_noprom_move!(Square::INC_S,  Piece::RAION);
                        add_noprom_move!(Square::INC_W,  Piece::RAION);
                        add_noprom_move!(Square::INC_E,  Piece::RAION);
                        add_noprom_move!(Square::INC_NW, Piece::RAION);
                        add_noprom_move!(Square::INC_NE, Piece::RAION);
                        add_noprom_move!(Square::INC_SW, Piece::RAION);
                        add_noprom_move!(Square::INC_SE, Piece::RAION);
                    },
                    Piece::NIWATORI => {
                        
                        if me == Color::BLACK {
                            add_noprom_move!(Square::INC_NE, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_N,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_NW, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_E,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_S,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_W,  Piece::NIWATORI);
                        } else {
                            add_noprom_move!(Square::INC_SW, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_S,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_SE, Piece::NIWATORI);
                            add_noprom_move!(Square::INC_E,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_N,  Piece::NIWATORI);
                            add_noprom_move!(Square::INC_W,  Piece::NIWATORI);
                        }
                    }
                    _ => {  unreachable!() }
                }    
            }
        }
    }
}
#[test]
fn test_gen() {
    macro_rules! test_gen_all {
        ($sfen:expr, $size:expr) => {
            {
                let pos = Position::init_sfen($sfen);
                println!("{}",pos);
                let mut ml = MoveList::new();
                ml.gen_all(&pos);
                println!("{}",ml);
                assert_eq!(ml.size, $size);
            }
        };
    }
    test_gen_all!(START_SFEN,4);
    test_gen_all!("krz/1h1/1H1/ZRK w - 1",4);
    test_gen_all!("1r1/3/3/1R1 b hkzHKZ",35);
    test_gen_all!("1r1/3/3/1R1 w hkzHKZ",35);
    test_gen_all!("r2/3/1N1/R2 b",8);
    test_gen_all!("r2/3/1H1/R2 b",3);
    test_gen_all!("r2/3/1h1/R2 w",4);
    test_gen_all!("r2/1n1/3/R2 w",8);
    test_gen_all!("RKZ/HH1/3/2r b",0);
    test_gen_all!("R2/3/hh1/rkz w",0);
    test_gen_all!("kHz/1r1/3/ZRK b h",5);
    {
        let pos = Position::init_sfen(START_SFEN);
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        for mc in ml.begin().iter() {
            println!("{}",mc.mv);
        }
        ml.clear();
    }
    {
        let pos = Position::init_sfen("1r1/3/zhk/KRZ b H");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        ml.insersion_sort();
        for mc in ml.begin().iter() {
            println!("{} {}",mc.mv, mc.sc.0);
        }
        //assert!(false);
    }
}
