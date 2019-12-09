use crate::operation::Operation;
use crate::State;

#[derive(Debug, Clone)]
enum Mode{
    Position,
    Immediate,
    Relative
}

#[derive(Debug, Clone)]
struct Parameter{
    mode: Mode,
    value: i64 
}

#[derive(Debug)]
pub struct Instruction{
    operation: Operation,
    parameters: Vec<Parameter>,
    size: usize
}

impl Instruction{
    pub fn get_target_address(&self, relative_base: i64) -> usize{
        let parameter = self.parameters[self.size - 2].clone(); 
        
       
            
        return match parameter.mode{
            Mode::Position => parameter.value,
            Mode::Immediate => parameter.value,
            Mode::Relative => parameter.value + relative_base
        } as usize;

    }
    pub fn size(&self) -> usize{
        return self.size;
    }
    pub fn get_parameters(&self, st: &State) -> Vec<i64>{
        return  
            (self.parameters.clone())
            .into_iter()
            .map(
                |p| 
                match p.mode {
                    Mode::Position => *(st.opcodes.get(p.value as usize).unwrap_or(&0)),
                    Mode::Immediate => p.value,
                    Mode::Relative => *(st.opcodes.get( (st.relative_base + p.value) as usize).unwrap_or(&0))
                }
            ).collect();
    }

    pub fn get_operation(&self) -> &Operation{
        return &self.operation;
    }
    
    pub fn new(counter: &i64, opcodes: &Vec<i64>) -> Option<Instruction>{
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
            9 => (Operation::AdjustRelativeBase, 2),
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
                            0 => Mode::Position,
                            1 => Mode::Immediate,
                            2 => Mode::Relative,
                            _ => panic!("found a number that should not exist")
                        },
                    value:
                        opcodes[*counter as usize + i as usize - 1]
                } 
            ).collect();

        return Some(Instruction {operation: op, parameters: op_parameters, size: op_size});
    }
}
