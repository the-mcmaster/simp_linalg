use crate::matrix_impl::{Matrix, traits::MatrixLambda};

/// Unlike the &Matrix implementation of MatrixLambda,
/// this implementation returns the left-hand-side 
/// borrowed mutable Matrix with changes applied.
impl<'a, T> MatrixLambda<T> for &'a mut Matrix<T> {
    type Output = &'a mut Matrix<T>;

    /// Applies a function dependent on value
    /// to each individual element in the matrix
    /// and applies the transformation to the 
    /// left-hand-side matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let mut matrix1 = matrix![[1, 2],
    ///                           [3, 4]];
    /// 
    /// (&mut matrix1).lambda(|val| val * val);
    /// 
    /// assert_eq!(matrix1, matrix![[1, 4],
    ///                             [9, 16]]);
    /// ```
    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(&self.matrix[row][col])
            }
        }
        self
    }

    /// Applies a function dependent on location
    /// to each individual element in the matrix
    /// and applies the transformation to the 
    /// left-hand-side matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let mut matrix1 = matrix![[1, 2],
    ///                           [3, 4]];
    /// 
    /// (&mut matrix1).lambda_index(|row, col| row * col + row + col);
    /// 
    /// assert_eq!(matrix1, matrix![[0, 1],
    ///                             [1, 3]]);
    /// ```
    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(row, col)
            }
        }
        self
    }

    /// Applies a function dependent on location and value
    /// to each individual element in the matrix
    /// and applies the transformation to the 
    /// left-hand-side matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let mut matrix1 = matrix![[1, 2],
    ///                           [3, 4]];
    /// 
    /// (&mut matrix1).lambda_enumerate(|row, col, val| row + col + val);
    /// 
    /// assert_eq!(matrix1, matrix![[1, 3],
    ///                             [4, 6]]);
    /// ```
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize, &T) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(row, col, &self.matrix[row][col])
            }
        }
        self
    }
}