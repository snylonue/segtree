use num_traits::{One, Zero};

pub trait Operation<T> {
    fn unit() -> T;
    fn combine(lhs: &T, rhs: &T) -> T;
}

pub struct Sum;

impl<N: Zero> Operation<N> for Sum
where
    for<'a> &'a N: std::ops::Add<Output = N>,
{
    fn unit() -> N {
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
    fn unit() -> N {
        N::one()
    }

    fn combine(lhs: &N, rhs: &N) -> N {
        lhs * rhs
    }
}
