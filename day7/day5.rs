use std::io::{self, Read};

#[allow(dead_code)]
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
#[derive(Debug, Clone)]
pub struct State{
    input: Vec<i64>,
    output: Option<i64>,
    address: i64,
    opcodes: Vec<i64>
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
    
    fn new(counter: &i64, opcodes: &Vec<i64>) -> Option<Instruction>{
        let raw_op = opcodes[*counter as usize];

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
        if !(*counter as usize + op_size < opcodes.len()){
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
                        opcodes[*counter as usize + i as usize - 1]
                } 
            ).collect();

        return Some(Instruction {operation: op, parameters: op_parameters, size: op_size});
    }
}
pub fn address_counter(opcodes: &Vec<i64>, input: &Vec<i64>) -> State {

    let reversed_input: Vec<i64> = 
        input
        .iter()
        .rev()
        .map(|i|
            *i
        ).collect();

    return State{ output: None, input: reversed_input, opcodes: (*opcodes).clone(), address: 0 }.process();

}
impl State {
    pub fn process(&self) -> State {
        let inst = 
            match Instruction::new(&self.address, &self.opcodes){
                Some(i) => i,
                None => return (*self).set_address(self.opcodes.len() as i64 + 20).clone()
            };
        let result_state: State = 
            self.execute_instruction(
                    &inst
            );

        if result_state.output.is_some(){
            return result_state;
        }

        return result_state.process(); 
    }
    pub fn is_halted(&self) -> bool{
        return !(self.address < self.opcodes.len() as i64); 
    }
    #[allow(dead_code)]
    pub fn get_input(&self) -> &Vec<i64>{
        return &self.input;
    }
    pub fn get_output(&self) -> &Option<i64>{
        return &self.output;
    }
    pub fn clean_output(&self) -> State{
        return State{input: self.input.clone(), output: None, opcodes: self.opcodes.clone(), address: self.address};
    }
    // Todo maybe keep old input?
    pub fn add_input(&self, input: i64) -> State{
        let mut new_input: Vec<i64> = self.input.clone();
        
        new_input.insert(0, input);

        return State{input: new_input, output: self.output, opcodes: self.opcodes.clone(), address: self.address};
    }
    fn increment_address(&self, amount: i64) -> State{
        let new_address = self.address + amount;
        return self.set_address(new_address);
    }
    fn set_address(&self, new_address: i64) -> State{
        return State {input: self.input.clone(), output: self.output, opcodes: self.opcodes.clone(), address: new_address};
    }
    fn set_opcodes(&self, new_opcodes: Vec<i64>) -> State{
        return State {input: self.input.clone(), output: self.output, opcodes: new_opcodes, address: self.address};
    }
    fn execute_instruction(&self, ins: &Instruction)-> State {
        fn op_addition(params: Vec<i64>, st: &State, ins: &Instruction) -> State {
            
            let mut opcodes = st.opcodes.clone();

            opcodes[ins.get_target_address()] = 
                params[0]
                +
                params[1];        

            return st.set_opcodes(opcodes).increment_address(ins.size as i64);
        }
        
        fn op_multiplication(params: Vec<i64>, st: &State, ins: &Instruction) -> State {

            let mut opcodes = st.opcodes.clone();

            opcodes[ins.get_target_address()] = 
                params[0]
                *
                params[1];        

            return st.set_opcodes(opcodes).increment_address(ins.size as i64);
        }
        
        fn op_store(ins: &Instruction, st: &State) -> State{
            
            let mut new_input= st.input.clone();

            let value = new_input.pop().expect("did not get enough inputs");

            let mut opcodes = st.opcodes.clone();
            opcodes[ins.get_target_address()] = value;

            return State{input: new_input, output: st.output, opcodes: opcodes, address: st.address}.increment_address(ins.size as i64);

        }

        fn op_output(params: Vec<i64>, st: &State, ins: &Instruction) -> State{

            return State{input: st.input.clone(), output: Some(params[0]), opcodes: st.opcodes.clone(), address: st.address}.increment_address(ins.size as i64);
        }

        fn op_jumpiftrue(params: Vec<i64>, st: &State, ins: &Instruction) -> State{

            let new_address = 
                match params[0] != 0{
                    true => params[1],
                    false => st.address + ins.size as i64

                };
            return st.set_address(new_address);
        }
        fn op_jumpiffalse(params: Vec<i64>, st: &State, ins: &Instruction) -> State{
            let new_address = 
                match params[0] == 0{
                    true => params[1],
                    false => st.address + ins.size as i64

                };
            return st.set_address(new_address);
        }
        fn op_lessthan(params: Vec<i64>, st: &State, ins: &Instruction) -> State{

            let mut opcodes = st.opcodes.clone();

            if params[0] < params[1] {
                opcodes[ins.get_target_address()] = 1;
            }else{
                opcodes[ins.get_target_address()] = 0;
            }
            return st.set_opcodes(opcodes).increment_address(ins.size as i64);
        }
        fn op_equals(params: Vec<i64>, st: &State, ins: &Instruction) -> State{

            let mut opcodes = st.opcodes.clone();

            if params[0] == params[1] {
                opcodes[ins.get_target_address()] = 1;
            }else{
                opcodes[ins.get_target_address()] = 0;
            }
            return st.set_opcodes(opcodes).increment_address(ins.size as i64);
        }

        //println!("Instruction: {:?}", ins);

        let params: Vec<i64> =
            (&ins.parameters)
            .into_iter()
            .map(
                |p| 
                match p.mode {
                    Mode::Position => self.opcodes[p.value as usize],
                    Mode::Immediate => p.value
                }
            ).collect();

        //println!("Params: {:?}", params);

        return 
            match ins.operation{
                Operation::Addition => op_addition(params, self, ins),
                Operation::Multiplication => op_multiplication(params, self, ins),
                Operation::Store => op_store(ins, self),
                Operation::Output => op_output(params, self, ins),
                Operation::JumpIfTrue => op_jumpiftrue(params, self, ins),
                Operation::JumpIfFalse => op_jumpiffalse(params, self, ins),
                Operation::LessThan => op_lessthan(params, self, ins),
                Operation::Equals => op_equals(params, self, ins)
            };

    }
}

#[allow(dead_code)]
fn main (){
    let io_input: Vec<i64> =
            read_stdin()
            .trim()
            .split(",")
            .map(|s| 
                    s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
            ).collect();
	
    let answer = address_counter(&io_input, &vec![5]); 
    println!("opcodes: {:?}", answer.opcodes);
    print!("answer: {:?}", answer.output); 				
}
