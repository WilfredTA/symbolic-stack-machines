mod z3;

pub use self::z3::{Z3Constraint, Z3Solver, Z3Value};

pub type Model<T> = Vec<(String, T)>;

#[derive(Debug)]
pub struct SolverOutput<ModelOutput, AdditionalOutput> {
    pub model: Model<ModelOutput>,
    pub additional_model: Model<AdditionalOutput>,
    pub additional: Vec<Option<AdditionalOutput>>
}

pub trait Solver<Constraint, AdditionalValue, ModelOutput, AdditionalValueOutput> {
    fn solve(
        &self,
        constraints: &Vec<Constraint>,
        additional: Vec<AdditionalValue>,
    ) -> Option<SolverOutput<ModelOutput, AdditionalValueOutput>>;
}
