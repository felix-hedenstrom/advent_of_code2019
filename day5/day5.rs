use std::io::{self, Read};


fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer.clone()
}

#[derive(Debug)]
enum Mode{
    Position,
    Immediate    
}

#[derive(Debug)]
enum Operation{
    Addition,
    Multiplication,
    Store,
    Output
}

#[derive(Debug)]
struct Parameter{
    mode: Mode,
    value: i64 
}
#[derive(Debug)]
struct Instruction{
    operation: Operation,
    parameters: Vec<Parameter>,
    size: usize
}

impl Instruction{
    fn new(counter: i64, opcodes: &Vec<i64>) -> Option<Instruction>{
        let raw_op = opcodes[counter as usize];

        let (op, op_size): (Operation, usize) = match raw_op % 10 {
            1 => (Operation::Addition, 4),
            2 => (Operation::Multiplication, 4),
            3 => (Operation::Store, 2),
            4 => (Operation::Output, 2),
            n => panic!("recieved bad operation: {}", n)
        };
       
        // We've read all possible instructions
        if !(counter as usize + op_size < opcodes.len()){
            return None;
        }

        let mut op_as_string: Vec<i64> = 
            raw_op
            .to_string()
            .split("")
            .map(|c| c.parse::<i64>().unwrap())
            .collect();

        op_as_string.reverse();
            


        let parameters: Vec<Parameter> = 
            (2..op_size)
            .map(|i| 
                Parameter{ 
                    mode: 
                        match op_as_string.get(i as usize).unwrap_or(&0){
                            1 => Mode::Immediate,
                            0 => Mode::Position,
                            _ => panic!("found a number that should not exist")
                        },
                    value:
                        opcodes[counter as usize + i as usize - 1]
                } 
            ).collect();

        //TODO
        return Some(Instruction {operation: op, parameters: vec![], size: op_size});
    }
}

fn address_counter(opcodes: &Vec<i64>) -> Vec<i64> {
    fn address_counter_internal(counter: i64, opcodes: Vec<i64>) -> Vec<i64>{
        let inst = Instruction::new(counter, &opcodes).unwrap_or({return opcodes});

        let new_opcodes: Vec<i64> = 
                execute_instruction(
                    &inst, 
                    opcodes
            );
        print!("{:?}", inst);
        return address_counter_internal(counter + inst.size as i64, new_opcodes); 
    }
    return address_counter_internal(0, opcodes.clone());

}

fn execute_instruction(ins: &Instruction, opcodes: Vec<i64>) -> Vec<i64>{
    return opcodes;
}

fn main (){
	let input: Vec<i64> =
		read_stdin()
		.trim()
		.split(",")
		.map(|s| 
			s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
		).collect();
	
    let answer = address_counter(&input); 
    print!("answer: {:?}", answer); 				
}
