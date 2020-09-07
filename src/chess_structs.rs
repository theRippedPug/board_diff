use chess::{File, Rank, Square};
use std::fmt;

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

//a struct that represents position in a way thats mathematically workable
pub type ActualPos = (u8, u8);

trait ToActualPos {
	fn to_actual_pos(&self) -> ActualPos;
}

impl ToActualPos for Square {
	fn to_actual_pos(&self) -> ActualPos {
		(
			match self.get_rank() {
				Rank::First => 0,
				Rank::Second => 1,
				Rank::Third => 2,
				Rank::Fourth => 3,
				Rank::Fifth => 4,
				Rank::Sixth => 5,
				Rank::Seventh => 6,
				Rank::Eighth => 7,
			},
			match self.get_file() {
				File::A => 0,
				File::B => 1,
				File::C => 2,
				File::D => 3,
				File::E => 4,
				File::F => 5,
				File::G => 6,
				File::H => 7,
			},
		)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ChessPiece {
	pub piece_color: char,
	pub piece_type: ChessPieceType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceDest {
	Disposed,
	OnBoard(ActualPos),
	OffToSide,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceOrigin {
	Existing(ActualPos),
	Reserve,
}
#[derive(Debug)]
pub enum AbstractMove {
	RegularMove(RegularMove),
	Catstling((ActualPos, ActualPos)),
}

#[derive(Debug)]
pub struct RegularMove {
	pub origin: PieceOrigin,
	pub dest: PieceDest,
}

#[derive(Debug)]
pub struct ParseError {}
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "SuperError is here!")
	}
}
impl std::error::Error for ParseError {}

#[derive(Debug)]
pub enum MoveType {
	Relocation,
	Capture,
	Promotion,
	CapturePromotion,
	ElPassaunt,
	Castling,
}

pub type BoardRep = [[Option<ChessPiece>; 8]; 8];
pub type PieceCoord = (usize, usize);
