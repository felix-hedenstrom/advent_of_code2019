use std::io::{self, Read};

extern crate intcode_computer;

use intcode_computer::{address_counter, State};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}

fn main (){
    let io_input: Vec<i64> =
            read_stdin()
            .trim()
            .split(",")
            .map(|s| 
                    s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
            ).collect();


    println!("Started!");

    let s: State = address_counter(&io_input, &vec![2]);

    println!("{:?}", s.get_output());
}
