use std::ops::{Add, Mul, AddAssign};

use crate::matrix::Matrix;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
/// The Vector type.
pub struct Vector<T>
{
    list : Vec<T>
}

#[allow(dead_code)]
impl<T> Vector<T>
{
    /// Converts a Vector<T> to a column Matrix<T>, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1],
    ///                                          vec![2],
    ///                                          vec![3]]));
    /// 
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![vec!["woah"], vec!["what"]]);
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![vec!["woah"]],
    ///                                          vec![vec!["what"]]]));
    /// ```
    pub fn into_col_matrix(self) -> Matrix<T> {
        let mut matrix = vec![];
        for param in self.list {
            matrix.push(vec![param])
        }
        Matrix::from(matrix)
    }

    /// Converts a Vector<T> to a row Matrix<T>, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1, 2, 3]]));
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![vec!["woah"], vec!["what"]]);
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![vec!["woah"], vec!["what"]]]));
    /// ```
    pub fn into_row_matrix(self) -> Matrix<T> {
        Matrix::from(vec![self.list])
    }

    /// Returns the length of the Vector<T>.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Borrows the list of elements in the Vector<T>.
    pub fn list(&self) -> &Vec<T> {
        &self.list
    }
}

/// Converts a Vec<T> to a Vector<T>
/// 
/// This function will never fail since a vector can
/// be constructed with any valid [std::vec::Vec]. However,
/// the methods available are based upon the Vector's internal
/// type's traits, such as multiplcation and addition.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3, 4]);
/// let vector2 = Vector::from(vec![1.7, -0.03]);
/// 
/// // Vectors can be constructed with any std::vec::Vec.
/// let vector3 = Vector::from(vec!["Hello", "I'm in the vector!"]);
/// ```
impl<T> From<Vec<T>> for Vector<T>
{
    fn from(vec: Vec<T>) -> Self {
        Vector {
            list : vec
        }
    }
}

/// The [Multiplication][std::ops::Mul] implementation for 'Vector * Vector'.
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

/// The [Addition][std::ops::Add] implementation for 'Vector + Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3]);
/// let vector2 = Vector::from(vec![4, 5, 6]);
/// 
/// let vector3 = &vector1 + &vector2;
/// 
/// assert_eq!(vector3, Vector::from(vec![5, 7, 9]))
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<T> Add for &Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        let mut params = vec![];
        for idx in 0..self.len() {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}