mod instruction;
mod operation;

use instruction::Instruction;

#[derive(Debug, Clone)]
pub struct State{
    input: Vec<i64>,
    output: Vec<i64>,
    address: i64,
    opcodes: Vec<i64>,
    relative_base: i64
}

pub fn address_counter(opcodes: &Vec<i64>, input: &Vec<i64>) -> State {
    let mut s = State::new(opcodes, input);

    s.process();
    return s; 
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

        return State{ 
            output: vec![], 
            input: reversed_input, 
            opcodes: (*opcodes).clone(), 
            address: 0, 
            relative_base: 0
        }; 
    }

    pub fn process(&mut self) {
        let inst = 
            match Instruction::new(&self.address, &self.opcodes){
                Some(i) => i,
                None => return
            };

        self.execute_instruction(&inst);

        self.process(); 
    }
    
    fn write(&mut self, address: usize, value: i64){
        
        if !(address < self.opcodes.len()){
            self.opcodes.resize_with(address + 1, || 0); 
        }

        self.opcodes[address] = value;

    }
    pub fn is_halted(&self) -> bool{
        return !(self.address < self.opcodes.len() as i64); 
    }

    #[allow(dead_code)]
    pub fn get_input(&self) -> &Vec<i64>{
        return &self.input;
    }

    pub fn get_output(&self) -> &Vec<i64>{
        return &self.output;
    }
    pub fn clean_output(&mut self){
        self.output = vec![];
    }

    pub fn add_input(&mut self, input: i64){
        
        self.input.insert(0, input);

    }
    fn increment_address(&mut self, amount: i64){
        self.set_address(self.address + amount); 
    }
    fn set_address(&mut self, new_address: i64){
        
        self.address = new_address; 

    }
    pub fn update_relative_base(&mut self, diff: i64){

        self.relative_base += diff;

    
    }
    fn execute_instruction(&mut self, ins: &Instruction){

        ins.get_operation().process(self, ins);
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
            address_counter(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![8]).output, 
            vec![1] 
        );
        assert_eq!(
            address_counter(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![1]).output, 
            vec![0]
        );
        
        let input: Vec<i64> = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];


        assert_eq!(
            address_counter(&input, &vec![7]).output, 
            vec![999]
        );
        assert_eq!(
            address_counter(&input, &vec![8]).output, 
            vec![1000]
        );
        assert_eq!(
            address_counter(&input, &vec![9]).output, 
            vec![1001]
        );

    }
    #[test]
    fn day9_relative_base(){
        
        let v: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        assert_eq!(
            address_counter(&v, &vec![]).output,
            v
        );
        
        assert_eq!(
            address_counter(&vec![1102,34915192,34915192,7,4,7,99,0], &vec![]).output,
            vec![1219070632396864]
        );

        assert_eq!(
            address_counter(&vec![104,1125899906842624,99], &vec![]).output,
            vec![1125899906842624]
        );
    }
}
