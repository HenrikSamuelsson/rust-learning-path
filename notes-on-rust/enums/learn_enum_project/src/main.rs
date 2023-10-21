use::core::fmt;

#[derive(Debug, Clone, Copy)]
enum ChessPiece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChessPiece::Pawn => write!(f, "pawn"),
            ChessPiece::Bishop => write!(f, "bishop"),
            ChessPiece::Knight => write!(f, "knight"),
            ChessPiece::Rook => write!(f, "rook"),
            ChessPiece::Queen => write!(f, "queen"),
            ChessPiece::King => write!(f, "king"),
        }
    }
}

fn main() {
    let piece = ChessPiece::Pawn;
    println!("Our chess piece is a {}", piece);
}
