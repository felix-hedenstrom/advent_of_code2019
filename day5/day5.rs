use std::io::{self, Read};


fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer.clone()
}


fn execute_opcodes(instruction_pointer: usize, opcodes: Vec<usize>) -> Result<Vec<usize>, ()> {
        fn code_3(instruction_pointer: usize, mut opcodes: Vec<usize>){
            let input = opcodes[instruction_pointer];

            opcodes[input] = input;
        }
        fn code_4(instruction_pointer: usize, mut opcodes: Vec<usize>){
            let input = opcodes[instruction_pointer];

            opcodes[input] = input;
        }
	if instruction_pointer + 3 > opcodes.len() - 1{
		return Ok(opcodes);
	} 

	let mut new_opcodes = opcodes.clone();
	
	let operation = opcodes[instruction_pointer];
	let data1_address = opcodes[instruction_pointer + 1];
	let data2_address = opcodes[instruction_pointer + 2];
	let write_address = opcodes[instruction_pointer + 3];


	if write_address > opcodes.len() - 1{
		return Err(());
	} 
	if data1_address > opcodes.len() - 1 || data2_address > opcodes.len() - 1{
		return Err(());
	}

	let data1 = opcodes[data1_address];
	let data2 = opcodes[data2_address];


	match operation {
		1 => new_opcodes[write_address] = data1 + data2,
		2 => new_opcodes[write_address] = data1 * data2,
                3 => code_3(instruction_pointer, new_opcodes), 
                4 => code_4(instruction_pointer, new_opcodes), 
		_ => () 
	};
	return execute_opcodes(instruction_pointer + 4, new_opcodes);
	
}

fn main (){
	let input: Vec<usize> =
		read_stdin()
		.trim()
		.split(",")
		.map(|s| 
			s.parse::<usize>().expect("of of the lines of the input could not be parsed into an integer") 
		).collect();
	
	for i in 0..99{
		for j in i..99{
			let mut update_indexes = input.clone();	
			
			update_indexes[1] = i;
			update_indexes[2] = j;
			let answer = execute_opcodes(0, update_indexes.clone()).unwrap_or(vec![0]); 
			//print!("{:?}\n", answer);
				
			if answer[0] == 19690720{
				print!("noun: {}, verb: {}, answer: {}", i, j, i * 100 + j);
			}
		}
	} 	
}
