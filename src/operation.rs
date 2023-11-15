use num_traits::{One, Zero};

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

pub struct Product;

impl<N: One> Operation<N> for Product
where
    for<'a> &'a N: std::ops::Mul<Output = N>,
{
    fn zero() -> N {
        N::one()
    }

    fn combine(lhs: &N, rhs: &N) -> N {
        lhs * rhs
    }
}
