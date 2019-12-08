use crate::State;
use crate::Instruction;

#[derive(Debug)]
pub enum Operation{
    Addition,
    Multiplication,
    Store,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}

impl Operation{
    pub fn process(&self, st: &State, ins: &Instruction) -> State{
        let operation_function = 
            match self{
                Operation::Addition => op_addition,
                Operation::Multiplication => op_multiplication,
                Operation::Store => op_store,
                Operation::Output => op_output,
                Operation::JumpIfTrue => op_jumpiftrue,
                Operation::JumpIfFalse => op_jumpiffalse,
                Operation::LessThan => op_lessthan,
                Operation::Equals => op_equals
            };
        
        return operation_function(st, ins);
    }
}


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
