enum Color {
    Black,
    White
}

#[derive(Debug)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

struct Piece<'a> {
    id: &'a str,
    piece_type: PieceType,
    piece_color: Color
}

impl <'a> Piece<'a> {
    fn new(id: &'a str, piece_type: PieceType, piece_color: Color) -> Self {
        Piece {
            id,
            piece_type,
            piece_color
        }
    }
}

fn main() {
    let game_finished = false;

    let w_tower1 = Piece::new("WT1", PieceType::Rook, Color::White);
    let w_knight1 = Piece::new("WK1", PieceType::Knight, Color::White);
    let w_bishop1 = Piece::new( "WB1", PieceType::Bishop, Color::White);
    let w_king = Piece::new("WK1", PieceType::King, Color::White);
    let w_queen = Piece::new("WQ1", PieceType::Queen, Color::White);
    let w_bishop2 = Piece::new("WB2", PieceType::Bishop, Color::White);
    let w_knight2 = Piece::new("WK2", PieceType::Knight, Color::White);
    let w_tower2 = Piece::new("WT2", PieceType::Rook, Color::White);
    let w_pawn1 = Piece::new("WP1", PieceType::Pawn, Color::White);
    let w_pawn2 = Piece::new("WP2", PieceType::Pawn, Color::White);
    let w_pawn3 = Piece::new("WP3", PieceType::Pawn, Color::White);
    let w_pawn4 = Piece::new("WP4", PieceType::Pawn, Color::White);
    let w_pawn5 = Piece::new("WP5", PieceType::Pawn, Color::White);
    let w_pawn6 = Piece::new("WP6", PieceType::Pawn, Color::White);
    let w_pawn7 = Piece::new("WP7", PieceType::Pawn, Color::White);
    let w_pawn8 = Piece::new("WP8", PieceType::Pawn, Color::White);
    
    let b_pawn1 = Piece::new("BP1", PieceType::Pawn, Color::Black);
    let b_pawn2 = Piece::new("BP2", PieceType::Pawn, Color::Black);
    let b_pawn3 = Piece::new("BP3", PieceType::Pawn, Color::Black);
    let b_pawn4 = Piece::new("BP4", PieceType::Pawn, Color::Black);
    let b_pawn5 = Piece::new("BP5", PieceType::Pawn, Color::Black);
    let b_pawn6 = Piece::new("BP6", PieceType::Pawn, Color::Black);
    let b_pawn7 = Piece::new("BP7", PieceType::Pawn, Color::Black);
    let b_pawn8 = Piece::new("BP8", PieceType::Pawn, Color::Black);
    let b_tower1 = Piece::new("BT1", PieceType::Rook, Color::Black);
    let b_knight1 = Piece::new("BK1", PieceType::Knight, Color::Black);
    let b_bishop1 = Piece::new("BB1", PieceType::Bishop, Color::Black);
    let b_king = Piece::new("BK1", PieceType::King, Color::Black);
    let b_queen = Piece::new("BQ1", PieceType::Queen, Color::Black);
    let b_bishop2 = Piece::new("BB2", PieceType::Bishop, Color::Black);
    let b_knight2 = Piece::new("BK2", PieceType::Knight, Color::Black);
    let b_tower2 = Piece::new("BT2", PieceType::Rook, Color::Black);

    let board_piece: [[Option<Piece>; 8]; 8] = [
        [Some(w_tower1), Some(w_knight1), Some(w_bishop1), Some(w_king), Some(w_queen), Some(w_bishop2), Some(w_knight2), Some(w_tower2)],
        [Some(w_pawn1), Some(w_pawn2), Some(w_pawn3), Some(w_pawn4), Some(w_pawn5), Some(w_pawn6), Some(w_pawn7), Some(w_pawn8)],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [Some(b_pawn1), Some(b_pawn2), Some(b_pawn3), Some(b_pawn4), Some(b_pawn5), Some(b_pawn6), Some(b_pawn7), Some(b_pawn8)],
        [Some(b_tower1), Some(b_knight1), Some(b_bishop1), Some(b_king), Some(b_queen), Some(b_bishop2), Some(b_knight2), Some(b_tower2)],
        ];
    
    let board_color: [[Color; 8]; 8] = [
        [Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black],
        [Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White],
        [Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black],
        [Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White],
        [Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black],
        [Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White],
        [Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black],
        [Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White, Color::Black, Color::White],
        ];
    
    display_board(&board_piece);
}

fn display_board(board_piece: &[[Option<Piece>; 8]; 8]) {
    println!("---------------------------------");
    for i in 0..8 {
        print!("|");
        for j in 0..8 {
            if let Some(piece) = &board_piece[i][j] {
                print!("{}|", piece.id);
            } else {
                print!("...|");
            }
        }
        println!(" {} ", i+1);
    }
    println!("---------------------------------");
    println!("  H   G   F   E   D   C   B   A")
}

fn minmax() {

}