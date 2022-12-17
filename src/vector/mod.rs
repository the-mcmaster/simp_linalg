mod mul_impl;
mod add_impl;

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

    pub fn lambda<F>(&self, funct: F) -> Vector<T>
    where
        F: Fn(&T) -> T
    {
        let mut params = vec![];
        
        for item in self.list() {
            params.push(funct(item))
        }

        Vector::from(params)
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