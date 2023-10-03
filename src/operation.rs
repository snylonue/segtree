pub trait Operation<T> {
    fn zero() -> T;
    fn combine(lhs: &T, rhs: &T) -> T;
}

pub struct Add;

impl Operation<i64> for Add {
    fn zero() -> i64 {
        0
    }

    fn combine(lhs: &i64, rhs: &i64) -> i64 {
        lhs + rhs
    }
}
