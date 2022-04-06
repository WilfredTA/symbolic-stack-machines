pub trait MachineEq {
    fn machine_eq(&self, other: &Self) -> Self;

    // self is predicate
    fn machine_ite(self, then: Self, xelse: Self) -> Self;
}
