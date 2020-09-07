// use chess::{Board, ChessMove, Color, Square};
use super::chess_structs::{
	AbstractMove, BoardRep, ParseError, PieceCoord, PieceDest, PieceOrigin,
};

// pub fn get_move_list(board1: BoardRep, board2: BoardRep) -> Vec<AbstractPhysicalMove> {
// 	let points = get_diff(board1, board2);
// 	let p1 = points[0];
// 	let p2 = points[1];

// 	//first two ifs are for relocation move
// 	if (board1[p1.1][p1.0].is_none())
// 		&& (board2[p1.1][p1.0].is_some())
// 		&& (board1[p2.1][p2.0].is_some())
// 		&& (board2[p2.1][p2.0].is_none())
// 	{
// 		return vec![AbstractPhysicalMove {
// 			origin: PieceOrigin::Existing(p2),
// 			dest: PieceDest::OnBoard(p1),
// 		}];
// 	} else if (board1[p2.1][p2.0].is_none())
// 		&& (board2[p2.1][p2.0].is_some())
// 		&& (board1[p1.1][p1.0].is_some())
// 		&& (board2[p1.1][p1.0].is_none())
// 	{
// 		return vec![AbstractPhysicalMove {
// 			origin: PieceOrigin::Existing(p1),
// 			dest: PieceDest::OnBoard(p2),
// 		}];
// 	//these two ifs are for capture condition
// 	} else if board1[p1.1][p1.0].is_some()
// 		&& board1[p2.1][p2.0].is_some()
// 		&& board2[p1.1][p1.0].is_some()
// 		&& board2[p2.1][p2.0].is_none()
// 	{
// 		return vec![
// 			AbstractPhysicalMove {
// 				origin: PieceOrigin::Existing(p1),
// 				dest: PieceDest::Disposed,
// 			},
// 			AbstractPhysicalMove {
// 				origin: PieceOrigin::Existing(p2),
// 				dest: PieceDest::OnBoard(p1),
// 			},
// 		];
// 	} else if board1[p1.1][p1.0].is_some()
// 		&& board1[p2.1][p2.0].is_some()
// 		&& board2[p1.1][p1.0].is_none()
// 		&& board2[p2.1][p2.0].is_some()
// 	{
// 		return vec![
// 			AbstractPhysicalMove {
// 				origin: PieceOrigin::Existing(p2),
// 				dest: PieceDest::Disposed,
// 			},
// 			AbstractPhysicalMove {
// 				origin: PieceOrigin::Existing(p1),
// 				dest: PieceDest::OnBoard(p2),
// 			},
// 		];
// 	} else {
// 		if board1[p1.1][p1.0].unwrap().piece_type == chess_structs::ChessPieceType::Rook {
// 			vec![
// 				AbstractPhysicalMove {
// 					origin: PieceOrigin::Existing(p1),
// 					dest: PieceDest::OffToSide(p1),
// 				},
// 				AbstractPhysicalMove {
// 					origin: PieceOrigin::Existing(p2),
// 					dest: PieceDest::OnBoard(p1),
// 				},
// 				AbstractPhysicalMove {
// 					origin: PieceOrigin::OffToSide(p1),
// 					dest: PieceDest::OnBoard(p2),
// 				},
// 			]
// 		} else {
// 			//TODO: write an errormessage nad send it to error channel
// 			Vec::<AbstractPhysicalMove>::new()
// 		}
// 	}
// }

pub fn get_diff(board1: &BoardRep, board2: &BoardRep) -> Vec<PieceCoord> {
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

pub fn parse(board1: BoardRep, board2: BoardRep) -> Result<Vec<AbstractMove>, ParseError> {
	let diffs = get_diff(&board1, &board2);
	let mut finalvec = Vec::new();
	match diffs.len() {
		3 => {//it's el passaunt move
			// we need to find the pawn that moved
			// first iterate through the different squares on both boards (so nested loop)
			// till you find where the piece that moved went.
			'outer: for i in &diffs {
				// this check is to ensure the algorithm does'nt conclude that
				// it was the empty square that moved, we wanna get a piece ONLY
				if board1[i.1][i.0].is_some() {
					for j in &diffs {
						// so after this conditional, we have the same picece
						// in two different boards,BINGO! Now we add the generate
						// the move for the piece that does the capturing
						if board1[i.1][i.0] == board2[j.1][j.0] {
							finalvec.push(AbstractMove {
								origin: PieceOrigin::Existing((i.0 as u8, i.1 as u8)),
								dest: PieceDest::OnBoard((j.0 as u8, j.1 as u8)),
							});
							// now we know piece at i(in board1) is the pawn that captures
							// time to find the one that gets captured
							for k in &diffs {
								// NOTE:the following conditional is written with respect 
								// to board 1 positions
								// we know this piece is not the pice that captures
								// (so its not at i) also it's not an empty
								if k != i && board1[k.1][k.0].is_some() {
									// now we generate the remove the piece that gets captured
									finalvec.push(AbstractMove {
										origin: PieceOrigin::Existing((k.0 as u8, k.1 as u8)),
										dest: PieceDest::OffToSide,
									})
								}
							}
							break 'outer;
						}
					}
				}
			}
			Ok(finalvec)
		}
		2 => Err(ParseError {}),
		_ => Err(ParseError {}),
	}
}

