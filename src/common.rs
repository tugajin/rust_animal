use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color(pub i32);
    
impl Color {
    pub const BLACK : Color = Color(1);
    pub const WHITE : Color = Color(2);
    pub const COLOR_SIZE : usize = 3;
    pub fn flip(c : Color) -> Color {
        Color(3 - c.0) 
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::BLACK => write!(f, "BLACK"),
            Color::WHITE => write!(f, "WHITE"),
            _ => write!(f, "COLOR:ERROR:{}", self.0),
        }
    }  
}

#[test]
fn test_color() {
    let b = Color::BLACK;
    let w = Color::WHITE;
    assert_eq!(Color::WHITE , Color::flip(b));
    assert_eq!(Color::BLACK , Color::flip(w));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece(pub i32);
 #[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PieceColor(pub i32); 

impl Piece {
    pub const EMPTY : Piece = Piece(0);
    pub const HIYOKO: Piece = Piece(1);
    pub const KIRIN: Piece = Piece(2);
    pub const ZOU: Piece = Piece(3);
    pub const NIWATORI: Piece = Piece(4);
    pub const RAION: Piece = Piece(5);
    pub const PIECE_SIZE : usize = 6;

    pub fn to_piece_color(self, c : Color) -> PieceColor {
        PieceColor(self.0 | (c.0 << 3))
    }

    pub fn is_ok(p : Piece) -> bool {
        Piece::is_piece(p) || p == Piece::EMPTY
    }
    pub fn is_piece(p : Piece) -> bool {
        match p {
            Piece::HIYOKO | Piece::KIRIN  |
            Piece::ZOU | Piece::NIWATORI | Piece::RAION => { true },
            _ => { false }
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::EMPTY => write!(f, "*"),
            Piece::HIYOKO => write!(f, "H"),
            Piece::KIRIN =>  write!(f, "K"),
            Piece::ZOU =>  write!(f, "Z"),
            Piece::RAION =>  write!(f, "R"),
            Piece::NIWATORI =>  write!(f, "N"),
            _ => write!(f, "PIECE:ERROR:{}", self.0),
        }
    }  
}

impl PieceColor {

    pub const BLACK_FLAG: PieceColor = PieceColor(1 << 3);
    pub const WHITE_FLAG: PieceColor = PieceColor(1 << 4);

    pub const EMPTY : PieceColor = PieceColor(0);
    pub const WALL : PieceColor = PieceColor(1 << 5);

    pub const HIYOKO_B: PieceColor = PieceColor(Piece::HIYOKO.0 | PieceColor::BLACK_FLAG.0);
    pub const KIRIN_B: PieceColor = PieceColor(Piece::KIRIN.0 | PieceColor::BLACK_FLAG.0);
    pub const ZOU_B: PieceColor = PieceColor(Piece::ZOU.0 | PieceColor::BLACK_FLAG.0);
    pub const RAION_B: PieceColor = PieceColor(Piece::RAION.0 | PieceColor::BLACK_FLAG.0);
    pub const NIWATORI_B: PieceColor = PieceColor(Piece::NIWATORI.0 | PieceColor::BLACK_FLAG.0);
    
    pub const HIYOKO_W: PieceColor = PieceColor(Piece::HIYOKO.0 | PieceColor::WHITE_FLAG.0);
    pub const KIRIN_W: PieceColor = PieceColor(Piece::KIRIN.0 | PieceColor::WHITE_FLAG.0);
    pub const ZOU_W: PieceColor = PieceColor(Piece::ZOU.0 | PieceColor::WHITE_FLAG.0);
    pub const RAION_W: PieceColor = PieceColor(Piece::RAION.0 | PieceColor::WHITE_FLAG.0);
    pub const NIWATORI_W: PieceColor = PieceColor(Piece::NIWATORI.0 | PieceColor::WHITE_FLAG.0);

    pub fn to_piece(self) -> Piece {
        Piece(self.0 & 7)
    }

    pub fn color(self) -> Color {
        Color(self.0 >> 3)
    }

    pub fn is_us(self, c : Color) -> bool {
        (self.0 & (c.0 << 3)) != 0
    }

    pub fn is_ok(pc : PieceColor) -> bool {
        PieceColor::is_piece(pc) || pc == PieceColor::EMPTY
    }
    
    pub fn is_piece(pc : PieceColor) -> bool {
        match pc {
            PieceColor::HIYOKO_B | PieceColor::KIRIN_B  |PieceColor::ZOU_B | PieceColor::NIWATORI_B | PieceColor::RAION_B |
            PieceColor::HIYOKO_W | PieceColor::KIRIN_W  |PieceColor::ZOU_W | PieceColor::NIWATORI_W | PieceColor::RAION_W => { true },
            _ => { false }
        }
    }

    pub fn from_str(s : char) -> PieceColor {
        match s {
            'h' => PieceColor::HIYOKO_W,
            'k' => PieceColor::KIRIN_W,
            'z' => PieceColor::ZOU_W,
            'r' => PieceColor::RAION_W,
            'n' => PieceColor::NIWATORI_W,
            'H' => PieceColor::HIYOKO_B,
            'K' => PieceColor::KIRIN_B,
            'Z' => PieceColor::ZOU_B,
            'R' => PieceColor::RAION_B,
            'N' => PieceColor::NIWATORI_B,
            _ => PieceColor::EMPTY
        }
    }
}

impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceColor::EMPTY => write!(f, " *"),
            PieceColor::WALL => write!(f, "!!!!WALL!!!"),
            PieceColor::HIYOKO_B => write!(f, "^H"),
            PieceColor::KIRIN_B =>  write!(f, "^K"),
            PieceColor::ZOU_B =>  write!(f, "^Z"),
            PieceColor::RAION_B =>  write!(f, "^R"),
            PieceColor::NIWATORI_B =>  write!(f, "^N"),
            PieceColor::HIYOKO_W => write!(f, "vH"),
            PieceColor::KIRIN_W =>  write!(f, "vK"),
            PieceColor::ZOU_W =>  write!(f, "vZ"),
            PieceColor::RAION_W =>  write!(f, "vR"),
            PieceColor::NIWATORI_W =>  write!(f, "vN"),
            _ => write!(f, "PIECE_COLOR:ERROR:{}", self.0),
        }
    }  
}

