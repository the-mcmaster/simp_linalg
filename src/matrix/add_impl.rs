use std::ops::Add;
use crate::matrix::Matrix;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// [Addition][std::ops::Add] implementation of '&Matrix<T> + &Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// let matrix3 = &matrix1 + &matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![8,  10],
///                                       vec![12, 14],
///                                       vec![16, 18]]));
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        let mut params = vec![vec![]; self.rows];
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                params[row_idx].push(self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx])
            }
        }

        Matrix::from(params)
    }
}


//
//
//          Borrowed/Owned Implementation
//
//
/// [Addition][std::ops::Add] implementation of '&Matrix<T> + Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// let matrix3 = &matrix1 + matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![8,  10],
///                                       vec![12, 14],
///                                       vec![16, 18]]));
/// ```
/// This is useful for addition of matrices that are scaled.
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// // The result of '&matrix2 * 2' is an owned Matrix,
/// // which is then added to '&matrix1'.
/// let matrix3 = &matrix1 + &matrix2 * 2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![15, 18],
///                                       vec![21, 24],
///                                       vec![27, 30]]));
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<T> Add<Matrix<T>> for &Matrix<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        let mut params = vec![vec![]; self.rows];
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                params[row_idx].push(self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx])
            }
        }

        Matrix::from(params)
    }
}


//
//
//          Owned/Borrowed Implementation
//
//
/// [Addition][std::ops::Add] implementation of 'Matrix<T> + &Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// let matrix3 = matrix1 + &matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![8,  10],
///                                       vec![12, 14],
///                                       vec![16, 18]]));
/// ```
/// This is useful for addition of matrices that are scaled.
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// // The result of '&matrix1 * 2' is an owned Matrix,
/// // which is then added to '&matrix2'.
/// let matrix3 = &matrix1 * 2 + &matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![9,  12],
///                                       vec![15, 18],
///                                       vec![21, 24]]));
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<T> Add<&Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        let mut params = vec![vec![]; self.rows];
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                params[row_idx].push(self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx])
            }
        }

        Matrix::from(params)
    }
}


//
//
//          Owned/Owned Implementation
//
//
/// [Addition][std::ops::Add] implementation of 'Matrix<T> + Matrix<T>'.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// let matrix3 = matrix1 + matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![8,  10],
///                                       vec![12, 14],
///                                       vec![16, 18]]));
/// ```
/// This is useful for addition of matrices that are scaled.
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![7,  8],
///                                 vec![9,  10],
///                                 vec![11, 12]]);
/// 
/// // The result of '&matrix1 * 2' is an owned Matrix,
/// // which is then added to another owned matrix '&matrix2 * 3'.
/// let matrix3 = &matrix1 * 2 + &matrix2 * 3;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![23, 28],
///                                       vec![33, 38],
///                                       vec![43, 48]]));
/// ```
/// 
/// # Panic!
/// This function will panic if the matrices are not
/// equivalent in size.
impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Differently sized matrices cannot be added together.")
        }

        let mut params = vec![vec![]; self.rows];
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                params[row_idx].push(self.matrix[row_idx][col_idx] + rhs.matrix[row_idx][col_idx])
            }
        }

        Matrix::from(params)
    }
}