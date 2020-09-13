use super::chess_structs::*;
use std::collections::{VecDeque};
struct PathNode{
	pos: PhysicalPos,
	dist: u64,
	path: Vec<PhysicalPos>,
	penalty_reset: Vec<PhysicalPos>
}
impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}


lazy_static!{
	static ref reverve_pieces: std::sync::Mutex<i8> = std::sync::Mutex::new(5);
}

pub fn parse(board: BoardRep, move_list: Vec<AbstractMove>) -> Result<(Vec<PhysicalMove>, Vec<PhysicalPos>), ParseError>{
	let mut physical_move_list: Vec<PhysicalMove> = Vec::new();
	let mut reset_positions: Vec<PhysicalPos> = Vec::new();

	for mov in move_list{
		//deal with castling
		match mov {
			AbstractMove::Catstling((p1,p2)) => {
				
			},
			AbstractMove::RegularMove(mv) => {
				match mv.origin{
					PieceOrigin::Existing(start_pos) =>{
						match mv.dest {
							PieceDest::OnBoard(end_pos) =>{
								// Move from board to board

							},
							PieceDest::Disposed => {
								// From board to bin
							}
						}
					},
					PieceOrigin::Reserve => {
						match mv.dest {
							PieceDest::OnBoard(end_pos) => {
								// Move from pawn reserve to board
							},
							PieceDest::Disposed => {
								// Why would you move a piece from reverve straight to to the bin??
								return Err(ParseError{})
							}
						}
					}
				}
			}
		}
	}
	Ok((physical_move_list, reset_positions))
}

// This is a modified dejkstara implementation thats too comple to explain in comments 
//checkout the dijkstra file in MDocs folder for detailed explanation
fn dijkstra_path(board: BoardRep, start_pos: AbstractPos) -> (Vec<PhysicalPos>, Vec<PhysicalPos>) {

	board[start_pos.1 as usize][start_pos.0 as usize] = None;

	let mut node_list: Vec<PathNode> = Vec::new();
	node_list.push(PathNode{
		pos: (start_pos.0 as f32, start_pos.1 as f32),
		path: Vec::new(),
		dist: 0,
		penalty_reset: Vec::new()
	});
	loop{

		//Step 1 (termination condition):
		if node_list[0].pos == (-1.0,3.0){
			let final_node = node_list[0];
			break (final_node.path, final_node.penalty_reset);
		}

		//Step 3:
		let first_node = node_list.remove(0);

		//Step 4:
		node_list.append(&mut find_surround_nodes(&board, first_node));

		//Step 5:
		node_list.sort_unstable_by_key(|a|{
			a.dist + (a.penalty_reset.len() * 25) as u64
		});

		//Step 6:
		node_list.dedup();

	}
}

//Step 2:
fn find_surround_nodes(board: &BoardRep, node: PathNode) -> Vec<PathNode>{
	let mut surround_nodes = Vec::new();
}

fn try_get_node(board: &BoardRep, node: PathNode, pos: PhysicalPos) -> PathNode{

}

fn get_ambient_pieces(board: &BoardRep, pos: PhysicalPos) -> u8{
	let mut pawn_count: u8 = 0;

	//you are on a diagonal, check for pieces on all 4 sides
	if pos.1.fract() != 0.0 && pos.0.fract() != 0.0{
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
		if board[pos.1.floor() as usize][pos.0 as usize].is_some(){
			pawn_count += 1;
		}
		if pos.1 < 7.0{
		if board[pos.1.ceil() as usize][pos.0 as usize].is_some(){
			pawn_count += 1;
		}}
	}else if pos.0.fract().abs() != 0.0{
		if board[pos.1 as usize][pos.0.floor() as usize].is_some(){
			pawn_count += 1;
		}
		if pos.0 < 7.0{
		if board[pos.1 as usize][pos.0.ceil() as usize].is_some(){
			pawn_count += 1;
		}}
	}

	pawn_count
}

