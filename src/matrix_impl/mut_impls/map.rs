use crate::matrix_impl::{MatrixMap, Matrix};

impl<'a, T> MatrixMap<T> for &'a mut Matrix<T> {
    type Other = &'a Matrix<T>;

    type Output = &'a mut Matrix<T>;

    /// Applies a function dependent on location and value 
    /// to each corresponding element between the two matrices. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let mut matrix1 = matrix![[1, 2],
    ///                           [3, 4]];
    /// 
    /// let matrix2 = matrix![[5, 6],
    ///                       [7, 8]];
    /// 
    /// (&mut matrix1).map_enumerate(&matrix2, |row, col, val1, val2| val1 * val2 + (row * col + 1));
    /// 
    /// assert_eq!(matrix1, matrix![[6,  13],
    ///                             [22, 34]]);
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two matrices are not identically
    /// sized.
    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, usize, &T, &T) -> T {
        if (self.rows != other.rows) || (self.cols != other.cols) { 
            panic!("Cannot map matrices of different sizes.")
        }

        for row_idx in 0..self.rows {

            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(
                    row_idx, col_idx, &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]
                );
            }

        }

        self
    }

    /// Applies a function dependent on value 
    /// to each corresponding element between
    /// the two matrices. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::prelude::*;
    /// 
    /// let mut matrix1 = matrix![[1, 2],
    ///                           [3, 4]];
    /// 
    /// let matrix2 = matrix![[5, 6],
    ///                       [7, 8]];
    /// 
    /// (&mut matrix1).map(&matrix2, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(matrix1, matrix![[5,  12],
    ///                             [21, 32]]);
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two matrices are not identically
    /// sized.
    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T {
        if (self.rows != other.rows) || (self.cols != other.cols) { 
            panic!("Cannot map matrices of different sizes.")
        }

        for row_idx in 0..self.rows {

            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(
                    &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]
                );
            }
            
        }

        self
    }
}