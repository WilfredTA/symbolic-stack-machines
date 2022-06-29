use crate::{
    constraint::Constraint,
    environment::EnvExtension,
    instructions::{AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord},
};

use super::{r#abstract::AbstractMachine, MachineResult};

pub trait InnerInterpreter<'a, E, I, InstructionStepResult, InterpreterStepResult>
where
    E: EnvExtension,
    I: AbstractInstruction<E, InstructionStepResult>,
{
    fn step(&self, m: AbstractMachine<'a, E, I>) -> MachineResult<InterpreterStepResult>;
}

pub struct ConcreteInnerInterpreter {}

impl<'a, E, I>
    InnerInterpreter<
        'a,
        E,
        I,
        ConcreteAbstractExecRecord<E::DiffRecordType>,
        AbstractMachine<'a, E, I>,
    > for ConcreteInnerInterpreter
where
    E: EnvExtension,
    I: AbstractInstruction<E, ConcreteAbstractExecRecord<E::DiffRecordType>>,
{
    fn step(&self, m: AbstractMachine<'a, E, I>) -> MachineResult<AbstractMachine<'a, E, I>> {
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

pub type AbstractExecBranch<'a, E, I, C> = Vec<SingleBranch<'a, E, I, C>>;

pub type SingleBranch<'a, E, I, C> = (AbstractMachine<'a, E, I>, Vec<Constraint<C>>);

pub struct SymbolicInnerInterpreter {}

impl<'a, E, I, C>
    InnerInterpreter<
        'a,
        E,
        I,
        Vec<AbstractExecRecord<E::DiffRecordType, C>>,
        AbstractExecBranch<'a, E, I, C>,
    > for SymbolicInnerInterpreter
where
    E: EnvExtension,
    I: AbstractInstruction<E, Vec<AbstractExecRecord<E::DiffRecordType, C>>>,
{
    fn step(&self, m: AbstractMachine<'a, E, I>) -> MachineResult<AbstractExecBranch<'a, E, I, C>> {
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
