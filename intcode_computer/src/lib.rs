mod instruction;
mod operation;

use instruction::Instruction;

#[derive(Debug, Clone)]
pub struct State{
    input: Vec<i64>,
    //output: Vec<i64>,
    output: Option<i64>,
    address: i64,
    opcodes: Vec<i64>,
    relative_base: i64
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

        return State{ output: None, input: reversed_input, opcodes: (*opcodes).clone(), address: 0, relative_base: 0}; 
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
    fn write(&self, address: usize, value: i64) -> State{
        let mut opcodes = self.get_opcodes().clone();
        
        if !(address < opcodes.len()){
            dbg!("resizing opcodes to size {}", address);
            opcodes.resize_with(address + 1, || 0); 
        }

        opcodes[address] = value;

        return self.set_opcodes(opcodes.to_vec());

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
        return State{
            input: self.input.clone(), 
            output: None, 
            opcodes: self.opcodes.clone(), 
            address: self.address,
            relative_base: self.relative_base
        };
    }

    pub fn add_input(&self, input: i64) -> State{
        let mut new_input: Vec<i64> = self.input.clone();
        
        new_input.insert(0, input);

        return State{
            input: new_input, 
            output: self.output, 
            opcodes: self.opcodes.clone(), 
            address: self.address,
            relative_base: self.relative_base
        };
    }
    fn increment_address(&self, amount: i64) -> State{
        let new_address = self.address + amount;
        return self.set_address(new_address);
    }
    fn set_address(&self, new_address: i64) -> State{
        return 
            State {
                input: self.input.clone(), 
                output: self.output, 
                opcodes: self.opcodes.clone(), 
                address: new_address,
                relative_base: self.relative_base
            };
    }
    fn set_opcodes(&self, new_opcodes: Vec<i64>) -> State{
        return State {
            input: self.input.clone(), 
            output: self.output, 
            opcodes: new_opcodes, 
            address: self.address,
            relative_base: self.relative_base
        };
    }
    pub fn update_relative_base(&self, diff: i64) -> State{
        let mut data = self.clone();

        data.relative_base += diff;

        return data;
    
    }
    fn execute_instruction(&self, ins: &Instruction)-> State {
        return ins.get_operation().process(self, ins);
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
    #[test]
    fn day9_relative_base(){
        /*
        let v: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        assert_eq!(
            address_counter(&v, &vec![]).output,
        );
        */
        assert_eq!(
            address_counter(&vec![1102,34915192,34915192,7,4,7,99,0], &vec![]).output.unwrap(),
            1219070632396864
        );

        assert_eq!(
            address_counter(&vec![104,1125899906842624,99], &vec![]).output.unwrap(),
            1125899906842624
        );
    }
}
