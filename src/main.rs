use chess::{Board, ChessMove, Color, Square};
pub mod chess_structs;
use chess_structs::{AbstractPhysicalMove, BoardRep, PieceCoord, PieceDest, PieceOrigin};
pub mod reneder_hepler;

pub fn get_diff(board1: BoardRep, board2: BoardRep) -> Vec<PieceCoord> {
	let mut mismatch_list = Vec::<PieceCoord>::new();
	for i in 0..8 {
		for j in 0..8 {
			if board1[i][j] != board2[i][j] {
				mismatch_list.push((j, i))
			}
		}
	}
	mismatch_list
}

pub fn get_move_list(board1: BoardRep, board2: BoardRep) -> Vec<AbstractPhysicalMove> {
	let points = get_diff(board1, board2);
	let p1 = points[0];
	let p2 = points[1];

	//first two ifs are for relocation move
	if (board1[p1.1][p1.0].is_none())
		&& (board2[p1.1][p1.0].is_some())
		&& (board1[p2.1][p2.0].is_some())
		&& (board2[p2.1][p2.0].is_none())
	{
		return vec![AbstractPhysicalMove {
			origin: PieceOrigin::Existing(p2),
			dest: PieceDest::OnBoard(p1),
		}];
	} else if (board1[p2.1][p2.0].is_none())
		&& (board2[p2.1][p2.0].is_some())
		&& (board1[p1.1][p1.0].is_some())
		&& (board2[p1.1][p1.0].is_none())
	{
		return vec![AbstractPhysicalMove {
			origin: PieceOrigin::Existing(p1),
			dest: PieceDest::OnBoard(p2),
		}];
	//these two ifs are for capture condition
	} else if board1[p1.1][p1.0].is_some()
		&& board1[p2.1][p2.0].is_some()
		&& board2[p1.1][p1.0].is_some()
		&& board2[p2.1][p2.0].is_none()
	{
		return vec![
			AbstractPhysicalMove {
				origin: PieceOrigin::Existing(p1),
				dest: PieceDest::Disposed,
			},
			AbstractPhysicalMove {
				origin: PieceOrigin::Existing(p2),
				dest: PieceDest::OnBoard(p1),
			},
		];
	} else if board1[p1.1][p1.0].is_some()
		&& board1[p2.1][p2.0].is_some()
		&& board2[p1.1][p1.0].is_none()
		&& board2[p2.1][p2.0].is_some()
	{
		return vec![
			AbstractPhysicalMove {
				origin: PieceOrigin::Existing(p2),
				dest: PieceDest::Disposed,
			},
			AbstractPhysicalMove {
				origin: PieceOrigin::Existing(p1),
				dest: PieceDest::OnBoard(p2),
			},
		];
	} else {
		if board1[p1.1][p1.0].unwrap().piece_type == chess_structs::ChessPieceType::Rook {
			vec![
				AbstractPhysicalMove {
					origin: PieceOrigin::Existing(p1),
					dest: PieceDest::OffToSide(p1),
				},
				AbstractPhysicalMove {
					origin: PieceOrigin::Existing(p2),
					dest: PieceDest::OnBoard(p1),
				},
				AbstractPhysicalMove {
					origin: PieceOrigin::OffToSide(p1),
					dest: PieceDest::OnBoard(p2),
				},
			]
		} else {
			//TODO: write an errormessage nad send it to error channel
			Vec::<AbstractPhysicalMove>::new()
		}
	}
}

fn main() {
	let board1 = Board::default();

	let m = ChessMove::new(Square::D2, Square::D4, None);

	let board2 = board1.make_move_new(m);

	let s1 = reneder_hepler::getMatrix(&board1);
	let s2 = reneder_hepler::getMatrix(&board2);

	println!("{:?}", get_diff(s1, s2));
}