#[test]
fn test_piece() {
    {
        let p = Piece::HIYOKO;
        assert_eq!(p.to_piece_color(Color::BLACK) , PieceColor::HIYOKO_B);
        assert_eq!(p.to_piece_color(Color::WHITE) , PieceColor::HIYOKO_W);
        
    }
    {
        let p = Piece::KIRIN;
        assert_eq!(p.to_piece_color(Color::BLACK) , PieceColor::KIRIN_B);
        assert_eq!(p.to_piece_color(Color::WHITE) , PieceColor::KIRIN_W);
        
    }
    {
        let p = Piece::ZOU;
        assert_eq!(p.to_piece_color(Color::BLACK) , PieceColor::ZOU_B);
        assert_eq!(p.to_piece_color(Color::WHITE) , PieceColor::ZOU_W);
        
    }
    {
        let p = Piece::RAION;
        assert_eq!(p.to_piece_color(Color::BLACK) , PieceColor::RAION_B);
        assert_eq!(p.to_piece_color(Color::WHITE) , PieceColor::RAION_W);
        
    } 
    {
        let p = Piece::NIWATORI;
        assert_eq!(p.to_piece_color(Color::BLACK) , PieceColor::NIWATORI_B);
        assert_eq!(p.to_piece_color(Color::WHITE) , PieceColor::NIWATORI_W);
        
    } 
}
#[test]
fn test_piece_color() {
    {
        let pb = PieceColor::HIYOKO_B;
        let pw = PieceColor::HIYOKO_W;
        assert_eq!(pb.to_piece() , Piece::HIYOKO);
        assert_eq!(pw.to_piece() , Piece::HIYOKO);
        assert_eq!(pb.color(), Color::BLACK);
        assert_eq!(pw.color(), Color::WHITE);
    } 
    {
        let pb = PieceColor::RAION_B;
        let pw = PieceColor::RAION_W;
        assert_eq!(pb.to_piece() , Piece::RAION);
        assert_eq!(pw.to_piece() , Piece::RAION);
        assert_eq!(pb.color(), Color::BLACK);
        assert_eq!(pw.color(), Color::WHITE);
    } 
    {
        let pb = PieceColor::NIWATORI_B;
        let pw = PieceColor::NIWATORI_W;
        let wall = PieceColor::WALL;
        let empty = PieceColor::EMPTY;

        assert!(pb.is_us(Color::BLACK));
        assert!(pw.is_us(Color::WHITE));
        
        assert!(!pb.is_us(Color::WHITE));
        assert!(!pw.is_us(Color::BLACK));

        assert!(!wall.is_us(Color::BLACK));
        assert!(!wall.is_us(Color::WHITE));
        
        assert!(!empty.is_us(Color::BLACK));
        assert!(!empty.is_us(Color::WHITE));
        
       }
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
pub struct Square(pub i32);
    
impl Square {

    pub const WALL : Square = Square(0);

    pub const A1 : Square = Square(6);
    pub const B1 : Square = Square(7);
    pub const C1 : Square = Square(8);
    pub const A2 : Square = Square(11);
    pub const B2 : Square = Square(12);
    pub const C2 : Square = Square(13);
    pub const A3 : Square = Square(16);
    pub const B3 : Square = Square(17);
    pub const C3 : Square = Square(18);
    pub const A4 : Square = Square(21);
    pub const B4 : Square = Square(22);
    pub const C4 : Square = Square(23);

    pub const SQ_SIZE : usize = 12;
    pub const SQ_ALL_SIZE : usize = 32;
    pub const SQ_DROP : Square = Square(1 << 5);

    pub const INC_W  : Square = Square(-1);
    pub const INC_E  : Square = Square( 1);

    pub const INC_NE : Square = Square(-4);
    pub const INC_SW : Square = Square( 4);

    pub const INC_N  : Square = Square(-5);
    pub const INC_S  : Square = Square( 5);

    pub const INC_NW : Square = Square(-6);
    pub const INC_SE : Square = Square( 6);

    pub const SQ_INDEX : [Square; Square::SQ_SIZE ] = [
        Square::A1, Square::B1, Square::C1,
        Square::A2, Square::B2, Square::C2,
        Square::A3, Square::B3, Square::C3,
        Square::A4, Square::B4, Square::C4
    ];

    pub fn is_ok(sq : Square) -> bool {
        match sq {
            Square::A1 | Square::B1 | Square::C1 | 
            Square::A2 | Square::B2 | Square::C2 | 
            Square::A3 | Square::B3 | Square::C3 | 
            Square::A4 | Square::B4 | Square::C4 => { true },
            _ => { false }
        }
    }
    pub fn reverse(sq : Square) -> Square {
        Square(29 - sq.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::A1 => write!(f, "A1"),
            Square::B1 => write!(f, "B1"),
            Square::C1 => write!(f, "C1"),
            Square::A2 => write!(f, "A2"),
            Square::B2 => write!(f, "B2"),
            Square::C2 => write!(f, "C2"),
            Square::A3 => write!(f, "A3"),
            Square::B3 => write!(f, "B3"),
            Square::C3 => write!(f, "C3"),
            Square::A4 => write!(f, "A4"),
            Square::B4 => write!(f, "B4"),
            Square::C4 => write!(f, "C4"),
            _ => write!(f, "SQUARE_ERROR:{}", self.0),
        }
    }  
}

#[test]
fn test_square() {
    {
        let sq = Square::A1;
        assert!(Square::is_ok(sq));
    }
    {
        let sq = Square(0);
        assert!(!Square::is_ok(sq));
    }
    {
        let sq = Square::A1;
        assert_eq!(Square::reverse(sq),Square::C4);
    }
    {
        let sq = Square::B2;
        assert_eq!(Square::reverse(sq),Square::B3);
    }
}

// * **  *  **  *  **
//  zou    kirn   hiyoko
#[derive(Debug, Copy, Clone)]
pub struct Hand(u32);

impl Hand {
    pub const HAND_NONE : Hand = Hand(0);
    const HAND_SHIFT : [i32; Piece::PIECE_SIZE - 1] = [0, 0, 3, 6, 0];
    const HAND_INC : [u32; Piece::PIECE_SIZE - 1] = [
        1 << Hand::HAND_SHIFT[0],  //empty -> not use
        1 << Hand::HAND_SHIFT[1], // hiyoko
        1 << Hand::HAND_SHIFT[2], // kirin
        1 << Hand::HAND_SHIFT[3], // zou
        1 << Hand::HAND_SHIFT[4], // niwatori -> hiyoko
    ];
    const BORROW_MASK : u32 = (1 << 8) | (1 << 5) | (1 << 2);    

    pub fn new() -> Hand {
        Hand(0)
    }
    pub fn inc(&mut self, p : Piece) {
        assert_ne!(p,Piece::EMPTY);
        assert_ne!(p,Piece::RAION);
        self.0 += Hand::HAND_INC[p.0 as usize];
    }
    pub fn dec(&mut self, p : Piece) {
        assert_ne!(p,Piece::EMPTY);
        assert_ne!(p,Piece::RAION);
        self.0 -= Hand::HAND_INC[p.0 as usize];
    }
    pub fn num(self, p : Piece) -> u32 {
        assert_ne!(p,Piece::EMPTY);
        assert_ne!(p,Piece::RAION);
        (self.0 >> Hand::HAND_SHIFT[p.0 as usize]) & 0x3
    }
    pub fn has(self, p : Piece) -> bool {
        self.num(p) != 0
    }
    pub fn eq(self, h : Hand) -> bool {
        self.0 == h.0
    }
    pub fn win(self, h : Hand) -> bool {
        ((self.0 - h.0) & Hand::BORROW_MASK) == 0
    }
    pub fn val(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str : String = "".to_string();
        str = str + "H:" + &(self.num(Piece::HIYOKO)).to_string();
        str = str + " K:" + &(self.num(Piece::KIRIN)).to_string();
        str = str + " Z:" + &(self.num(Piece::ZOU)).to_string();
        write!(f,"{}",str)
    }  
}

#[test]
fn test_hand() {
    {
        let hd = Hand::new();
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),0);
        assert_eq!(hd.num(Piece::KIRIN),0);
        assert_eq!(hd.num(Piece::ZOU),0);
        
    }
    {
        let mut hd = Hand::new();
        hd.inc(Piece::HIYOKO);
        assert_eq!(hd.0,Hand::HAND_INC[1]*1);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),1);

        hd.inc(Piece::HIYOKO);
        assert_eq!(hd.0,Hand::HAND_INC[1]*2);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),2);

        hd.dec(Piece::HIYOKO);
        assert_eq!(hd.0,Hand::HAND_INC[1]);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),1);

        hd.dec(Piece::HIYOKO);
        assert_eq!(hd.0,0);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),0);
    }
    {
        let mut hd = Hand::new();
        hd.inc(Piece::KIRIN);
        assert_eq!(hd.0,Hand::HAND_INC[2]*1);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::KIRIN),1);

        hd.inc(Piece::KIRIN);
        assert_eq!(hd.0,Hand::HAND_INC[2]*2);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::KIRIN),2);

        hd.dec(Piece::KIRIN);
        assert_eq!(hd.0,Hand::HAND_INC[2]);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::KIRIN),1);

        hd.dec(Piece::KIRIN);
        assert_eq!(hd.0,0);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::KIRIN),0);

    }
    {
        let mut hd = Hand::new();
        hd.inc(Piece::ZOU);
        assert_eq!(hd.0,Hand::HAND_INC[3]*1);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::ZOU),1);

        hd.inc(Piece::ZOU);
        assert_eq!(hd.0,Hand::HAND_INC[3]*2);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::ZOU),2);

        hd.dec(Piece::ZOU);
        assert_eq!(hd.0,Hand::HAND_INC[3]);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::ZOU),1);

        hd.dec(Piece::ZOU);
        assert_eq!(hd.0,0);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::ZOU),0);
    }
    {
        let mut hd = Hand::new();
        hd.inc(Piece::NIWATORI);
        assert_eq!(hd.0,Hand::HAND_INC[1]*1);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),1);

        hd.inc(Piece::NIWATORI);
        assert_eq!(hd.0,Hand::HAND_INC[1]*2);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),2);

        hd.dec(Piece::HIYOKO);
        assert_eq!(hd.0,Hand::HAND_INC[1]);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),1);

        hd.dec(Piece::HIYOKO);
        assert_eq!(hd.0,0);
        assert_eq!(hd.0 & Hand::BORROW_MASK,0);
        assert_eq!(hd.num(Piece::HIYOKO),0);
    }
}
//  ***  ***    *     *****  ******
//  cap  piece  prom  to     from
// drop move -> from : SQ_DROP
// piece is Piece not PieceType

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move(u32);

