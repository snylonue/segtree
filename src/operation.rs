use num_traits::{One, Zero};

pub trait Monoid<T> {
    fn zero(&self) -> T;
    fn combine(&self, lhs: &T, rhs: &T) -> T;
}

pub struct Sum;

impl<N: Zero> Monoid<N> for Sum
where
    for<'a> &'a N: std::ops::Add<Output = N>,
{
    fn zero(&self) -> N {
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
    fn zero(&self) -> N {
        N::one()
    }

    fn combine(&self, lhs: &N, rhs: &N) -> N {
        lhs * rhs
    }
}

pub struct MonoidType<T, F> {
    pub zero: T,
    pub op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> Monoid<T> for MonoidType<T, F> {
    fn zero(&self) -> T {
        self.zero.clone()
    }

    fn combine(&self, lhs: &T, rhs: &T) -> T {
        (self.op)(lhs, rhs)
    }
}
