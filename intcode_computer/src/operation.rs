use crate::State;
use crate::{Instruction, HaltState};

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
    pub fn process(&self, st: &mut State, ins: &Instruction) -> Option<HaltState> {
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
        
        //dbg!(ins);
        return operation_function(st, ins);
    }
}

fn op_adjust_relative_base(st: &mut State, ins: &Instruction) -> Option<HaltState>{
    let params = ins.get_parameters(st);

    st.update_relative_base(params[0]);

    st.increment_address(ins.size() as i64);
    return None;
}

fn op_addition(st: &mut State, ins: &Instruction) -> Option<HaltState> {
    
    let params = ins.get_parameters(st);

    st.write(
        ins.get_target_address(st.relative_base),
        params[0] + params[1]
    );
    st.increment_address(ins.size() as i64);
    return None;

}

fn op_multiplication(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let params = ins.get_parameters(st);

    st.write(
        ins.get_target_address(st.relative_base),
        params[0] * params[1]
    );

    st.increment_address(ins.size() as i64);
    return None;
}

fn op_store(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let value =  match st.input.pop(){
            Some(v) => v,
            None => return Some(HaltState::WaitingForInput)
    };

    st.write(
        ins.get_target_address(st.relative_base),
        value
    );
    st.increment_address(ins.size() as i64);
    return None;

}

fn op_output(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let params = ins.get_parameters(st);

    st.output.push(params[0]);

    st.increment_address(ins.size() as i64);
    return None;

}

fn op_jumpiftrue(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let params = ins.get_parameters(st);
    let new_address = 
        match params[0] != 0{
            true => params[1],
            false => st.address + ins.size() as i64

        };
    st.set_address(new_address);
    return None;
}
fn op_jumpiffalse(st: &mut State, ins: &Instruction) -> Option<HaltState> {
    let params = ins.get_parameters(st);
    let new_address = 
        match params[0] == 0{
            true => params[1],
            false => st.address + ins.size() as i64

        };
    st.set_address(new_address);
    return None;
}
fn op_lessthan(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let params = ins.get_parameters(st);
    let answer: i64 = 
        match params[0] < params[1] {
            true => 1,
            false => 0

        };
    st.write(ins.get_target_address(st.relative_base), answer);
    st.increment_address(ins.size() as i64);
    return None;
}

fn op_equals(st: &mut State, ins: &Instruction) -> Option<HaltState> {

    let params = ins.get_parameters(st);
    let answer: i64 = 
        match params[0] == params[1] {
            true => 1,
            false => 0

        };

    st.write(ins.get_target_address(st.relative_base), answer);
    st.increment_address(ins.size() as i64);
    return None;
}