// TODO NonZeroU32
impl Move {

    const FROM_SHIFT : i32 = 0;
    const TO_SHIFT : i32 = Move::FROM_SHIFT + 6;
    const PROM_SHIFT : i32 = Move::TO_SHIFT + 5;
    const PIECE_SHIFT : i32 = Move::PROM_SHIFT + 1;
    const CAP_SHIFT : i32 = Move::PIECE_SHIFT + 3;

    const FROM_MASK : u32 = 0x3F;
    const PIECE_MASK : u32 = 0x7;
    const TO_MASK : u32 = 0x1F;

    pub const MOVE_NONE : Move = Move(0);

    pub const MOVE_NULL : Move = Move(std::u32::MAX);

    pub fn new() -> Move {
        Move(0)
    }
    pub fn from(self) -> Square {
        Square(((self.0 >> Move::FROM_SHIFT) & Move::FROM_MASK) as i32)
    }
    pub fn to(self) -> Square {
        Square(((self.0 >> Move::TO_SHIFT) & Move::TO_MASK) as i32)    
    }
    pub fn piece(self) -> Piece {
        Piece(((self.0 >> Move::PIECE_SHIFT) & Move::PIECE_MASK) as i32)    
    }
    pub fn cap(self) -> Piece {
        Piece(((self.0 >> Move::CAP_SHIFT) & Move::PIECE_MASK) as i32)
    }
    pub fn prom(self) -> bool {
        self.0 & (1 << Move::PROM_SHIFT) != 0
    }
    pub fn is_drop(self) -> bool {
        self.0 & (Square::SQ_DROP.0 as u32) != 0
    } 

