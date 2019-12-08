use std::io::{self, Read};

use permutohedron::heap_recursive;

extern crate intcode_computer;

use intcode_computer::{address_counter, State};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
}

fn max_signal(opcodes: &Vec<i64>, setting_sequence: &Vec<i64>, input_signal: &i64) -> i64{
    
    let mut signal = *input_signal;

    for i in 0..(setting_sequence.len()){
        let result: State = address_counter(&opcodes, &vec![setting_sequence[i], signal]);

        signal = result.get_output().expect("Did not get an output when one was expected");
    }

    return signal;
}

fn find_max_signal(opcodes: &Vec<i64>, base_seq: Vec<i64>) -> i64{

    let mut best_signal: i64 = 0;
   
    
    let mut data = base_seq.clone();
    let mut permutations = Vec::new();

    heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
    });

    for seq in &permutations{
        let signal_strength = max_signal(&opcodes, &seq, &0); 
        if signal_strength > best_signal{
            best_signal = signal_strength;
        }
    }

    return best_signal;


}

fn max_signal_p2(opcodes: &Vec<i64>, setting_sequence: &Vec<i64>, input_signal: &i64) -> i64{
    
    let mut signal = *input_signal;

    let mut latest_end_signal = 0;

    let mut states: Vec<Option<State>> = vec![None, None, None, None, None];

    loop {
        for i in 0..(setting_sequence.len()){
            if states[i].as_ref().map_or(false, |s| s.is_halted()){
                return latest_end_signal;
            }
            let result: State = 
                match &states[i]{
                    None => {
                        address_counter(&opcodes, &vec![setting_sequence[i], signal])
                    },
                    Some(s) => {
                        //println!("Input: {:?}", s.get_input()); 
                        s.add_input(signal).process() 
                    }
                };
                
            
            signal = 
                match result.get_output(){
                    Some(o) => *o,
                    None => return latest_end_signal
                };

            states[i] = Some(result.clean_output());

        }
        latest_end_signal = signal;
    }
}

fn find_max_signal_p2(opcodes: &Vec<i64>, base_seq: Vec<i64>) -> i64{

    let mut best_signal: i64 = 0;
   
    
    let mut data = base_seq.clone();
    let mut permutations = Vec::new();

    heap_recursive(&mut data, |permutation| {
            permutations.push(permutation.to_vec())
    });

    for seq in &permutations{
        let signal_strength = max_signal_p2(&opcodes, &seq, &0); 
        if signal_strength > best_signal{
            best_signal = signal_strength;
        }
    }

    return best_signal;


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
    println!("Max signal: {:?}", find_max_signal(&io_input, (0..5).collect()));
    //println!("test: {:?}", max_signal_p2(&io_input, &vec![9,8,7,6,5], &0)); 

    println!("Max signal: {:?}", find_max_signal_p2(&io_input, (5..10).collect()));
}
