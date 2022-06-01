use crate::{
    constraint::Constraint,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, EnvExtension,
    },
    memory::{Mem, WriteableMem},
    stack::Stack,
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>
where
    S: Stack,
    M: Mem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
{
    fn step(&self, m: AbstractMachine<'a, S, M, E, I>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, S, M, E, I>
    InnerInterpreter<
        'a,
        S,
        M,
        E,
        I,
        ConcreteAbstractExecRecord<S, M, E::DiffRecordType>,
        AbstractMachine<'a, S, M, E, I>,
    > for ConcreteInnerInterpreter
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, ConcreteAbstractExecRecord<S, M, E::DiffRecordType>>,
{
    fn step(
        &self,
        m: AbstractMachine<'a, S, M, E, I>,
    ) -> MachineResult<AbstractMachine<'a, S, M, E, I>> {
        let i = m.pgm.get(m.pc.unwrap()).unwrap();

        let exec_record = i.exec(&m.stack, &m.mem, &m.custom_env)?;

        Ok(m.apply(
            exec_record.stack_diff,
            exec_record.mem_diff,
            exec_record.ext_diff,
            exec_record.pc_change,
            exec_record.halt,
        ))
    }
}

pub type AbstractExecBranch<'a, S, M, E, I, C> = Vec<SingleBranch<'a, S, M, E, I, C>>;

pub type SingleBranch<'a, S, M, E, I, C> = (AbstractMachine<'a, S, M, E, I>, Vec<Constraint<C>>);

pub struct SymbolicInnerInterpreter {}

impl<'a, S, M, E, I, C>
    InnerInterpreter<
        'a,
        S,
        M,
        E,
        I,
        Vec<AbstractExecRecord<S, M, E::DiffRecordType, C>>,
        AbstractExecBranch<'a, S, M, E, I, C>,
    > for SymbolicInnerInterpreter
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, Vec<AbstractExecRecord<S, M, E::DiffRecordType, C>>>,
{
    fn step(
        &self,
        m: AbstractMachine<'a, S, M, E, I>,
    ) -> MachineResult<AbstractExecBranch<'a, S, M, E, I, C>> {
        let pgm = m.pgm;
        let pc = m.pc.unwrap();

        let i = pgm.get(pc).unwrap();

        let exec_records = i.exec(&m.stack, &m.mem, &m.custom_env)?;

        let rv = exec_records
            .into_iter()
            .map(|exec_record| {
                let constraints = exec_record.constraints.unwrap_or(vec![]);

                let new_machine = m.xclone().apply(
                    exec_record.stack_diff,
                    exec_record.mem_diff,
                    exec_record.ext_diff,
                    exec_record.pc_change,
                    exec_record.halt,
                );

                (new_machine, constraints)
            })
            .collect();

        Ok(rv)
    }
}
