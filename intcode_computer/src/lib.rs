mod instruction;

use instruction::{Instruction, Operation};

#[derive(Debug, Clone)]
pub struct State{
    input: Vec<i64>,
    output: Option<i64>,
    address: i64,
    opcodes: Vec<i64>
}

pub fn address_counter(opcodes: &Vec<i64>, input: &Vec<i64>) -> State {

    return State::new(opcodes, input).process();
}

impl State {
    pub fn new(opcodes: &Vec<i64>, input: &Vec<i64>) -> State{
        let reversed_input: Vec<i64> = 
            input
            .iter()
            .rev()
            .map(|i|
                *i
            ).collect();
        return State{ output: None, input: reversed_input, opcodes: (*opcodes).clone(), address: 0}; 
    }

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
    
    fn get_opcodes(&self) -> &Vec<i64>{
        return &self.opcodes;
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
        fn op_addition(st: &State, ins: &Instruction) -> State {
            
            let mut opcodes = st.opcodes.clone();
            let params = ins.get_parameters(&st.get_opcodes());

            opcodes[ins.get_target_address()] = 
                params[0]
                +
                params[1];        

            return st.set_opcodes(opcodes).increment_address(ins.size() as i64);
        }
        
        fn op_multiplication(st: &State, ins: &Instruction) -> State {

            let mut opcodes = st.opcodes.clone();
            let params = ins.get_parameters(&st.get_opcodes());

            opcodes[ins.get_target_address()] = 
                params[0]
                *
                params[1];        

            return st.set_opcodes(opcodes).increment_address(ins.size() as i64);
        }
        
        fn op_store(st: &State, ins: &Instruction) -> State{
            
            let mut new_input = st.input.clone();

            let value = new_input.pop().expect("did not get enough inputs");

            let mut opcodes = st.opcodes.clone();
            opcodes[ins.get_target_address()] = value;

            return State{input: new_input, output: st.output, opcodes: opcodes, address: st.address}.increment_address(ins.size() as i64);

        }

        fn op_output(st: &State, ins: &Instruction) -> State{


            let params = ins.get_parameters(&st.get_opcodes());
            return State{input: st.input.clone(), output: Some(params[0]), opcodes: st.opcodes.clone(), address: st.address}.increment_address(ins.size() as i64);
        }

        fn op_jumpiftrue(st: &State, ins: &Instruction) -> State{

            let params = ins.get_parameters(&st.get_opcodes());
            let new_address = 
                match params[0] != 0{
                    true => params[1],
                    false => st.address + ins.size() as i64

                };
            return st.set_address(new_address);
        }
        fn op_jumpiffalse(st: &State, ins: &Instruction) -> State{
            let params = ins.get_parameters(&st.get_opcodes());
            let new_address = 
                match params[0] == 0{
                    true => params[1],
                    false => st.address + ins.size() as i64

                };
            return st.set_address(new_address);
        }
        fn op_lessthan(st: &State, ins: &Instruction) -> State{

            let params = ins.get_parameters(&st.get_opcodes());
            let mut opcodes = st.opcodes.clone();

            if params[0] < params[1] {
                opcodes[ins.get_target_address()] = 1;
            }else{
                opcodes[ins.get_target_address()] = 0;
            }
            return st.set_opcodes(opcodes).increment_address(ins.size() as i64);
        }
        fn op_equals(st: &State, ins: &Instruction) -> State{

            let params = ins.get_parameters(&st.get_opcodes());
            let mut opcodes = st.opcodes.clone();

            if params[0] == params[1] {
                opcodes[ins.get_target_address()] = 1;
            }else{
                opcodes[ins.get_target_address()] = 0;
            }
            return st.set_opcodes(opcodes).increment_address(ins.size() as i64);
        }

        let operation_function = 
            match ins.get_operation(){
                Operation::Addition => op_addition,
                Operation::Multiplication => op_multiplication,
                Operation::Store => op_store,
                Operation::Output => op_output,
                Operation::JumpIfTrue => op_jumpiftrue,
                Operation::JumpIfFalse => op_jumpiffalse,
                Operation::LessThan => op_lessthan,
                Operation::Equals => op_equals
            };
        
        return operation_function(self, ins);

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_p1_example() {
        
        assert_eq!(
            address_counter(&vec![1,0,0,0,99], &vec![]).opcodes, 
            vec![2,0,0,0,99]
        );
        assert_eq!(
            address_counter(&vec![2,3,0,3,99], &vec![]).opcodes, 
            vec![2,3,0,6,99]
        );
        assert_eq!(
            address_counter(&vec![2,4,4,5,99, 0], &vec![]).opcodes, 
            vec![2,4,4,5,99, 9801]
        );
        assert_eq!(
            address_counter(&vec![1,1,1,4,99, 5, 6, 0, 99], &vec![]).opcodes, 
            vec![30,1,1,4,2,5,6,0,99]
        );
    }
    #[test]
    fn day5_example() {
    
        assert_eq!(
            address_counter(&vec![1101, 100, -1, 4, 0], &vec![]).opcodes, 
            vec![1101, 100, -1, 4, 99]
        );
        assert_eq!(
            address_counter(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![8]).output.unwrap(), 
            1 
        );
        assert_eq!(
            address_counter(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![1]).output.unwrap(), 
            0 
        );
        
        let input: Vec<i64> = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];


        assert_eq!(
            address_counter(&input, &vec![7]).output.unwrap(), 
            999
        );
        assert_eq!(
            address_counter(&input, &vec![8]).output.unwrap(), 
            1000 
        );
        assert_eq!(
            address_counter(&input, &vec![9]).output.unwrap(), 
            1001
        );

    }
}
