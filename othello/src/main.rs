#![allow(unused_variables)]

#![allow(dead_code)]

use std::fmt;
use std::collections::HashMap;



#[derive(Clone, Debug)]
enum BoardSquareState {
    None,
    EMPTY(i32,i32),
    BLACK(i32,i32),
    WHITE(i32,i32),
}



#[derive(Clone)]
struct OthelloBoard {
    num_rows: u8,
    board: HashMap<(i32, i32), BoardSquareState>,
}

static NULL_SQUARE : BoardSquareState = BoardSquareState::None;

impl OthelloBoard {
     fn get_row_vec(&self, row: i32) -> Vec<&BoardSquareState> {

	    let mut sortedrow: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == row)
	    .collect::<Vec<&(i32, i32)>>();


        sortedrow.sort_by(|&a, &b| a.1.cmp(&b.1));

        let sorted_values = sortedrow.iter().map(|key| {
        		match self.board.get(key){
        			Some(m)  => m,
        			_ => &NULL_SQUARE
        		}
        	}
        	)
        .collect::<Vec<_>>();

        sorted_values
    }
    pub fn pretty_print(&self){
        for row in 0..7{
          let sorted_values = self.get_row_vec(row);
          for square in sorted_values{
                match square{
                    &BoardSquareState::EMPTY(a,b) => print!( "|{}[1;30;42m {}[0m",27 as char ,27 as char),
                    &BoardSquareState::BLACK(a,b) => print!( "|{}[1;30;42mB{}[0m",27 as char ,27 as char),
                    &BoardSquareState::WHITE(a,b) => print!( "|{}[1;97;42mW{}[0m",27 as char ,27 as char),
                    _ => println!(""),
               }
             }
         print!("\n");
        }
    }
    pub fn new(size: i32) -> OthelloBoard {

        let mut map = HashMap::new();
        let top_left_col = (size - 2) / 3 + 1;
        for x in 0..size {
            for y in 0..size {
                let tup = (x, y);
                match tup {
                    (k, l) if ((k == top_left_col) && (l == top_left_col)) ||
                              ((k == top_left_col + 1) && (l == top_left_col + 1)) => {
                        let gg = (x, y);
                        map.insert(gg, BoardSquareState::WHITE(x,y));
                    }
                    (k, l) if ((k == top_left_col + 1) && (l == top_left_col)) ||
                              ((k == top_left_col) && (l == top_left_col + 1)) => {
                        let gg = (x, y);
                        map.insert(gg, BoardSquareState::BLACK(x,y));
                    }
                    (k, l) => {
                        let gg = (x, y);
                        map.insert(gg, BoardSquareState::EMPTY(x,y));
                    }
                }
            }
        }
        OthelloBoard { num_rows: size as u8, board: map }
    }
}
impl fmt::Debug for OthelloBoard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    for row in 0 .. self.num_rows-1{
        let sorted_row = self.get_row_vec(row as i32);
        write!(f, "{:?}\n",sorted_row);
    }
        let sorted_row= self.get_row_vec((self.num_rows - 1) as i32);
        write!(f, "{:?}\n",sorted_row)

	}
}



fn main() {

    let n = 4;
    let size: i32 = 2 * n;
    let board = OthelloBoard::new(size);
    println!("board=\n{:?}", board);

    board.pretty_print();

}
