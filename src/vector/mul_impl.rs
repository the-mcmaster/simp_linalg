use std::ops::{Mul, AddAssign};
use crate::vector::Vector;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// The [Multiplication][std::ops::Mul] implementation for '&Vector * &Vector'.
/// 
/// This calculates the dot product of the two vectors.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3]);
/// let vector2 = Vector::from(vec![4, 5, 6]);
/// 
/// let value = &vector1 * &vector2;
/// 
/// assert_eq!(value, 32)
/// ```
/// 
/// # Panic!
/// This function will panic if the vectors are not the same size.
impl<T> Mul for &Vector<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Default
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Cannot find dot product of two differently sized vectors.")
        }

        let mut product = T::default();
        
        for idx in 0..self.len() {
            product += self.list[idx] * rhs.list[idx]
        }

        product
    }
}


//
//
//          Borrowed/Owned Implementation
//
//
impl<T> Mul<Vector<T>> for &Vector<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Default
{
    type Output = T;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Cannot find dot product of two differently sized vectors.")
        }

        let mut product = T::default();
        
        for idx in 0..self.len() {
            product += self.list[idx] * rhs.list[idx]
        }

        product
    }
}


//
//
//          Owned/Borrowed Implementation
//
//
impl<T> Mul<&Vector<T>> for Vector<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Default
{
    type Output = T;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Cannot find dot product of two differently sized vectors.")
        }

        let mut product = T::default();
        
        for idx in 0..self.len() {
            product += self.list[idx] * rhs.list[idx]
        }

        product
    }
}


//
//
//          Owned/Owned Implementation
//
//
impl<T> Mul for Vector<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Default
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Cannot find dot product of two differently sized vectors.")
        }

        let mut product = T::default();
        
        for idx in 0..self.len() {
            product += self.list[idx] * rhs.list[idx]
        }

        product
    }
}


//
//
//          Borrowed/Scalar Implementation
//
//
impl<T> Mul<T> for &Vector<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = vec![];
        for item in self.list() {
            params.push(rhs * *item)
        }
        Vector::from(params)
    }
}

//
//
//          Owned/Scalar Implementation
//
//
impl<T> Mul<T> for Vector<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = vec![];
        for item in self.list() {
            params.push(rhs * *item)
        }
        Vector::from(params)
    }
}