use crate::{
    constraint::Constraint,
    instructions::{AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord},
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, I, InstructionStepResult, InterpreterStepResult>
where
    I: AbstractInstruction<InstructionStepResult>,
{
    fn step(&self, m: AbstractMachine<'a, I>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, I> InnerInterpreter<'a, I, ConcreteAbstractExecRecord, AbstractMachine<'a, I>>
    for ConcreteInnerInterpreter
where
    I: AbstractInstruction<ConcreteAbstractExecRecord>,
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

pub type AbstractExecBranch<'a, I, C> = Vec<SingleBranch<'a, I, C>>;

pub type SingleBranch<'a, I, C> = (AbstractMachine<'a, I>, Vec<Constraint<C>>);

pub struct SymbolicInnerInterpreter {}

impl<'a, I, C> InnerInterpreter<'a, I, Vec<AbstractExecRecord<C>>, AbstractExecBranch<'a, I, C>>
    for SymbolicInnerInterpreter
where
    I: AbstractInstruction<Vec<AbstractExecRecord<C>>>,
{
    fn step(&self, m: AbstractMachine<'a, I>) -> MachineResult<AbstractExecBranch<'a, I, C>> {
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
