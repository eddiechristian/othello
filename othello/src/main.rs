#![allow(unused_variables)]

#![allow(dead_code)]

use std::fmt;
use std::collections::HashMap;



#[derive(Clone, Debug)]
enum BoardSquareState {
    EMPTY,
    BLACK,
    WHITE,
}



#[derive(Clone)]
struct OthelloBoard {
    board: HashMap<(i32, i32), BoardSquareState>,
}

impl OthelloBoard {
    fn test(&self, _row: i32) -> Vec<&(i32, i32)> {

	    let mut sortedrow0: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 0)
	    .collect::<Vec<&(i32, i32)>>();
	   
        
        sortedrow0.sort_by(|&a, &b| a.1.cmp(&b.1));


        sortedrow0
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
                        map.insert(gg, BoardSquareState::WHITE);
                    }
                    (k, l) if ((k == top_left_col + 1) && (l == top_left_col)) ||
                              ((k == top_left_col) && (l == top_left_col + 1)) => {
                        let gg = (x, y);
                        map.insert(gg, BoardSquareState::BLACK);
                    }
                    (k, l) => {
                        let gg = (x, y);
                        map.insert(gg, BoardSquareState::EMPTY);
                    }
                }
            }
        }
        OthelloBoard { board: map }
    }
}
impl fmt::Debug for OthelloBoard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		//let st = "eddie";
		//f.write_str("eddie");
		//f.write_str("christian");
		 
		 let mut sortedrow0: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 0)
	    .collect::<Vec<&(i32, i32)>>();
	   
        sortedrow0.sort_by(|&a, &b| a.1.cmp(&b.1));

        let sorted_values = sortedrow0.iter().map(|key| self.board.get(key));
        println!("sss {:?}\n",sorted_values);
         let mut sortedrow1: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 1)
	    .collect::<Vec<&(i32, i32)>>();
	   
        sortedrow1.sort_by(|&a, &b| a.1.cmp(&b.1));

		write!(f, "{:?}\n{:?}",sorted_values,sortedrow1)
	}
}



fn main() {

    let n = 4;
    let size: i32 = 2 * n;
    let board = OthelloBoard::new(size);
    println!("board=\n{:?}", board);
    let row1 = board.test(1);
    //println!("row1= {:?}", row1 as String);
}
