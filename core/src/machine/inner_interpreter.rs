use crate::{
    constraint::Constraint,
    instructions::{AbstractExecRecord, AbstractInstruction},
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, I, InstructionStepResult, InterpreterStepResult>
where
    I: AbstractInstruction<InstructionStepResult>,
{
    fn step(&self, m: AbstractMachine<'a, I>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, I> InnerInterpreter<'a, I, AbstractExecRecord, AbstractMachine<'a, I>>
    for ConcreteInnerInterpreter
where
    I: AbstractInstruction<AbstractExecRecord>,
{
    fn step(&self, m: AbstractMachine<'a, I>) -> MachineResult<AbstractMachine<'a, I>> {
        let i = m.pgm.get(m.pc.unwrap()).unwrap();

        let exec_record = i.exec(&m.stack, &m.mem, &m.env)?;

        Ok(m.apply(
            exec_record.stack_diff,
            exec_record.mem_diff,
            exec_record.env_diff,
            exec_record.pc_change,
            exec_record.halt,
        ))
    }
}

pub type AbstractExecBranch<'a, I> = Vec<SingleBranch<'a, I>>;

pub type SingleBranch<'a, I> = (AbstractMachine<'a, I>, Vec<Constraint>);

pub struct SymbolicInnerInterpreter {}

impl<'a, I> InnerInterpreter<'a, I, Vec<AbstractExecRecord>, AbstractExecBranch<'a, I>>
    for SymbolicInnerInterpreter
where
    I: AbstractInstruction<Vec<AbstractExecRecord>>,
{
    fn step(&self, m: AbstractMachine<'a, I>) -> MachineResult<AbstractExecBranch<'a, I>> {
        let pgm = m.pgm;
        let pc = m.pc.unwrap();

        let i = pgm.get(pc).unwrap();

        let exec_records = i.exec(&m.stack, &m.mem, &m.env)?;

        let rv = exec_records
            .into_iter()
            .map(|exec_record| {
                let constraints = exec_record.constraints.unwrap_or(vec![]);

                let new_machine = m.xclone().apply(
                    exec_record.stack_diff,
                    exec_record.mem_diff,
                    exec_record.env_diff,
                    exec_record.pc_change,
                    exec_record.halt,
                );

                (new_machine, constraints)
            })
            .collect();

        Ok(rv)
    }
}
