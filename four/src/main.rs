
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BOARD_H : usize = 5;
const BOARD_W : usize = 5;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct BingoBoard {

    board : Vec<Vec<u32>>,
    _board_matches : Vec<Vec<bool>>,
    _winner : bool

}

impl BingoBoard {

    fn has_match( self : &Self, n : u32 ) -> Option<(usize,usize)> {

        for i in 0..BOARD_H {
            for j in 0..BOARD_W {
                if n == self.board[i][j] {
                    return Some( (i,j) );
                }
            }
        }

        None
    }

    fn is_winner( self : &Self ) -> bool {

        for i in 0..BOARD_H {
            for j in 0..BOARD_W {
                if self._board_matches[i][0] &&
                    self._board_matches[i][1] &&
                    self._board_matches[i][2] &&
                    self._board_matches[i][3] &&
                    self._board_matches[i][4] {
                        return true;
                    }
            }
        }

        false

    }
}

fn process_calls( calls : &[u32], boards : &mut Vec<BingoBoard> ) -> Option<BingoBoard> {

    let mut idx : usize = 0;
    let mut found : Option<BingoBoard> = None;

    for n in calls {

        boards.iter_mut().for_each( |b :&mut BingoBoard| {

            if found == None {

                match b.has_match(*n) {
                    
                    Some((i,j)) => {
                        b._board_matches[i][j] = true;
                        if b.is_winner() {
                            found = Some(b.clone());
                        }
                    },
                    None => ()
                }
                idx += 1;
            }
        });

        if found == None { break; }
    }

    return found;
}

fn main() {

    let mut data : Vec<String> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input1.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok( l ) = line {
                data.push( l );
            }
        }
    }

    let calls : Vec<u32> = data[0].split(",").map(|ns| ns.parse().unwrap()).collect();
    let mut boards : Vec<BingoBoard> = Vec::new();
    let mut temp_board  = Vec::new();

    for line in data.iter().skip(2) {

        if line.is_empty() {

            boards.push(

                BingoBoard { board: temp_board.clone(), _board_matches: vec![vec![false;BOARD_H]; BOARD_W], _winner: false}
            );

            temp_board.clear();
            continue;
        }
        else {
            temp_board.push( line.split_whitespace().map(|ns| ns.parse().unwrap()).collect() );

        }
    }

    boards.push(

        BingoBoard { board: temp_board.clone(), _board_matches: vec![vec![false;BOARD_H]; BOARD_W], _winner: false}
    );

    // println!("{}", boards[0].board.len() );
    let winning_board= process_calls( &calls, &mut boards );
    println!( "{:?}", winning_board );
    // for b in boards {
    //     println!("{:?}", b );
    // }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