    pub fn is_ok(m : Move) -> bool {
        if m.is_drop() {
            let to = m.to();
            let p = m.piece(); 
            if !Square::is_ok(to) { return false; }
            if !Piece::is_ok(p) { return false; }
        } else {
            let from = m.from();
            let to = m.to();
            let p = m.piece();
            let cap = m.cap();
            if !Square::is_ok(from) { return false; }
            if !Square::is_ok(to) { return false; }
            if !Piece::is_ok(p) { return false; }
            if !Piece::is_ok(cap) { return false; }
        }
        true
    }

}

pub fn make_move(from : Square, to : Square, piece : Piece, cap : Piece, prom : bool) -> Move {
    Move(((from.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((piece.0 as u32) << Move::PIECE_SHIFT) 
        | ((cap.0 as u32) << Move::CAP_SHIFT) | ((prom as u32) << Move::PROM_SHIFT))
}

pub fn make_drop_move(to : Square, piece : Piece) -> Move {
    Move(((Square::SQ_DROP.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((piece.0 as u32) << Move::PIECE_SHIFT))
}

pub fn make_prom_move(from : Square, to : Square, cap : Piece) -> Move {
    Move(((from.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((Piece::HIYOKO.0 as u32) << Move::PIECE_SHIFT) 
    | ((cap.0 as u32) << Move::CAP_SHIFT) | ((1) << Move::PROM_SHIFT))
}

pub fn make_prom_nocap_move(from : Square, to : Square) -> Move {
    Move(((from.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((Piece::HIYOKO.0 as u32) << Move::PIECE_SHIFT) | ((1) << Move::PROM_SHIFT))
}

pub fn make_noprom_move(from : Square, to : Square, piece : Piece, cap : Piece) -> Move {
    Move(((from.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((piece.0 as u32) << Move::PIECE_SHIFT) | ((cap.0 as u32) << Move::CAP_SHIFT))
}

pub fn make_noprom_nocap_mvoe(from : Square, to : Square, piece : Piece) -> Move {
    Move(((from.0 as u32) << Move::FROM_SHIFT) | ((to.0 as u32) << Move::TO_SHIFT) | ((piece.0 as u32) << Move::PIECE_SHIFT))
}


impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Move::MOVE_NONE {
            write!(f,"MOVE_NONE")
        } else if !self.is_drop() {
            let mut str : String = "".to_string();
            str = str + "from:" + &(self.from()).to_string();
            str = str + " to:" + &(self.to()).to_string();
            str = str + " piece:" + &(self.piece()).to_string();
            str = str + " cap:" + &(self.cap()).to_string();
            str = str + " prom:" + &(self.prom()).to_string();
            write!(f,"{}",str)
        } else {
            let mut str : String = "DROP ".to_string();
            str = str + " to:" + &(self.to()).to_string();
            str = str + " piece:" + &(self.piece()).to_string();
            write!(f,"{}",str)
        }
    }  
}


#[test]
fn test_move(){
    {
        let bf = Square::A1;
        let bt = Square::A2;
        let bp = Piece::HIYOKO;
        let bc = Piece::KIRIN;
        let br = false;

        let mv = make_move(bf,bt,bp,bc,br);
        let af = mv.from();
        let at = mv.to();
        let ap = mv.piece();
        let ac = mv.cap();
        let ar = mv.prom();

        assert_eq!(bf,af);
        assert_eq!(bt,at);
        assert_eq!(bp,ap);
        assert_eq!(bc,ac);
        assert_eq!(br,ar);
    }
    {
        let bf = Square::B3;
        let bt = Square::C4;
        let bp = Piece::NIWATORI;
        let bc = Piece::NIWATORI;
        let br = false;

        let mv = make_move(bf,bt,bp,bc,br);
        let af = mv.from();
        let at = mv.to();
        let ap = mv.piece();
        let ac = mv.cap();
        let ar = mv.prom();

        assert_eq!(bf,af);
        assert_eq!(bt,at);
        assert_eq!(bp,ap);
        assert_eq!(bc,ac);
        assert_eq!(br,ar);
    }
    {
        let bt = Square::A2;
        let bp = Piece::HIYOKO;
        
        let mv = make_drop_move(bt,bp);
      
        let at = mv.to();
        let ap = mv.piece();
      
        assert_eq!(bt,at);
        assert_eq!(bp,ap);
        assert_eq!(mv.is_drop(),true);
    }
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
pub struct Score(pub i32);
impl Score {
    pub const  SCORE_MIN  : Score = Score(-32000);
    pub const  SCORE_MAX  : Score = Score( 32000);
    pub const  SCORE_NONE : Score = Score(0);
    pub const  EVAL_MIN   : Score = Score(-30000);
    pub const  EVAL_MAX   : Score = Score( 30000);
    pub const  SCORE_REP  : Score = Score( 30001);
    pub fn in_mate(ply : i32) -> Score {
        Score(Score::EVAL_MAX.0 - ply)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BitXor, BitXorAssign, Hash)]
pub struct Key(pub u64);