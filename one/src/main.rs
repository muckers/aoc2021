
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn triplet_sum( i : usize, data : &Vec<u32>) -> u32 { 

    data[i] + data[i+1] + data[i+2]
}

fn part_a( data : &Vec<u32> ) {

    let mut prev : u32 = data[0];
    let mut count : u32 = 0;
    data.iter()
    .skip(1)
    .for_each( |n| {
        if *n > prev {
            count = count + 1;
        }  
        prev = *n;
    });

    println!("Count: {}", count );
}

fn part_b( data : &Vec<u32> ) {

    let mut count = 0;

    for i in 0..data.len()-1 {
        
        if i + 2 < data.len()-1 {
    
            let sum1 = triplet_sum( i, &data );
            let sum2 = triplet_sum( i+1, &data);

            if sum2 > sum1 {
                count = count + 1;
            }
        }
        else {
            break;
        }
    }

    println!("count: {}", count );
}

fn main() {

    let mut data : Vec<u32> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    data.push( l.parse().unwrap() );
                }
            }
        }
    }

    part_a( &data );
    part_b( &data );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
