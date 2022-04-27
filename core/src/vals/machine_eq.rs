pub trait MachineEq {
    type Pred;

    fn machine_eq(&self, other: &Self) -> Self::Pred;
    fn machine_ite(pred: Self::Pred, then: Self, xelse: Self) -> Self;
}
