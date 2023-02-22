mod mut_impls;

mod mul_impl;
mod add_impl;
mod lambda;
mod map;
pub mod prelude;
pub mod traits;

use crate::matrix_impl::Matrix;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// The Vector type.
/// 
/// By default, overloaded addition and multiplication
/// is implemented. However, other functionality like
/// lambda and map functions are only available through
/// using their associated traits 'VectorLambda' and 'VectorMap'.
/// 
/// The preludes in [Root Prelude][crate::prelude] and [Vector Prelude][crate::vector_impl::prelude]
/// use these traits by default.
pub struct Vector<T>
{
    list : Vec<T>
}

impl<T> Vector<T>
{
    /// Converts a Vector into a column Matrix, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = vector![1, 2, 3];
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, matrix![
    ///     [1],
    ///     [2],
    ///     [3]
    /// ]);
    /// 
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = vector![vec!["woah"], vec!["what"]];
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, matrix![
    ///     [vec!["woah"]],
    ///     [vec!["what"]]
    /// ]);
    /// ```
    pub fn into_col_matrix(self) -> Matrix<T> {
        let mut matrix = Vec::with_capacity(self.len());

        for param in self.list {
            matrix.push(vec![param])
        }

        Matrix::from(matrix)
    }

    /// Converts a Vector<T> into a row Matrix<T>, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = vector![1, 2, 3];
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, matrix![
    ///     [1, 2, 3]
    /// ]);
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = vector![vec!["woah"], vec!["what"]];
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, matrix![
    ///     [vec!["woah"], vec!["what"]]
    /// ]);
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

    /// Unwraps the vector.
    pub fn into_inner(self) -> Vec<T> {
        self.list
    }
}

/// Converts a [Vec][std::vec::Vec] to a Vector.
/// 
/// This function will never fail since a Vector can
/// be constructed with any valid [std::vec::Vec]. However,
/// the methods available are based upon the Vector's internal
/// type's traits, such as multiplcation and addition.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3, 4];
/// let vector2 = vector![1.7, -0.03];
/// 
/// // Vectors can be constructed with any std::vec::Vec.
/// let vector3 = vector!["Hello", "I'm in the vector!"];
/// ```
impl<T> From<Vec<T>> for Vector<T>
{
    fn from(vec: Vec<T>) -> Self {
        Vector {
            list : vec
        }
    }
}