
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn part_a( data : &Vec<String> ) -> u32 {

    let mut one_counts : Vec<u32> = vec!(0;12);
    let dlen = data.len() as u32;
    let mut gamma_str : String = String::new();
    let mut epsilon_str : String = String::new();

    for i in 0..12 {

        data.iter().for_each( |s| {

            let b : char = s.chars().nth( i ).unwrap();
            if b == '1' {
                one_counts[i] += 1;
            }

        });

        if one_counts[i] > dlen/2 {

            gamma_str.push('1');
            epsilon_str.push( '0');
        }
        else {
            gamma_str.push( '0');
            epsilon_str.push( '1');
        }
    }

    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = u32::from_str_radix( &epsilon_str, 2).unwrap();

    gamma * epsilon
}

fn main() {

    let mut data : Vec<String> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    data.push( l );
                }
            }
        }
    }

    println!( "{}", part_a( &data ) );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
