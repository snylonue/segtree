use num_traits::Zero;

pub trait Operation<T> {
    fn zero() -> T;
    fn combine(lhs: &T, rhs: &T) -> T;
}

pub struct Add;

impl<N: Zero> Operation<N> for Add
where
    for<'a> &'a N: std::ops::Add<Output = N>,
{
    fn zero() -> N {
        N::zero()
    }

    fn combine(lhs: &N, rhs: &N) -> N {
        lhs + rhs
    }
}
