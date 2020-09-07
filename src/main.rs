//Remove when done
#![allow(unused_imports)]
//Remove when done

use chess::{Board, ChessMove, Color, Square};
pub mod chess_structs;
use chess_structs::{AbstractMove, BoardRep, PieceCoord, PieceDest, PieceOrigin};
pub mod abstract_move_parser;
pub mod reneder_hepler;

fn main() {
	let board1 = Board::default();

	let m = ChessMove::new(Square::D2, Square::D4, None);

	let board2 = board1.make_move_new(m);

	let s1 = reneder_hepler::get_matrix(&board1);
	let s2 = reneder_hepler::get_matrix(&board2);

	println!("{:?}", abstract_move_parser::get_diff(&s1, &s2));

	println!(
		"{:?} , {:?} , {:?}",
		board1.piece_on(Square::E8),
		board1.color_on(Square::E8),
		reneder_hepler::get_matrix(&board1)[7][4]
	);
}
#[test]
fn el_passauntest() {
	let board_init = Board::default();
	let m1 = ChessMove::new(Square::A2, Square::A4, None);
	let m2 = ChessMove::new(Square::E7, Square::E6, None);
	let m3 = ChessMove::new(Square::A4, Square::A5, None);
	let m4 = ChessMove::new(Square::B7, Square::B5, None);
	let boardt1 = board_init.make_move_new(m1);
	let boardt2 = boardt1.make_move_new(m2);
	let boardt3 = boardt2.make_move_new(m3);
	let boardt4 = boardt3.make_move_new(m4);

	let mep = ChessMove::new(Square::A5, Square::B6, None);

	let boardt5 = boardt4.make_move_new(mep);

	let s1 = reneder_hepler::get_matrix(&boardt4);
	let s2 = reneder_hepler::get_matrix(&boardt5);

	println!("{:?}", abstract_move_parser::get_diff(&s1, &s2));
	println!("{:?}", abstract_move_parser::parse(s1, s2));
}