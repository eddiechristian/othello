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
    fn test(&self, row: i32) -> Vec<&BoardSquareState> {

	    let mut sortedrow: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == row)
	    .collect::<Vec<&(i32, i32)>>();
	   
	   let x = Box(BoardSquareState::EMPTY);
        
        sortedrow.sort_by(|&a, &b| a.1.cmp(&b.1));

        let sorted_values = sortedrow.iter().map(|key| {
        		match self.board.get(key){
        			Some(m) => m,
        			_ => &x
        		}
        	}
        	)
        .collect::<Vec<_>>();

        sorted_values
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
		 
		 //row0
		 let mut sortedrow0: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 0)
	    .collect::<Vec<&(i32, i32)>>();
	   
        sortedrow0.sort_by(|&a, &b| a.1.cmp(&b.1));
 		let x = &BoardSquareState::EMPTY;

        let sorted_values0 = sortedrow0.iter().map(|key| {
        		match self.board.get(key){
        			Some(m) => m,
        			_ => x
        		}
        	}
        	)
        .collect::<Vec<_>>();
        
        //row1
         let mut sortedrow1: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 1)
	    .collect::<Vec<&(i32, i32)>>();
	   
        sortedrow1.sort_by(|&a, &b| a.1.cmp(&b.1));
       let sorted_values1 = sortedrow1.iter().map(|key| {
        		match self.board.get(key){
        			Some(m) => m,
        			_ => x
        		}
        	}
        	)
        .collect::<Vec<_>>();

  		//row2
         let mut sortedrow2: Vec<&(i32, i32)> = self.board.keys()
	    .filter(|&key| key.0 == 2)
	    .collect::<Vec<&(i32, i32)>>();
	   
        sortedrow2.sort_by(|&a, &b| a.1.cmp(&b.1));
       let sorted_values2 = sortedrow2.iter().map(|key| {
        		match self.board.get(key){
        			Some(m) => m,
        			_ => x
        		}
        	}
        	)
        .collect::<Vec<_>>();

		write!(f, "{:?}\n{:?}\n{:?}",sorted_values0,sorted_values1,sorted_values2)
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
