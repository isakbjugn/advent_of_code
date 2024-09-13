use std::any::type_name;
use num_traits::{Num, CheckedAdd};
use std::iter::Sum;

pub trait SumTo<T, U> {
    fn sum_to(&mut self) -> U where U: Num + Sum<U>, T: Into<U> + Copy;
    #[allow(unused)]
    fn checked_sum_to(&mut self) -> Result<U, String> where U: Num + CheckedAdd + Sum<U>, T: Into<U> + Copy;
}

impl<T, U, I> SumTo<T, U> for I
    where
        I: Iterator<Item = T>,
        T: Into<U> + Copy,
        U: Num + Sum<U>
{
    fn sum_to(&mut self) -> U {
        self.fold(U::zero(), |acc, x| acc + x.into())
    }

    fn checked_sum_to(&mut self) -> Result<U, String>
    where U: Num + CheckedAdd + Num + Sum<U>, T: Into<U> + Copy
    {
        self.try_fold(U::zero(), |acc, x| {
            acc.checked_add(&x.into()).ok_or_else(|| format!("attempt to sum {} to {} with overflow", type_name::<T>(), type_name::<U>()))
        })
    }
}
