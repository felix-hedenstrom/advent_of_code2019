use std::io::{self, Read};

use std::collections::HashMap;

extern crate intcode_computer;

use intcode_computer::State;

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

    let mut s: State = State::new(&io_input, &vec![]);
    s.process();

    let output = s.get_output(); 

    let answer = 
        (2..(s.get_output().len()))
        .step_by(3)
        .filter(|i|
            output[*i] == 2
        ).count();


    println!("{:?}", answer);
}

        
