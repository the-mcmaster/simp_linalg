use std::ops::{Add, Mul, AddAssign};

#[deny(missing_docs)]

use crate::matrix::Matrix;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T>
{
    size : usize,
    list : Vec<T>
}

#[allow(dead_code)]
impl<T> Vector<T>
where
    T: Copy
{
    /// Converts a Vector<T> to a column Matrix<T>, consuming the Vector.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.to_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![
    ///                                          vec![1],
    ///                                          vec![2],
    ///                                          vec![3]]));
    /// ```
    pub fn to_col_matrix(self) -> Matrix<T> {
        let mut matrix = vec![];
        for param in self.list {
            matrix.push(vec![param])
        }
        Matrix::new(matrix.len(), 1, matrix)
    }

    /// Converts a Vector<T> to a row Matrix<T>, consuming the Vector.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.to_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1, 2, 3]]));
    /// ```
    pub fn to_row_matrix(self) -> Matrix<T> {
        Matrix::new(1, self.list.len(), vec![self.list])
    }

    /// Returns the size of the Vector
    pub fn size(&self) -> usize {
        self.size
    }

    /// Borrows the list of elements in the Vector<T>
    pub fn list(&self) -> &Vec<T> {
        &self.list
    }

    /// Construct a new Vector<T>
    pub fn new(size: usize, list: Vec<T>) -> Self {
        Vector { size: size, list: list }
    }
}

/// Converts a Vec<T> to a Vector<T>
/// 
/// This function will never fail.
impl<T> From<Vec<T>> for Vector<T>
{
    fn from(vec: Vec<T>) -> Self {
        Vector {
            size : vec.len(),
            list : vec
        }
    }
}

/// The dot product implementation for 'Vector * Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3]);
/// let vector2 = Vector::from(vec![4, 5, 6]);
/// 
/// let value = vector1 * vector2;
/// 
/// assert_eq!(value, 32)
/// ```
/// 
/// # Panic!
/// 
/// This function will panic is the vectors are not the same size.
impl<T> Mul for Vector<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Default
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Cannot find dot product of two differently sized vectors.")
        }

        let mut product = T::default();
        for idx in 0..self.size {
            product += self.list[idx] * rhs.list[idx]
        }
        product
    }
}

/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3]);
/// let vector2 = Vector::from(vec![4, 5, 6]);
/// 
/// let vector3 = vector1 + vector2;
/// 
/// assert_eq!(vector3, Vector::from(vec![5, 7, 9]))
/// ```
/// 
/// # Panic!
/// 
/// This function will panic is the vectors are not the same size.
impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Vectors with different sizes cannot be added together.")
        }

        let mut params = vec![];
        for idx in 0..self.size {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}