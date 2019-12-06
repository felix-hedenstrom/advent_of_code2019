use std::io::{self, Read};


fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer;
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
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
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
    fn get_target_address(&self) -> usize{
        return self.parameters[self.size - 2].value as usize;
    }

    fn new(counter: i64, opcodes: &Vec<i64>) -> Option<Instruction>{
        let raw_op = opcodes[counter as usize];

        let (op, op_size): (Operation, usize) = match raw_op % 100 {
            1 => (Operation::Addition, 4),
            2 => (Operation::Multiplication, 4),
            3 => (Operation::Store, 2),
            4 => (Operation::Output, 2),
            5 => (Operation::JumpIfTrue, 3),
            6 => (Operation::JumpIfFalse, 3),
            7 => (Operation::LessThan, 4),
            8 => (Operation::Equals, 4),
            _ => return None 
        };
       
        // We've read all possible instructions
        if !(counter as usize + op_size < opcodes.len()){
            println!("Finished");
            return None;
        }

        let mut op_as_string: Vec<i64> = 
            raw_op
            .to_string()
            .chars() 
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        op_as_string.reverse();
            
        let op_parameters: Vec<Parameter> = 
            (2..op_size + 1)
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

        return Some(Instruction {operation: op, parameters: op_parameters, size: op_size});
    }
}

fn address_counter(opcodes: &Vec<i64>, input: &i64) -> Vec<i64> {
    fn address_counter_internal(counter: i64, opcodes: Vec<i64>, input: &i64) -> Vec<i64>{
        let inst = 
            match Instruction::new(counter, &opcodes){
                Some(i) => i,
                None => return opcodes
            };

        let (new_counter, new_opcodes): (Option<i64>, Vec<i64>) = 
            execute_instruction(
                    &inst, 
                    opcodes,
                    input
            );
        return address_counter_internal(new_counter.unwrap_or(counter + inst.size as i64), new_opcodes, input); 
    }
    return address_counter_internal(0, opcodes.clone(), input);

}

fn execute_instruction(ins: &Instruction, opcodes: Vec<i64>, input: &i64) -> (Option<i64>, Vec<i64>){
    fn op_addition(params: Vec<i64>, mut opcodes: Vec<i64>, ins: &Instruction) -> Vec<i64>{

        opcodes[ins.get_target_address()] = 
            params[0]
            +
            params[1];        
        return opcodes;
    }
    
    fn op_multiplication(params: Vec<i64>, mut opcodes: Vec<i64>, ins: &Instruction) -> Vec<i64>{

        opcodes[ins.get_target_address()] = 
            params[0]
            *
            params[1];        
        return opcodes;
    }
    
    fn op_store(ins: &Instruction, mut opcodes: Vec<i64>, input: &i64) -> Vec<i64>{
        
        opcodes[ins.get_target_address()] = *input;

        return opcodes;

    }
    fn op_output(params: Vec<i64>, opcodes: Vec<i64>) -> Vec<i64>{

        println!("output {:?}", params[0]);
        return opcodes;
    }

    fn op_jumpiftrue(params: Vec<i64>, opcodes: Vec<i64>) -> (Option<i64>, Vec<i64>){

        if params[0] != 0{
            return (Some(params[1]), opcodes);
        }
        return (None, opcodes);
    }
    fn op_jumpiffalse(params: Vec<i64>, opcodes: Vec<i64>) -> (Option<i64>, Vec<i64>){
        if params[0] == 0{
            return (Some(params[1]), opcodes);
        }
        return (None, opcodes);
    }
    fn op_lessthan(params: Vec<i64>, mut opcodes: Vec<i64>, ins: &Instruction) -> Vec<i64>{

        if params[0] < params[1] {
            opcodes[ins.get_target_address()] = 1;
        }else{
            opcodes[ins.get_target_address()] = 0;
        }
        return opcodes;
    }
    fn op_equals(params: Vec<i64>, mut opcodes: Vec<i64>, ins: &Instruction) -> Vec<i64>{

        if params[0] == params[1] {
            opcodes[ins.get_target_address()] = 1;
        }else{
            opcodes[ins.get_target_address()] = 0;
        }
        return opcodes;
    }

    //println!("Instruction: {:?}", ins);

    let params: Vec<i64> =
        (&ins.parameters)
        .into_iter()
        .map(
            |p| 
            match p.mode {
                Mode::Position => opcodes[p.value as usize],
                Mode::Immediate => p.value
            }
        ).collect();

    //println!("Params: {:?}", params);

    return 
        match ins.operation{
            Operation::Addition => (None, op_addition(params, opcodes, ins)),
            Operation::Multiplication => (None, op_multiplication(params, opcodes, ins)),
            Operation::Store => (None, op_store(ins, opcodes, input)),
            Operation::Output => (None, op_output(params, opcodes)),
            Operation::JumpIfTrue => op_jumpiftrue(params, opcodes),
            Operation::JumpIfFalse => op_jumpiffalse(params, opcodes),
            Operation::LessThan => (None, op_lessthan(params, opcodes, ins)),
            Operation::Equals => (None, op_equals(params, opcodes, ins))
        };

}

fn main (){
    let io_input: Vec<i64> =
            read_stdin()
            .trim()
            .split(",")
            .map(|s| 
                    s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
            ).collect();
	
    let answer = address_counter(&io_input, &5); 
    //print!("answer: {:?}", answer); 				
}
