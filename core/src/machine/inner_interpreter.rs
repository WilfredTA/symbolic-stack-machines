use crate::{
    constraint::Constraint,
    environment::EnvExtension,
    instructions::{AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord},
    memory::{Mem, WriteableMem},
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, M, E, I, InstructionStepResult, InterpreterStepResult>
where
    M: Mem,
    E: EnvExtension,
    I: AbstractInstruction<M, E, InstructionStepResult>,
{
    fn step(&self, m: AbstractMachine<'a, M, E, I>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, M, E, I>
    InnerInterpreter<
        'a,
        M,
        E,
        I,
        ConcreteAbstractExecRecord<M, E::DiffRecordType>,
        AbstractMachine<'a, M, E, I>,
    > for ConcreteInnerInterpreter
where
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<M, E, ConcreteAbstractExecRecord<M, E::DiffRecordType>>,
{
    fn step(
        &self,
        m: AbstractMachine<'a, M, E, I>,
    ) -> MachineResult<AbstractMachine<'a, M, E, I>> {
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

pub type AbstractExecBranch<'a, M, E, I, C> = Vec<SingleBranch<'a, M, E, I, C>>;

pub type SingleBranch<'a, M, E, I, C> = (AbstractMachine<'a, M, E, I>, Vec<Constraint<C>>);

pub struct SymbolicInnerInterpreter {}

impl<'a, M, E, I, C>
    InnerInterpreter<
        'a,
        M,
        E,
        I,
        Vec<AbstractExecRecord<M, E::DiffRecordType, C>>,
        AbstractExecBranch<'a, M, E, I, C>,
    > for SymbolicInnerInterpreter
where
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<M, E, Vec<AbstractExecRecord<M, E::DiffRecordType, C>>>,
{
    fn step(
        &self,
        m: AbstractMachine<'a, M, E, I>,
    ) -> MachineResult<AbstractExecBranch<'a, M, E, I, C>> {
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
