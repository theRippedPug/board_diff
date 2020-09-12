use super::chess_structs::{BoardRep, AbstractMoveList, ParseError, PhysicalPos, PhysicalMove, ChessPiece};

struct PathNode{
	pos: PhysicalPos,
	dist: u64,
	path: Vec<PhysicalPos>,
	penalty_reset: Vec<PhysicalPos>
}



lazy_static!{
	static ref reverve_pieces: std::sync::Mutex<i8> = std::sync::Mutex::new(5);
}

// pub fn parse(board: BoardRep, move_list: AbstractMoveList) -> Result<Vec<PhysicalMove>, ParseError>{
	
// }

fn find_surround() -> Vec<PathNode>{

}

fn get_ambient_pieces(board: BoardRep, pos: PhysicalPos) -> u8{
	let mut pawn_count: u8 = 0;

	//you are on a diagonal, check for pieces on all 4 sides
	if pos.1.fract().abs() != 0.0 && pos.0.fract().abs() != 0.0{
		if board[pos.1.floor() as usize][pos.0.floor() as usize].is_some(){
			pawn_count += 1;
		}
		if board[pos.1.floor() as usize][pos.0.ceil() as usize].is_some(){
			pawn_count += 1;
		}
		if board[pos.1.ceil() as usize][pos.0.floor() as usize].is_some(){
			pawn_count += 1;
		}
		if board[pos.1.ceil() as usize][pos.0.ceil() as usize].is_some(){
			pawn_count += 1;
		}
	}else if pos.1.fract().abs() != 0.0{
		if
	}

	pawn_count
}

