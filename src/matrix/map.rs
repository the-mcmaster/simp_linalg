use crate::matrix::Matrix;

impl<T> Matrix<T> {
    /// Applies a function to each corresponding 
    /// elements between the two matrices. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// 
    /// let matrix1 = Matrix::from(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    /// let matrix2 = Matrix::from(vec![vec![5, 6],
    ///                                 vec![7, 8]]);
    /// 
    /// let matrix3 = matrix1.map(&matrix2, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(matrix3, Matrix::from(vec![vec![5,  12],
    ///                                       vec![21, 32]]));
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

        let mut rows = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut cols = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                cols.push(funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            rows.push(cols)
        }

        Matrix::from(rows)
    }
}