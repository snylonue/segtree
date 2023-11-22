use num_traits::{One, Zero};

pub trait Monoid<T> {
    fn unit(&self) -> T;
    fn combine(&self, lhs: &T, rhs: &T) -> T;
}

pub struct Sum;

impl<N: Zero> Monoid<N> for Sum
where
    for<'a> &'a N: std::ops::Add<Output = N>,
{
    fn unit(&self) -> N {
        N::zero()
    }

    fn combine(&self, lhs: &N, rhs: &N) -> N {
        lhs + rhs
    }
}

pub struct Product;

impl<N: One> Monoid<N> for Product
where
    for<'a> &'a N: std::ops::Mul<Output = N>,
{
    fn unit(&self) -> N {
        N::one()
    }

    fn combine(&self, lhs: &N, rhs: &N) -> N {
        lhs * rhs
    }
}
