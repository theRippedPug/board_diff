
## These are pieces of code I refuse to delete


### These ones are from abstract move parser
```Rust
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
```