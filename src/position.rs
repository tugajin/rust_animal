use crate::common::*;
use crate::eval::*;
use std::fmt;
use std::collections::HashSet;
use rand::Rng;

#[derive(Clone)]
pub struct Position {
    key : Key,
    hand_b: Key,
    square : [PieceColor; Square::SQ_ALL_SIZE],
    turn : Color,
    hand : [Hand; Color::COLOR_SIZE],
    raion_sq : [Square; Color::COLOR_SIZE],
    material : Score,
}

pub const START_SFEN: &str = "krz/1h1/1H1/ZRK b - 1";

pub const ZOBRIST_TURN : Key = Key(1234567890);

lazy_static! {
    pub static ref ZOBRIST : [[[Key; Square::SQ_ALL_SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE] = {
        let mut hs : HashSet<u64> = HashSet::new();
        let mut z  = [[[Key(0); Square::SQ_ALL_SIZE]; Piece::PIECE_SIZE]; Color::COLOR_SIZE ];
        //seedを固定
        //https://scrapbox.io/nwtgck/Rust%E3%81%A7%E5%86%8D%E7%8F%BE%E7%9A%84%E3%81%AA%E4%B9%B1%E6%95%B0%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B
        let seed: [u8; 32] = [13; 32];
        let mut rng : rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
        for col in 0..Color::COLOR_SIZE {
            for p in 0..Piece::PIECE_SIZE {
                for sq in 0..Square::SQ_ALL_SIZE {
                    loop {
                        let rnd : u64 = rng.gen();
                        if !hs.contains(&rnd) && ZOBRIST_TURN.0 != rnd && rnd != 0 {
                            z[col][p][sq] = Key(rnd);
                            hs.insert(rnd);
                            break;                       
                        } else {
                            println!("same hash!");
                        }
                    }
                }
            }
        }
        z
    };
}

impl Position {

    pub fn new() -> Position {
        Position {
            key : Key(0),
            hand_b : Key(0),
            square: [PieceColor::WALL; Square::SQ_ALL_SIZE ],
            turn: Color::BLACK,
            hand: [Hand::HAND_NONE; Color::COLOR_SIZE],
            raion_sq: [Square::WALL; Color::COLOR_SIZE],
            material : Score::SCORE_NONE,
        }
    }

    pub fn square(&self, sq : Square) -> PieceColor {
        self.square[sq.0 as usize]
    }
    pub fn turn(&self) -> Color {
        self.turn
    }
    pub fn raion_sq(&self, c : Color) -> Square {
        self.raion_sq[c.0 as usize]
    }
    pub fn hand_num(&self, c : Color, p : Piece) -> u32 {
        self.hand[c.0 as usize].num(p)
    }
    pub fn has(&self, c : Color, p : Piece) -> bool {
        self.hand[c.0 as usize].has(p)
    }
    pub fn material(&self) -> Score {
        self.material
    }
    pub fn key(&self) -> Key {
        self.key
    }
    pub fn hand_b(&self) -> Key {
        self.hand_b
    }
    pub fn do_move(&self, mv : Move) -> Position {
        
        debug_assert!(self.is_ok(), "{}", self);
        debug_assert!(Move::is_ok(mv), "{}",mv);

        let mut pos : Position = self.clone();
        let me = pos.turn();
        let opp = Color::flip(me);
        if mv.is_drop() {
            let p = mv.piece();
            let to = mv.to();
            pos.square[to.0 as usize] = p.to_piece_color(pos.turn());
            pos.hand[pos.turn().0 as usize].dec(p);
            pos.key ^= ZOBRIST[me.0 as usize][p.0 as usize][to.0 as usize];
        } else {
            let from = mv.from();
            let to = mv.to();
            let prom = mv.prom();
            let p = mv.piece();
            let cap = mv.cap();
            pos.square[from.0 as usize] = PieceColor::EMPTY;
            pos.key ^= ZOBRIST[me.0 as usize][p.0 as usize][from.0 as usize];
            if !prom {
                pos.square[to.0 as usize] = p.to_piece_color(pos.turn());
                pos.key ^= ZOBRIST[me.0 as usize][p.0 as usize][to.0 as usize];
                if p == Piece::RAION {
                    pos.raion_sq[pos.turn.0 as usize] = to;
                }
            } else {
                pos.square[to.0 as usize] = Piece::NIWATORI.to_piece_color(pos.turn());
                pos.key ^= ZOBRIST[me.0 as usize][Piece::NIWATORI.0 as usize][to.0 as usize];
                pos.material += if me == Color::BLACK {  PIECE_SCORE[Piece::NIWATORI.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize]  } 
                                                 else {-(PIECE_SCORE[Piece::NIWATORI.0 as usize] - PIECE_SCORE[Piece::HIYOKO.0 as usize]) };
            }
            if cap != Piece::EMPTY {
                pos.hand[pos.turn().0 as usize].inc(cap);
                pos.key ^= ZOBRIST[opp.0 as usize][cap.0 as usize][to.0 as usize];
                pos.material += if me == Color::BLACK { PIECE_EX_SCORE[cap.0 as usize] } else { -PIECE_EX_SCORE[cap.0 as usize] };
            }
        }
        pos.turn = Color::flip(pos.turn());
        pos.key ^= ZOBRIST_TURN;
        pos.hand_b = Key(pos.hand[Color::BLACK.0 as usize].val() as u64);
        debug_assert!(self.is_ok(), "{}", self);
        pos
    }

    pub fn init_sfen(sfen : &str) -> Position {
        let mut pos = Position::new();
        let mut sq_index = 0;
        // pos loop
        let sfen_vec = sfen.chars().collect::<Vec<char>>();
        let mut sfen_vec_index : usize = 0;
        loop { 
            //念の為
            if sfen_vec_index >= sfen_vec.len() {
                break;
            } else if sfen_vec[sfen_vec_index].is_whitespace() {
                break;
            } else if sfen_vec[sfen_vec_index].is_alphabetic() {
                let pc_c = sfen_vec[sfen_vec_index];
                let pc = PieceColor::from_str(pc_c);

                pos.square[Square::SQ_INDEX[sq_index].0 as usize] = pc;

                if pc == PieceColor::RAION_B {
                    pos.raion_sq[Color::BLACK.0 as usize] = Square::SQ_INDEX[sq_index];
                } else if pc == PieceColor::RAION_W {
                    pos.raion_sq[Color::WHITE.0 as usize] = Square::SQ_INDEX[sq_index];
                }
                sq_index += 1;
            } else if sfen_vec[sfen_vec_index].is_digit(10) {
                let num : u32 = sfen_vec[sfen_vec_index].to_digit(10).unwrap();
                for _i in 0..num {
                    pos.square[Square::SQ_INDEX[sq_index].0 as usize] = PieceColor::EMPTY;
                    sq_index += 1;
                }
            } 
            sfen_vec_index += 1;
        }
        sfen_vec_index += 1;
        //turn
        if sfen_vec[sfen_vec_index] == 'b' {
            pos.turn = Color::BLACK;
        } else if sfen_vec[sfen_vec_index] == 'w' {
            pos.turn = Color::WHITE;
        }
        sfen_vec_index += 2;
        let mut hand_num = 1;
        //hand loop
        loop {
            //念の為
            if sfen_vec_index >= sfen_vec.len() {
                break;
            } else if sfen_vec[sfen_vec_index].is_whitespace() {
                break;
            } else if sfen_vec[sfen_vec_index] == '-'  {
                break;
            } else if sfen_vec[sfen_vec_index].is_digit(10)  {
                hand_num = sfen_vec[sfen_vec_index].to_digit(10).unwrap();
            } else if sfen_vec[sfen_vec_index].is_alphabetic() {
                let p = PieceColor::from_str(sfen_vec[sfen_vec_index]).to_piece();
                let col = if sfen_vec[sfen_vec_index].is_uppercase() { Color::BLACK } else { Color::WHITE };
                for _i in 0..hand_num {
                    pos.hand[col.0 as usize].inc(p);
                }
                
                hand_num = 1;
            }
            sfen_vec_index += 1;
        }
        pos.hand_b = Key(pos.hand[Color::BLACK.0 as usize].val() as u64);
        pos.key = pos.gen_key();
        pos.material = pos.calc_material();
        pos
    }
    fn gen_key(&self) -> Key {
        let mut key = Key(0);
        if self.turn == Color::WHITE {
            key ^= ZOBRIST_TURN;
        }
        for sq in Square::SQ_INDEX.iter() {
            let pc = self.square[sq.0 as usize];
            if PieceColor::is_piece(pc) {
                let p = pc.to_piece();
                let col = pc.color();
                key ^= ZOBRIST[col.0 as usize][p.0 as usize][sq.0 as usize];
            }
        }
        key
    }
    fn calc_material(&self) -> Score {
        let mut sc = Score::SCORE_NONE;
        for sq in Square::SQ_INDEX.iter() {
            let pc = self.square[sq.0 as usize];
            if PieceColor::is_piece(pc) {
                let p = pc.to_piece();
                let col = pc.color();
                sc += if col == Color::BLACK { PIECE_SCORE[p.0 as usize] } else { -PIECE_SCORE[p.0 as usize] };
            }
        }
        sc += PIECE_SCORE[Piece::HIYOKO.0 as usize] * self.hand_num(Color::BLACK, Piece::HIYOKO) as i32;
        sc += PIECE_SCORE[Piece::KIRIN.0 as usize] * self.hand_num(Color::BLACK, Piece::KIRIN) as i32;
        sc += PIECE_SCORE[Piece::ZOU.0 as usize] * self.hand_num(Color::BLACK, Piece::ZOU) as i32;

        sc -= PIECE_SCORE[Piece::HIYOKO.0 as usize] * self.hand_num(Color::WHITE, Piece::HIYOKO) as i32;
        sc -= PIECE_SCORE[Piece::KIRIN.0 as usize] * self.hand_num(Color::WHITE, Piece::KIRIN) as i32;
        sc -= PIECE_SCORE[Piece::ZOU.0 as usize] * self.hand_num(Color::WHITE, Piece::ZOU) as i32;
        sc
    }
    pub fn is_ok(&self) -> bool {
        let wall_sq = vec![0, 1, 2, 3, 4, 5, 9, 10, 14, 15, 19, 20, 24, 25, 26, 27, 28, 29];
        for sq in &wall_sq {
            if self.square[*sq as usize] != PieceColor::WALL {
                println!("broke wall. {}  {} line: {}", sq, self.square[*sq as usize], line!());
                return false;
            }
        }
        let mut piece_num = [0 ; Piece::PIECE_SIZE];

        for sq in Square::SQ_INDEX.iter() {
            if !PieceColor::is_ok(self.square[sq.0 as usize]) {
                println!("broke position. {}  {} line: {}", sq, self.square[sq.0 as usize], line!());
                return false;
            } else {
                let mut p = self.square[sq.0 as usize].to_piece();
                if p == Piece::NIWATORI { p = Piece::NIWATORI; }
                piece_num[p.0 as usize] += 1;
            }
        }
        piece_num[Piece::HIYOKO.0 as usize] += self.hand[Color::BLACK.0 as usize].num(Piece::HIYOKO);
        piece_num[Piece::KIRIN.0 as usize] += self.hand[Color::BLACK.0 as usize].num(Piece::KIRIN);
        piece_num[Piece::ZOU.0 as usize] += self.hand[Color::BLACK.0 as usize].num(Piece::ZOU);

        piece_num[Piece::HIYOKO.0 as usize] += self.hand[Color::WHITE.0 as usize].num(Piece::HIYOKO);
        piece_num[Piece::KIRIN.0 as usize] += self.hand[Color::WHITE.0 as usize].num(Piece::KIRIN);
        piece_num[Piece::ZOU.0 as usize] += self.hand[Color::WHITE.0 as usize].num(Piece::ZOU);

        if self.square[self.raion_sq(Color::BLACK).0 as usize] != PieceColor::RAION_B {
            println!("raion_sq error. {}  {} line: {}", self.raion_sq(Color::BLACK), self.square[self.raion_sq(Color::BLACK).0 as usize], line!());
            return false;
        }
        if self.square[self.raion_sq(Color::WHITE).0 as usize] != PieceColor::RAION_W {
            println!("raion_sq error. {}  {} line: {}", self.raion_sq(Color::WHITE), self.square[self.raion_sq(Color::WHITE).0 as usize], line!());
            return false;
        }
        if piece_num[Piece::EMPTY.0 as usize] > 10 {
            println!("too many empty. {} line: {}", piece_num[Piece::EMPTY.0 as usize] , line!());
            return false;
        }
        if piece_num[Piece::HIYOKO.0 as usize] > 2 {
            println!("too many HIYOKO. {} line: {}", piece_num[Piece::HIYOKO.0 as usize] , line!());
            return false;
        }
        if piece_num[Piece::KIRIN.0 as usize] > 2 {
            println!("too many KIRIN. {} line: {}", piece_num[Piece::KIRIN.0 as usize] , line!());
            return false;
        }
        if piece_num[Piece::ZOU.0 as usize] > 2 {
            println!("too many ZOU. {} line: {}", piece_num[Piece::ZOU.0 as usize] , line!());
            return false;
        }
        if piece_num[Piece::RAION.0 as usize] > 2 {
            println!("too many RAION. {} line: {}", piece_num[Piece::RAION.0 as usize] , line!());
            return false;
        }
        if self.hand[Color::BLACK.0 as usize].val() != (self.hand_b.0 as u32) {
            println!("hand_b error. {} {} line: {}", self.hand[Color::BLACK.0 as usize].val(), self.hand_b.0, line!());
            return false;
        }
        let debug_key = self.gen_key();
        if self.key != debug_key {
            println!("key error. {} {} line: {}", self.key.0, debug_key.0, line!());
            return false;
        }
        let debug_material = self.calc_material();
        if self.material != debug_material {
            println!("material error. {}\n {} {} line: {}",self ,self.material.0, debug_material.0, line!());
            return false;
        }
        true
    }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s : String = "".to_string();
        s = s + "Key : " + &self.key.0.to_string() + "\n";
        s = s + "Turn : " + &self.turn.to_string() + "\n";
        s = s +  &self.hand[Color::WHITE.0 as usize].to_string() + "\n";
        s += "   A  B  C\n";
        for (sq, index) in Square::SQ_INDEX.iter().enumerate() {
            if (sq) % 3 == 0 {
                s = s + &((sq / 3) + 1).to_string() + " ";
            }
            s = s + &self.square[index.0 as usize].to_string() + " ";
            if (sq + 1) % 3 == 0 {
                s = s + "\n";
            }
        }
        s = s +  &self.hand[Color::BLACK.0 as usize].to_string();
        write!(f, "{}", s)
    }  
}
#[test]
fn test_position() {
    {
        let pos = Position::init_sfen(START_SFEN);
        assert_eq!(pos.square(Square::A1),PieceColor::KIRIN_W);
        assert_eq!(pos.square(Square::B1),PieceColor::RAION_W);
        assert_eq!(pos.square(Square::C1),PieceColor::ZOU_W);
        assert_eq!(pos.square(Square::A2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B2),PieceColor::HIYOKO_W);
        assert_eq!(pos.square(Square::C2),PieceColor::EMPTY);
        
        assert_eq!(pos.square(Square::A3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B3),PieceColor::HIYOKO_B);
        assert_eq!(pos.square(Square::C3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A4),PieceColor::ZOU_B);
        assert_eq!(pos.square(Square::B4),PieceColor::RAION_B);
        assert_eq!(pos.square(Square::C4),PieceColor::KIRIN_B);
        
        assert_eq!(pos.raion_sq(Color::BLACK),Square::B4);
        assert_eq!(pos.raion_sq(Color::WHITE),Square::B1);

        assert_eq!(pos.turn(), Color::BLACK);
        
        assert_eq!(pos.hand_num(Color::BLACK, Piece::HIYOKO),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::ZOU),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::KIRIN),0);

        assert_eq!(pos.hand_num(Color::WHITE, Piece::HIYOKO),0);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::ZOU),0);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::KIRIN),0);
        
    }
    {
        let pos = Position::init_sfen("2r/3/3/R2 w 2K2Z2H");
        assert_eq!(pos.square(Square::A1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C1),PieceColor::RAION_W);
        assert_eq!(pos.square(Square::A2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C2),PieceColor::EMPTY);
        
        assert_eq!(pos.square(Square::A3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A4),PieceColor::RAION_B);
        assert_eq!(pos.square(Square::B4),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C4),PieceColor::EMPTY);
        
        assert_eq!(pos.raion_sq(Color::BLACK),Square::A4);
        assert_eq!(pos.raion_sq(Color::WHITE),Square::C1);

        assert_eq!(pos.turn(), Color::WHITE);

        assert_eq!(pos.hand_num(Color::BLACK, Piece::HIYOKO),2);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::ZOU),2);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::KIRIN),2);

        assert_eq!(pos.hand_num(Color::WHITE, Piece::HIYOKO),0);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::ZOU),0);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::KIRIN),0);
        
    }
    {
        let pos = Position::init_sfen("3/2r/R2/3 w 2k2z2h");
        assert_eq!(pos.square(Square::A1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C2),PieceColor::RAION_W);
        
        assert_eq!(pos.square(Square::A3),PieceColor::RAION_B);
        assert_eq!(pos.square(Square::B3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A4),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B4),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C4),PieceColor::EMPTY);
        
        assert_eq!(pos.raion_sq(Color::BLACK),Square::A3);
        assert_eq!(pos.raion_sq(Color::WHITE),Square::C2);

        assert_eq!(pos.turn(), Color::WHITE);

        assert_eq!(pos.hand_num(Color::BLACK, Piece::HIYOKO),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::ZOU),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::KIRIN),0);

        assert_eq!(pos.hand_num(Color::WHITE, Piece::HIYOKO),2);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::ZOU),2);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::KIRIN),2);

    }
    {
        let pos = Position::init_sfen("3/1r1/1R1/3 w 2k2z2h");
        assert_eq!(pos.square(Square::A1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C1),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A2),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B2),PieceColor::RAION_W);
        assert_eq!(pos.square(Square::C2),PieceColor::EMPTY);
        
        assert_eq!(pos.square(Square::A3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B3),PieceColor::RAION_B);
        assert_eq!(pos.square(Square::C3),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::A4),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::B4),PieceColor::EMPTY);
        assert_eq!(pos.square(Square::C4),PieceColor::EMPTY);
        
        assert_eq!(pos.raion_sq(Color::BLACK),Square::B3);
        assert_eq!(pos.raion_sq(Color::WHITE),Square::B2);

        assert_eq!(pos.turn(), Color::WHITE);

        assert_eq!(pos.hand_num(Color::BLACK, Piece::HIYOKO),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::ZOU),0);
        assert_eq!(pos.hand_num(Color::BLACK, Piece::KIRIN),0);

        assert_eq!(pos.hand_num(Color::WHITE, Piece::HIYOKO),2);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::ZOU),2);
        assert_eq!(pos.hand_num(Color::WHITE, Piece::KIRIN),2);

    } 
}