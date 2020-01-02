use std::fmt;
use crate::common::*;
use crate::position::*;
use crate::attack::*;

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
    pub pos : usize,
}
impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let mut s : String = "".to_string();
       for _i in 0..self.pos {
           s = s + &_i.to_string() + " : " +  &self.mv[_i].mv.to_string() + " : " + &self.mv[_i].sc.0.to_string() + "\n";
       }
       write!(f,"{}",s)
    }  
}

impl MoveList {
    //TODO skip init mv 
    pub fn new() -> MoveList {
        MoveList {
            mv : [ MoveSc::MOVE_NONE; MoveList::MAX_LEGAL_MOVE],
            pos : 0,
        }
    }
    pub fn add(&mut self, mv : MoveSc) {
        self.mv[self.pos] = mv;
        self.pos += 1;
    }
    const MAX_LEGAL_MOVE : usize = 400;
    pub fn gen_all(&mut self, pos : &Position) {
        self.add_all(pos);
    }
    pub fn gen_cap(&mut self, pos : &Position) {
        self.add_cap(pos);
    }
    pub fn gen_legal(&mut self, pos : &Position) {
        let tmp_pos = pos;
        let mut tmp_ml = MoveList::new();
        tmp_ml.add_all(&pos);
        for i in 0..tmp_ml.pos {
            let ms = tmp_ml.mv[i as usize];
            let tmp_pos2 = tmp_pos.do_move(ms.mv);
            if is_win(&tmp_pos2) { continue; }
            self.add(ms);
        }
    }
    fn add_all(&mut self, pos : &Position) {
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
                            let mv = make_noprom_move(*sq, to, $p, cap.to_piece());
                            self.add(MoveSc::mv(mv));                        
                        }
                    };
                }
                
                match p {
                    Piece::HIYOKO => {
                        let to = if me == Color::BLACK { *sq + Square::INC_N } else { *sq + Square::INC_S };
                        let cap = pos.square(to);
                        if (cap == PieceColor::EMPTY) || (cap.is_us(opp)) {
                            let prom = if me == Color::BLACK { to <= Square::C1 } else { to >= Square::A4 };
                            let mv = if prom { make_prom_move(*sq, to, cap.to_piece()) } 
                                     else    { make_noprom_move(*sq, to, Piece::HIYOKO, cap.to_piece()) };
                            self.add(MoveSc::mv(mv));                        
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
                            self.add(MoveSc::mv(mv));                            
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
                            let mv = make_noprom_move(*sq, to, $p, cap.to_piece());
                            self.add(MoveSc::mv(mv));                        
                        }
                    };
                }
                
                match p {
                    Piece::HIYOKO => {
                        let to = if me == Color::BLACK { *sq + Square::INC_N } else { *sq + Square::INC_S };
                        let cap = pos.square(to);
                        if cap.is_us(opp) {
                            let prom = if me == Color::BLACK { to <= Square::C1 } else { to >= Square::A4 };
                            let mv = if prom { make_prom_move(*sq, to, cap.to_piece()) } 
                                     else    { make_noprom_move(*sq, to, Piece::HIYOKO, cap.to_piece()) };
                            self.add(MoveSc::mv(mv));                        
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
    {
        let pos = Position::init_sfen(START_SFEN);
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 4);
    }
    {
        let pos = Position::init_sfen("krz/1h1/1H1/ZRK w - 1");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 4);
    }
    {
        let pos = Position::init_sfen("1r1/3/3/1R1 b hkzHKZ");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 35);
    }
    {
        let pos = Position::init_sfen("1r1/3/3/1R1 w hkzHKZ");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 35);
    }
    {
        let pos = Position::init_sfen("r2/3/1N1/R2 b");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 8);
    }
    {
        let pos = Position::init_sfen("r2/3/1H1/R2 b");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 3);
    }
    {
        let pos = Position::init_sfen("r2/3/1h1/R2 w");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 4);
    }
    {
        let pos = Position::init_sfen("r2/1n1/3/R2 w");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 8);
    }
    {
        let pos = Position::init_sfen("RKZ/HH1/3/2r b");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 0);
    }
    {
        let pos = Position::init_sfen("R2/3/hh1/rkz w");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 0);
    }
    {
        let pos = Position::init_sfen("kHz/1r1/3/ZRK b h");
        println!("{}",pos);
        let mut ml = MoveList::new();
        ml.gen_all(&pos);
        println!("{}",ml);
        assert_eq!(ml.pos, 5);
    }
}