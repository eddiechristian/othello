#![allow(unused_variables)]

#![allow(dead_code)]

use std::fmt;
use std::collections::HashMap;



#[derive(Clone, Debug)]
enum BoardSquareState {
	EMPTY,
	BLACK,
	WHITE
}



#[derive(Clone, Debug)]
struct OthelloBoard{
	board: HashMap<(i32,i32),BoardSquareState>
}

impl OthelloBoard {
	fn test(&self,row: i32) -> Vec<(i32,i32)>{
		  let mut ret_vec = Vec::new();
		  let keys = self.board.keys();
		  //doesnt compile???? println!("keys ={:?}\n" ,keys);
		  for key in keys{
		 	match *key{
		 		(x,y) if x ==row => ret_vec.push(key.clone()),//println!("row {:?} ({:?},{:?})\n",row,x,y),
		 		_ =>{}
		 	}
		  }
		  /* example    http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
		  let mut sorted_row = {
		  	ret_vec.as_slice()
		  	.iter_mut()
		  	.map(|the_key| {
		  			the_key.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
		  			the_key
		  	})
		  	.collect::<Vec<_>>();
		  };
		  */
		  //how can I sort them 
		  ret_vec
	}
    pub fn new(size: i32) -> OthelloBoard {
    	
       let mut map = HashMap::new();
        let top_left_col = (size-2)/3 + 1;
	 for x in 0..size {	
        	for y in 0..size {
        		let tup = (x,y);
        		match tup{
        			(k,l) if ((k == top_left_col) && (l==top_left_col)) ||
        					 ((k == top_left_col+1) && (l==top_left_col+1))  => {
        					 	let gg = (x,y);
        					 	map.insert(gg,BoardSquareState::WHITE);
        					 },
        			(k,l) if ((k == top_left_col+1) && (l==top_left_col)) ||
        					 ((k == top_left_col) && (l==top_left_col+1))  => {
        					 	let gg = (x,y);
        					 	map.insert(gg,BoardSquareState::BLACK);
        					 }
        			(k,l) => {
        				let gg = (x,y);
        					 	map.insert(gg,BoardSquareState::EMPTY);
        			}
        		}
        	}
        }
       OthelloBoard{board: map}
    }
}
/*
impl fmt::Debug for OthelloBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let st = "eddie";
        write!(f, "{:?}",st)
    }
}
*/


fn main() {
	
	let n = 4;
	let size :i32 = 2 * n;
	let board = OthelloBoard::new(size);
	println!("board= {:?}",board);
	let row1 = board.test(1);
	println!("row1= {:?}",row1);
}
