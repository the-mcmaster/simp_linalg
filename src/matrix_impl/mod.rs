mod mut_impls;

mod mul_impl;
mod add_impl;
mod lambda;
mod map;
pub mod prelude;
pub mod traits;

use crate::vector_impl::Vector;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// The Matrix type.
/// 
/// By default, overloaded addition and multiplication
/// is implemented. However, other functionality like
/// lambda and map functions are only available through
/// using their associated traits 'MatrixLambda' and 'MatrixMap'.
/// 
/// The preludes in [Root Prelude][crate::prelude] and [Matrix Prelude][crate::matrix_impl::prelude]
/// use these traits by default.
pub struct Matrix<T>
{
    rows : usize,
    cols : usize,
    matrix : Vec<Vec<T>>
}

impl<T> Matrix<T>
where
    T: Copy
{
    /// Converts a single dimentional Matrix into a Vector, consuming the Matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let row_matrix = matrix![[1, 2, 4]];
    /// 
    /// let col_matrix = matrix![[1],
    ///                          [2],
    ///                          [3]];
    /// 
    /// assert_eq!(row_matrix.into_vector(), vector![1, 2, 4]);
    /// assert_eq!(col_matrix.into_vector(), vector![1, 2, 3]);
    /// ```
    /// # Panic!
    /// This function will panic if neither rows nor columns are equal to 1.
    /// ## Example
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let row_matrix = matrix![[1,2]];
    /// 
    /// let col_matrix = matrix![[1],
    ///                          [2]];
    /// 
    /// let both_matrix = matrix![[1,2],
    ///                           [3,4]];
    /// 
    /// let vector1 = row_matrix.into_vector();
    /// let vector2 = col_matrix.into_vector();
    /// // Panics vvv
    /// // let vector3 = both_matrix.into_vector();
    /// ```
    pub fn into_vector(self) -> Vector<T> {
        if self.rows == 1 {
            let mut params = Vec::with_capacity(self.cols);

            for col in &self.matrix[0] {
                params.push(*col)
            }

            return Vector::from(params)
        }
        
        else if self.cols == 1 {
            let mut params: Vec<T> = Vec::with_capacity(self.rows);

            for row in self.matrix {
                params.push(row[0])
            }

            return Vector::from(params);
        }
        
        else {
            panic!("Cannot convert matrix because neither rows nor columns are 1")
        }
    }
}

impl<T> Matrix<T> {
    /// Returns the number of rows of the Matrix<T>.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns of the Matrix<T>.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Unwraps the matrix.
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.matrix
    }
}

/// # Panic!
/// 
/// This function will panic if there exists a differently sized internal [vec][std::vec::Vec].
/// 
/// ## Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 0],
///                       [1, 2]];
/// //This will panic.
/// //let matrix2 = matrix![[1],
/// //                      [1, 2]];
/// ```
impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(params: Vec<Vec<T>>) -> Self {

        let rows = params.len();
        let cols = params[0].len();

        for row in 1..rows {
            if params[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }

        Matrix {
            rows : params.len(),
            cols : params[0].len(),
            matrix : params
        }
    }
}

impl<T> From<&Vec<Vec<T>>> for Matrix<T>
where
    T: Copy
{
    fn from(params: &Vec<Vec<T>>) -> Self {

        let rows = params.len();
        let cols = params[0].len();

        for row in 1..rows {
            if params[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }

        Matrix {
            rows : params.len(),
            cols : params[0].len(),
            matrix : params.to_vec()
        }
    }
}