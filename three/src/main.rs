
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WORD_LEN : usize = 12;

fn part_a( data : &Vec<String> ) -> u32 {

    let mut one_counts : Vec<u32> = vec!(0;WORD_LEN);
    let dlen = data.len() as u32;
    let mut gamma_str : String = String::new();
    let mut epsilon_str : String = String::new();

    for i in 0..WORD_LEN {

        data.iter().for_each( |s| {

            let b : char = s.chars().nth( i ).unwrap();
            if b == '1' {
                one_counts[i] += 1;
            }
        });

        if one_counts[i] >= dlen/2 {

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

fn get_dominant_bit( data : &Vec<String>, bit_pos : usize, cmp: fn(c:usize,l:usize) -> char ) -> char {

    // Get all the bits from the specified position
    //
    let bits : Vec<char> = data
    .iter()
    .map( |s| s.chars().nth( bit_pos ).unwrap())
    .collect();

    //  Now only the 1 bits
    //
    let ones : Vec<char> = bits
    .clone()
    .into_iter()
    .filter( |d| *d == '1' )
    .collect();

    // Return the bit that is most or least common depending on the
    // comparison function passed in
    //
    cmp( ones.len()*2, data.len() )

}


fn part_b( data : &Vec<String> ) -> u32 {

    let oxy_rating = match_bits( data, 0, | c, l | { if c >= l {'1'} else {'0'} } );    
    let co2_rating = match_bits( data, 0, | c, l | { if c >= l {'0'} else {'1'} } );

    let oxy = u32::from_str_radix( oxy_rating.first().unwrap(), 2 ).unwrap();
    let co2 = u32::from_str_radix( co2_rating.first().unwrap(), 2 ).unwrap();

    oxy * co2
}


fn match_bits( data : &Vec<String>,  bit_pos : usize, cmp: fn(c:usize,l:usize) -> char ) -> Vec<String> {

    if bit_pos >= WORD_LEN || data.len() == 1 {
        return data.to_vec();
    }

    let mut temp : Vec<String> = Vec::new();
    let dom_bit = get_dominant_bit( data, bit_pos, cmp);

    for d in data {

        if d.chars().nth( bit_pos ).unwrap() == dom_bit {
            temp.push( d.to_string() );
        }
    }
 
    return match_bits( &temp, bit_pos+1, cmp );

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

    println!( "part a: {}", part_a( &data ) );
    println!( "part b: {}", part_b( &data ) );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
