#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChessPieceType {
	Pawn = 0,
	Bishop = 1,
	Knight = 2,
	Rook = 3,
	Queen = 4,
	King = 5,
}

//struct that describes position of pawn on chessboard
//x goes from 1 to 8 -and- y from 'a' to 'h'
#[derive(Debug)]
pub struct PiecePos {
	pub x: i16,
	pub y: char,
}

//describes type and position of a piece
//the type goes from 0 to 5 in and this order(
//	pawn,bishop,knight,rook,queen,king
// )

#[derive(Debug,Copy, Clone, Eq, PartialEq)]
pub struct ChessPiece {
	pub piece_color: char,
	pub piece_type: ChessPieceType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceDest {
	Disposed,
	OnBoard(PieceCoord),
	OffToSide(PieceCoord),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceOrigin {
	New,
	Existing(PieceCoord),
	OffToSide(PieceCoord),
}

pub struct AbstractPhysicalMove {
	pub origin: PieceOrigin,
	pub dest: PieceDest,
}

pub enum MoveType{
	Relocation,
	Capture,
	Promotion,
	CapturePromotion,
	ElPassaunt,
	Castling
}

pub type BoardRep = [[Option<ChessPiece>; 8]; 8];
pub type PieceCoord = (usize, usize);
