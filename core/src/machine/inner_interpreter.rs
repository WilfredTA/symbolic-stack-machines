use crate::{
    constraint::Constraint,
    instructions::{AbstractExecRecord, AbstractInstruction}, environment::Env,
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, I, InstructionStepResult, InterpreterStepResult, E, V>
where
    I: AbstractInstruction<InstructionStepResult, E, V>,
    V: Default + Clone,
    E: Env
{
    fn step(&self, m: AbstractMachine<'a, I, E, V>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, I, E, V> InnerInterpreter<'a, I, AbstractExecRecord<E>, AbstractMachine<'a, I, E, V>, E, V>
    for ConcreteInnerInterpreter
where
    I: AbstractInstruction<AbstractExecRecord<E>, E, V>,
    V: Default + Clone,
    E: Env
{
    fn step(&self, m: AbstractMachine<'a, I, E, V>) -> MachineResult<AbstractMachine<'a, I, E, V>> {
        let i = m.pgm.get(m.pc.unwrap()).unwrap();

        let exec_record = i.exec(&m.stack, &m.mem, &m.env)?;

        Ok(m.apply::<E::RecordType>(
            exec_record.stack_diff,
            exec_record.mem_diff,
            exec_record.env_diff,
            exec_record.pc_change,
            exec_record.halt,
        ))
    }
}

pub type AbstractExecBranch<'a, I, E, V> = Vec<SingleBranch<'a, I, E, V>>;

pub type SingleBranch<'a, I, E, V> = (AbstractMachine<'a, I, E, V>, Vec<Constraint>);

pub struct SymbolicInnerInterpreter {}

impl<'a, I, E, V> InnerInterpreter<'a, I, Vec<AbstractExecRecord<E>>, AbstractExecBranch<'a, I, E, V>, E, V>
    for SymbolicInnerInterpreter
where
    I: AbstractInstruction<Vec<AbstractExecRecord<E>>, E, V>,
    V: Default + Clone,
    E: Env,
{
    fn step(&self, m: AbstractMachine<'a, I, E, V>) -> MachineResult<AbstractExecBranch<'a, I, E, V>> {
        let pgm = m.pgm;
        let pc = m.pc.unwrap();

        let i = pgm.get(pc).unwrap();

        let exec_records = i.exec(&m.stack, &m.mem, &m.env)?;

        let rv = exec_records
            .into_iter()
            .map(|exec_record| {
                let constraints = exec_record.constraints.unwrap_or_default();

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
