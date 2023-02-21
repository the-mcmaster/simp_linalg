use crate::matrix_impl::{Matrix, traits::MatrixLambda};

impl<T> MatrixLambda<T> for &Matrix<T> {
    type Output = Matrix<T>;

    /// Applies a function dependent on value
    /// to each individual element in the matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let matrix1 = matrix![[1, 2],
    ///                       [3, 4]];
    /// 
    /// let matrix2 = matrix1.lambda(|val| val * val);
    /// 
    /// assert_eq!(matrix2, matrix![[1, 4],
    ///                             [9, 16]]);
    /// ```
    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T {
        let mut params = Vec::with_capacity(self.rows);
        
        
        for row in &self.matrix {
            let mut new_row = Vec::with_capacity(self.cols);
            
            for col in row {
                new_row.push(funct(col))
            }
            
            params.push(new_row)
        }

        Matrix::from(params)
    }

    /// Applies a function dependent on location
    /// to each individual element in the matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let matrix1 = matrix![[1, 2],
    ///                       [3, 4]];
    /// 
    /// let matrix2 = matrix1.lambda_index(|row, col| row * col + row + col);
    /// 
    /// assert_eq!(matrix2, matrix![[0, 1],
    ///                             [1, 3]]);
    /// ```
    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize) -> T {
        let mut params = Vec::with_capacity(self.rows);
        
        for row_idx in 0..self.rows {
            let mut new_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                new_row.push(funct(row_idx, col_idx))
            }

            params.push(new_row)
        }

        Matrix::from(params)
    }

    /// Applies a function dependent on location and value
    /// to each individual element in the matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let matrix1 = matrix![[1, 2],
    ///                       [3, 4]];
    /// 
    /// let matrix2 = matrix1.lambda_enumerate(|row, col, val| row + col + val);
    /// 
    /// assert_eq!(matrix2, matrix![[1, 3],
    ///                             [4, 6]]);
    /// ```
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize, &T) -> T {
        let mut params = Vec::with_capacity(self.rows);

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut new_row = Vec::with_capacity(self.cols);

            for (col_idx, col) in row.iter().enumerate() {
                new_row.push(funct(row_idx, col_idx, col))
            }
            
            params.push(new_row)
        }

        Matrix::from(params)
    }
}