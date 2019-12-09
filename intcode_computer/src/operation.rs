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
    Equals,
    AdjustRelativeBase
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
                Operation::Equals => op_equals,
                Operation::AdjustRelativeBase => op_adjust_relative_base
            };
        
        return operation_function(st, ins);
    }
}

fn op_adjust_relative_base(st: &State, ins: &Instruction) -> State {
    let params = ins.get_parameters(st);
    return st.update_relative_base(params[0]);
}

fn op_addition(st: &State, ins: &Instruction) -> State {
    
    let params = ins.get_parameters(st);

    return st.write(
        ins.get_target_address(),
        params[0] + params[1]
    ).increment_address(ins.size() as i64);
}

fn op_multiplication(st: &State, ins: &Instruction) -> State {

    let params = ins.get_parameters(st);

    return st.write(
        ins.get_target_address(),
        params[0] * params[1]
    ).increment_address(ins.size() as i64);
}

fn op_store(st: &State, ins: &Instruction) -> State{
    
    let mut new_input = st.input.clone();

    let value = new_input.pop().expect("did not get enough inputs");

    return State{
        input: new_input, 
        output: st.output, 
        opcodes: st.opcodes.clone(), 
        address: st.address, 
        relative_base: 
        st.relative_base
    }.write(
        ins.get_target_address(),
        value
    ).increment_address(ins.size() as i64);

}

fn op_output(st: &State, ins: &Instruction) -> State{


    let params = ins.get_parameters(st);
    return State{
        input: st.input.clone(), 
        output: Some(params[0]), 
        opcodes: st.opcodes.clone(), 
        address: st.address, 
        relative_base: st.relative_base
    }.increment_address(ins.size() as i64);
}

fn op_jumpiftrue(st: &State, ins: &Instruction) -> State{

    let params = ins.get_parameters(st);
    let new_address = 
        match params[0] != 0{
            true => params[1],
            false => st.address + ins.size() as i64

        };
    return st.set_address(new_address);
}
fn op_jumpiffalse(st: &State, ins: &Instruction) -> State{
    let params = ins.get_parameters(st);
    let new_address = 
        match params[0] == 0{
            true => params[1],
            false => st.address + ins.size() as i64

        };
    return st.set_address(new_address);
}
fn op_lessthan(st: &State, ins: &Instruction) -> State{

    let params = ins.get_parameters(st);
    let answer: i64 = 
        match params[0] < params[1] {
            true => 1,
            false => 0

        };
    return st.write(ins.get_target_address(), answer).increment_address(ins.size() as i64);
}

fn op_equals(st: &State, ins: &Instruction) -> State{

    let params = ins.get_parameters(st);
    let answer: i64 = 
        match params[0] == params[1] {
            true => 1,
            false => 0

        };

    return st.write(ins.get_target_address(), answer).increment_address(ins.size() as i64);
}
