
use crate::common::*;
use crate::position::*;

/*pub const ATTACK_SQ : [ [[Square ; 8 ]; Piece::PIECE_SIZE] ; Color::COLOR_SIZE ] = [
    [
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
    ],
    [
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::INC_N, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//HIYOKO
        [Square::INC_N, Square::INC_W, Square::INC_S, Square::INC_E, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//KIRIN
        [Square::INC_NW, Square::INC_SW, Square::INC_SE, Square::INC_NE, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//ZOU
        [Square::INC_N, Square::INC_NW, Square::INC_E, Square::INC_S, Square::INC_S, Square::INC_NE, Square::WALL, Square::WALL],//NIWATORI
        [Square::INC_N, Square::INC_NW, Square::INC_E, Square::INC_SW, Square::INC_S, Square::INC_SE, Square::INC_S, Square::INC_NE],//RAION
    ],
    [
        [Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//EMPTY
        [Square::INC_S, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//HIYOKO
        [Square::INC_N, Square::INC_W, Square::INC_S, Square::INC_E, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//KIRIN
        [Square::INC_NW, Square::INC_SW, Square::INC_SE, Square::INC_NE, Square::WALL, Square::WALL, Square::WALL, Square::WALL],//ZOU
        [Square::INC_S, Square::INC_SE, Square::INC_E, Square::INC_N, Square::INC_W, Square::INC_SW, Square::WALL, Square::WALL],//NIWATORI
        [Square::INC_N, Square::INC_NW, Square::INC_E, Square::INC_SW, Square::INC_S, Square::INC_SE, Square::INC_S, Square::INC_NE],//RAION
    ]
];
pub const ATTACK_NUM : [usize; Piece::PIECE_SIZE] = [0, 1, 4, 4, 8, 6];
*/
pub fn is_win(pos : &Position) -> bool {
    //トライ
    if      pos.turn() == Color::BLACK && pos.raion_sq(Color::BLACK) <= Square::C1 { return true; }
    else if pos.turn() == Color::WHITE && pos.raion_sq(Color::WHITE) >= Square::A4 { return true; }
    let opp = Color::flip(pos.turn());
    //ライオンが取れる
    is_attack(pos.raion_sq(opp),pos.turn(),pos)
}

pub fn in_checked(pos : &Position) -> bool {
    is_attacked(pos.raion_sq(pos.turn()),pos)
}

pub fn is_attacked(to : Square, pos : &Position) -> bool {
    is_attack(to, Color::flip(pos.turn()),pos)
}

pub fn is_attack(to : Square, attacker : Color ,pos : &Position) -> bool {
    if attacker == Color::BLACK {
        let from = to + Square::INC_S;
        let pc = pos.square(from);
        if pc == PieceColor::HIYOKO_B || pc == PieceColor::KIRIN_B || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_N;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_B  || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_E;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_B || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_W;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_B || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_SW;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_B || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_SE;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_B || pc == PieceColor::NIWATORI_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_NW;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_B || pc == PieceColor::RAION_B {
            return true;
        }
        let from = to + Square::INC_NE;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_B || pc == PieceColor::RAION_B {
            return true;
        }
    } else {
        let from = to + Square::INC_N;
        let pc = pos.square(from);
        if pc == PieceColor::HIYOKO_W   || pc == PieceColor::KIRIN_W  || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_S;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_W || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_E;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_W || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_W;
        let pc = pos.square(from);
        if pc == PieceColor::KIRIN_W || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_NW;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_W || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_NE;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_W || pc == PieceColor::NIWATORI_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_SW;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_W || pc == PieceColor::RAION_W {
            return true;
        }
        let from = to + Square::INC_SE;
        let pc = pos.square(from);
        if pc == PieceColor::ZOU_W || pc == PieceColor::RAION_W {
            return true;
        }
    }
    false
}

#[test]
fn test_attack() {
    macro_rules! test_is_win {
        ($sfen:expr, $result:expr) => {
            {
                let pos = Position::init_sfen($sfen);
                println!("{}",pos);
                assert_eq!(is_win(&pos),$result);
            }
        };
    }
    test_is_win!(START_SFEN,false);

    test_is_win!("R2/3/r2/3 b hkzHKZ",true);
    test_is_win!("R2/3/3/r2 w hkzHKZ",true);
    test_is_win!("1r1/3/3/1R1 w hkzHKZ",false);

    test_is_win!("3/1r1/1R1/3 w hkzHKZ",true);
    test_is_win!("3/1r1/1R1/3 b hkzHKZ",true);

    test_is_win!("1r1/1H1/3/1R1 b hkzKZ",true);
    test_is_win!("1r1/1K1/3/1R1 b hkzHZ",true);
    test_is_win!("1r1/Z2/3/1R1 b hkzKHK",true);
    test_is_win!("1r1/1N1/3/1R1 b hkzKZ",true);
    test_is_win!("1r1/N2/3/1R1 b hkzKZ",true);

    test_is_win!("1r1/H2/3/1R1 b hkzKZ",false);
    test_is_win!("1r1/K2/3/1R1 b hkzHZ",false);
    test_is_win!("1r1/1Z1/3/1R1 b hkzKHK",false);
    test_is_win!("N2/1r1/3/1R1 b hkzKHK",false);

    test_is_win!("1r1/3/1h1/1R1 w hkzHKZ",true);
    test_is_win!("1r1/3/1k1/1R1 w hkzHKZ",true);
    test_is_win!("1r1/3/2z/1R1 w hkzKHKZ",true);
    test_is_win!("1r1/3/1n1/1R1 w hkzHKZ",true);
    test_is_win!("1r1/3/n2/1R1 w hkzHKZ",true);

    test_is_win!("1r1/3/2h/1R1 w hkzHKZ",false);
    test_is_win!("1r1/3/2k/1R1 w hkzHKZ",false);
    test_is_win!("1r1/3/1z1/1R1 w hkzKHKZ",false);
    test_is_win!("1r1/3/1R1/n2 w hkzKHK",false);

}