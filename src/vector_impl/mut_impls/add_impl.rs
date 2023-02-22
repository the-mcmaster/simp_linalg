use std::ops::Add;
use crate::vector_impl::Vector;

//
//
//          Borrowed Mutable/Borrowed Mut Implementation
//
//
/// The [addition][std::ops::Add] implementation for '&mut Vector + &mut Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::prelude::*;
/// 
/// let mut vector1 = vector![1, 2, 3];
/// let mut vector2 = vector![4, 5, 6];
/// 
/// &mut vector1 + &mut vector2;
/// 
/// assert_eq!(vector1, vector![5, 7, 9])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<'a, T> Add for &'a mut Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = &'a Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        for idx in 0..self.len() {
            self.list[idx] = self.list[idx] + rhs.list[idx]
        }

        self
    }
}

//
//
//          Borrowed Mutable/Borrowed Implementation
//
//
/// The [addition][std::ops::Add] implementation for '&mut Vector + &Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::prelude::*;
/// 
/// let mut vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// &mut vector1 + &vector2;
/// 
/// assert_eq!(vector1, vector![5, 7, 9])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<'a, T> Add<&'a Vector<T>> for &'a mut Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = &'a Vector<T>;

    fn add(self, rhs: &'a Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        for idx in 0..self.len() {
            self.list[idx] = self.list[idx] + rhs.list[idx]
        }

        self
    }
}


//
//
//          Borrowed Mutable/Owned Implementation
//
//
/// The [addition][std::ops::Add] implementation for '&mut Vector + Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::prelude::*;
/// 
/// let mut vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // Notice that 'vector2' is moved here
/// &mut vector1 + vector2;
/// 
/// assert_eq!(vector1, vector![5, 7, 9]);
/// ```
/// This is useful for addition of vectors that are scaled.
/// ```
/// use simp_linalg::vector_impl::prelude::*;
/// 
/// let mut vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // The result of '&vector2 * 2' is an owned Vector,
/// // which is then added to '&mut vector1'.
/// &mut vector1 + &vector2 * 2;
/// 
/// assert_eq!(vector1, vector![9, 12, 15])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<'a, T> Add<Vector<T>> for &'a mut Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = &'a mut Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        for idx in 0..self.len() {
            self.list[idx] = self.list[idx] + rhs.list[idx]
        }

        self
    }
}