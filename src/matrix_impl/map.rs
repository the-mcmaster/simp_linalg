use crate::matrix_impl::Matrix;

impl<T> Matrix<T> {
    /// Applies a function dependent on value 
    /// to each corresponding element between
    /// the two matrices. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::Matrix;
    /// use simp_linalg::matrix;
    /// 
    /// let matrix1 = matrix![[1, 2],
    ///                       [3, 4]];
    /// 
    /// let matrix2 = matrix![[5, 6],
    ///                       [7, 8]];
    /// 
    /// let matrix3 = matrix1.map(&matrix2, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(matrix3, matrix![[5,  12],
    ///                             [21, 32]]);
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two matrices are not identically
    /// sized.
    pub fn map<F>(&self, other: &Matrix<T>, funct: F) -> Matrix<T>
    where
        F: Fn(&T, &T) -> T
    {
        if (self.rows != other.rows) || (self.cols != other.cols) { 
            panic!("Cannot map matrices of different sizes.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut new_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                new_row.push(funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(new_row)
        }

        Matrix::from(params)
    }

    /*
        For anyone following the source code, lambda.rs
        has a function called 'lambda_index' while map.rs
        does not. This is on purpose, as it is pointless.

        Consider what the generic function definition
        would be for a map_index.
        
        This is what it would be:
            F: Fn(usize, usize) -> T
        
        This is already used for lambda_index.

        Additionally, the matrices are independent from
        the function definition, and no actual mapping
        will be done.
        
        Therefore, it is pointless to add the method 'map_index',
        as it does not make sense.
    */

    /// Applies a function dependent on location and value 
    /// to each corresponding element between the two matrices. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix_impl::Matrix;
    /// use simp_linalg::matrix;
    /// 
    /// let matrix1 = matrix![[1, 2],
    ///                       [3, 4]];
    /// let matrix2 = matrix![[5, 6],
    ///                       [7, 8]];
    /// 
    /// let matrix3 = matrix1.map_enumerate(&matrix2, |row, col, val1, val2| val1 * val2 + (row * col + 1));
    /// 
    /// assert_eq!(matrix3, matrix![[6,  13],
    ///                             [22, 34]]);
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two matrices are not identically
    /// sized.
    pub fn map_enumerate<F>(&self, other: &Matrix<T>, funct: F) -> Matrix<T>
    where
        F: Fn(usize, usize, &T, &T) -> T
    {
        if (self.rows != other.rows) || (self.cols != other.cols) { 
            panic!("Cannot map matrices of different sizes.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut new_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                new_row.push(funct(row_idx, col_idx, &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(new_row)
        }

        Matrix::from(params)
    }
}