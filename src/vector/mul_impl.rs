use std::ops::{Mul, AddAssign};
use crate::vector::Vector;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// The [multiplication][std::ops::Mul] implementation for '&Vector * &Vector'.
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
/// The [multiplication][std::ops::Mul] implementation for '&Vector * Vector'.
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
/// // Notice 'vector2' is moved here.
/// let value = &vector1 * vector2;
/// 
/// assert_eq!(value, 32)
/// ```
/// 
/// # Panic!
/// This function will panic if the vectors are not the same size.
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
/// The [multiplication][std::ops::Mul] implementation for 'Vector * &Vector'.
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
/// // Notice 'vector1' is moved here.
/// let value = vector1 * &vector2;
/// 
/// assert_eq!(value, 32)
/// ```
/// 
/// # Panic!
/// This function will panic if the vectors are not the same size.
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
/// The [multiplication][std::ops::Mul] implementation for 'Vector * Vector'.
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
/// // Notice 'vector1' and 'vector2' are moved here.
/// let value = vector1 * &vector2;
/// 
/// assert_eq!(value, 32)
/// ```
/// 
/// # Panic!
/// This function will panic if the vectors are not the same size.
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
/// The [multiplication][std::ops::Mul] implementation for '&Vector * T'.
/// 
/// In contrast to common mathematical notation,
/// the scalar must be on the right of the vector.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector = Vector::from(vec![1, 2, 3]);
/// 
/// assert_eq!(&vector * 3, Vector::from(vec![3, 6, 9]))
/// ```
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
/// The [multiplication][std::ops::Mul] implementation for '&Vector * T'.
/// 
/// In contrast to common mathematical notation,
/// the scalar must be on the right of the vector.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector = Vector::from(vec![1, 2, 3]);
/// 
/// // Notice that the vector is moved here.
/// assert_eq!(vector * 3, Vector::from(vec![3, 6, 9]))
/// ```
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