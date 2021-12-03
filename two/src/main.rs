
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_a( data : &Vec<String> ) -> u32 {

    let mut hz : u32 = 0;
    let mut vt : u32 = 0;

    data.iter()
    .for_each( |cstr| {

        let (c,nstr) = cstr.split_at(cstr.find(' ').unwrap());
        let n : u32 = nstr.trim().parse().unwrap();
        match c {

            "forward" => hz = hz + n,
            "down" => vt = vt + n,
            "up" => vt = vt - n,
            _ => ()
            
        }
    });
    
    hz * vt
}

fn part_b( data : &Vec<String> ) -> u32 {

    let mut hz : u32 = 0;
    let mut aim : u32 = 0;
    let mut depth : u32 = 0;

    data.iter()
    .for_each( |cstr| {

        let (c,nstr) = cstr.split_at(cstr.find(' ').unwrap());
        let n : u32 = nstr.trim().parse().unwrap();
        match c {

            "forward" => { hz = hz + n; depth = depth + (n * aim) },
            "down" => aim = aim + n,
            "up" => aim = aim - n,
            _ => ()
            
        }
    });
    
    hz * depth
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

    println!("part_a: {}", part_a( &data ) );
    println!("part_b: {}", part_b( &data ) );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
