use std::ops::Add;
use crate::matrix_impl::Matrix;

//
//
//          Borrowed Mutable/Borrowed Mutable Implementation
//
//
/// [Addition][std::ops::Add] implementation of '&mut Matrix<T> + &mut Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let mut matrix1 = matrix![[1, 2],
///                           [3, 4],
///                           [5, 6]];
/// 
/// let mut matrix2 = matrix![[7,  8],
///                           [9,  10],
///                           [11, 12]];
/// 
/// &mut matrix1 + &mut matrix2;
/// 
/// assert_eq!(matrix1, matrix![[8,  10],
///                             [12, 14],
///                             [16, 18]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<'a, T> Add for &'a mut Matrix<T>
where
    T: Add<Output = T> + Copy + 'a
{
    type Output = &'a mut Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx]
            }
        }
        self
    }
}


//
//
//          Borrowed Mutable/Owned Implementation
//
//
/// [Addition][std::ops::Add] implementation of '&mut Matrix<T> + Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let mut matrix1 = matrix![[1, 2],
///                           [3, 4],
///                           [5, 6]];
/// 
/// let matrix2 = matrix![[7,  8],
///                       [9,  10],
///                       [11, 12]];
/// 
/// &mut matrix1 + matrix2;
/// 
/// assert_eq!(matrix1, matrix![[8,  10],
///                             [12, 14],
///                             [16, 18]]);
/// ```
/// This is useful for addition of matrices that are scaled.
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let mut matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[7,  8],
///                       [9,  10],
///                       [11, 12]];
/// 
/// // The result of '&matrix2 * 2' is an owned Matrix,
/// // which is then added to '&mut matrix1'.
/// let matrix3 = &mut matrix1 + &matrix2 * 2;
/// 
/// assert_eq!(matrix1, matrix![[15, 18],
///                             [21, 24],
///                             [27, 30]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<'a, T> Add<Matrix<T>> for &'a mut Matrix<T>
where
    T: Add<Output = T> + Copy + 'a
{
    type Output = &'a mut Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx]
            }
        }
        self
    }
}


//
//
//          Borrowed Mutable/Borrowed Implementation
//
//
/// [Addition][std::ops::Add] implementation of '&mut Matrix<T> + &Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let mut matrix1 = matrix![[1, 2],
///                           [3, 4],
///                           [5, 6]];
/// 
/// let matrix2 = matrix![[7,  8],
///                           [9,  10],
///                           [11, 12]];
/// 
/// &mut matrix1 + &matrix2;
/// 
/// assert_eq!(matrix1, matrix![[8,  10],
///                             [12, 14],
///                             [16, 18]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<'a, T> Add<&Matrix<T>> for &'a mut Matrix<T>
where
    T: Add<Output = T> + Copy + 'a
{
    type Output = &'a mut Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx]
            }
        }
        self
    }
}