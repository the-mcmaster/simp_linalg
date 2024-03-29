use std::ops::{Mul, AddAssign};
use crate::prelude::*;

//
//
//          MATRIX * MATRIX IMPLEMENTATIONS
//
//

//
//
//          Borrowed/Borrowed Implementation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&Matrix<T> * &Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// let matrix3 = &matrix1 * &matrix2;
/// 
/// assert_eq!(matrix3, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<T> Mul for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(self.cols); self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;

                    // SAFTEY: self.cols is guarenteed to be one-greater-than
                    //         the last index for the lhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;

                    // SAFETY: rhs.rows is guarenteed to be one-greater-than
                    //         the last index in the rhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        rhs_value = *rhs_row_ptr.add(out_col_index)
                    }

                    param += lhs_value * rhs_value
                }
                params[out_row_index].push(param)
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
/// [Multiplication][std::ops::Mul] implementation of '&Matrix<T> * Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// let matrix3 = &matrix1 * matrix2;
/// 
/// assert_eq!(matrix3, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// This is useful for scalar multiplication.
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// // Note: parenthesis are just visual
/// //       indicators in this context
/// let matrix3 = &matrix1 * (&matrix2 * 3);
/// 
/// assert_eq!(matrix3, matrix![[96,  105, 114, 123],
///                             [216, 237, 258, 279],
///                             [336, 369, 402, 435]]);
/// ```
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<T> Mul<Matrix<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(self.cols); self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;

                    // SAFTEY: self.cols is guarenteed to be one-greater-than
                    //         the last index for the lhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;

                    // SAFETY: rhs.rows is guarenteed to be one-greater-than
                    //         the last index in the rhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        rhs_value = *rhs_row_ptr.add(out_col_index)
                    }

                    param += lhs_value * rhs_value
                }
                params[out_row_index].push(param)
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
/// [Multiplication][std::ops::Mul] implementation of 'Matrix<T> * &Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// let matrix3 = matrix1 * &matrix2;
/// 
/// assert_eq!(matrix3, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// This is useful for scalar multiplication.
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// // Note: parenthesis are just visual
/// //       indicators in this context
/// let matrix3 = (&matrix1 * 3) * &matrix2;
/// 
/// assert_eq!(matrix3, matrix![[96,  105, 114, 123],
///                             [216, 237, 258, 279],
///                             [336, 369, 402, 435]]);
/// ```
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<T> Mul<&Matrix<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(self.cols); self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;

                    // SAFTEY: self.cols is guarenteed to be one-greater-than
                    //         the last index for the lhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;

                    // SAFETY: rhs.rows is guarenteed to be one-greater-than
                    //         the last index in the rhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        rhs_value = *rhs_row_ptr.add(out_col_index)
                    }

                    param += lhs_value * rhs_value
                }
                params[out_row_index].push(param)
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
/// [Multiplication][std::ops::Mul] implementation of 'Matrix<T> * Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// let matrix3 = matrix1 * matrix2;
/// 
/// assert_eq!(matrix3, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// This is useful for scalar multiplication.
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix1 = matrix![[1, 2],
///                       [3, 4],
///                       [5, 6]];
/// 
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// // Note: parenthesis are just visual
/// //       indicators in this context
/// let matrix3 = (&matrix1 * 3) * (&matrix2 * 3);
/// 
/// assert_eq!(matrix3, matrix![[288,  315,  342,  369],
///                             [648,  711,  774,  837],
///                             [1008, 1107, 1206, 1305]]);
/// ```
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<T> Mul for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(self.cols); self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;

                    // SAFTEY: self.cols is guarenteed to be one-greater-than
                    //         the last index for the lhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;

                    // SAFETY: rhs.rows is guarenteed to be one-greater-than
                    //         the last index in the rhs_row_ptr and therefore
                    //         will never go out of range.
                    unsafe {
                        rhs_value = *rhs_row_ptr.add(out_col_index)
                    }

                    param += lhs_value * rhs_value
                }
                params[out_row_index].push(param)
            }
        }
        Matrix::from(params)
    }
}

//-------------------------------------------------

//
//
//          MATRIX * VECTOR IMPLEMENTATIONS
//
//

//
//
//          Borrowed/Borrowed Vector Implmentation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&Matrix<T> * &Vector<T>'.
///
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.into_row_matrix()][crate::vector_impl::Vector] or
/// [.into_col_matrix()][crate::vector_impl::Vector] on the result of this multiplication.
///
/// # Example
/// ```
/// use simp_linalg::prelude::*;
/// 
/// let vector1 = vector![1, 2];
/// let matrix = matrix![[3, 4], 
///                      [5, 6]];
/// 
/// let vector2 = &matrix * &vector1;
/// 
/// assert_eq!(vector2, vector![11, 17])
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix_impl::Matrix] are not equal to the amount of elements
/// in [Vector][crate::vector_impl::Vector].
impl<T> Mul<&Vector<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = Vec::with_capacity(self.rows);
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
                // SAFETY: rhs.len() == self.cols and therefore
                //         it will always be in range
                unsafe {
                    param += *row_ptr.add(i) * *vector_ptr.add(i)
                }
            }

            params.push(param)
        }
        
        Vector::from(params)
    }
}

//
//
//          Borrowed/Owned Vector Implmentation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&Matrix<T> * Vector<T>'.
/// 
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.into_row_matrix()][crate::vector_impl::Vector] or
/// [.into_col_matrix()][crate::vector_impl::Vector] on the result of this multiplication.
///
/// # Example
/// ```
/// use simp_linalg::prelude::*;
/// 
/// let vector1 = vector![1, 2];
/// let matrix = matrix![[3, 4], 
///                      [5, 6]];
/// 
/// let vector2 = &matrix * vector1;
/// 
/// assert_eq!(vector2, vector![11, 17])
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix_impl::Matrix] are not equal to the amount of elements
/// in [Vector][crate::vector_impl::Vector].
impl<T> Mul<Vector<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = Vec::with_capacity(self.rows);
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
                // SAFETY: rhs.len() == self.cols and therefore
                //         it will always be in range
                unsafe {
                    param += *row_ptr.add(i) * *vector_ptr.add(i)
                }
            }

            params.push(param)
        }
        
        Vector::from(params)
    }
}

//
//
//          Owned/Borrowed Vector Implmentation
//
//
/// [Multiplication][std::ops::Mul] implementation of 'Matrix<T> * &Vector<T>'.
///
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.into_row_matrix()][crate::vector_impl::Vector] or
/// [.into_col_matrix()][crate::vector_impl::Vector] on the result of this multiplication.
///
/// # Example
/// ```
/// use simp_linalg::prelude::*;
/// 
/// let vector1 = vector![1, 2];
/// let matrix = matrix![[3, 4], 
///                      [5, 6]];
/// 
/// let vector2 = matrix * &vector1;
/// 
/// assert_eq!(vector2, vector![11, 17])
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix_impl::Matrix] are not equal to the amount of elements
/// in [Vector][crate::vector_impl::Vector].
impl<T> Mul<&Vector<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = Vec::with_capacity(self.rows);
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
                // SAFETY: rhs.len() == self.cols and therefore
                //         it will always be in range
                unsafe {
                    param += *row_ptr.add(i) * *vector_ptr.add(i)
                }
            }

            params.push(param)
        }
        
        Vector::from(params)
    }
}

//
//
//          Owned/Owned Vector Implmentation
//
//
/// [Multiplication][std::ops::Mul] implementation of 'Matrix<T> * Vector<T>'.
///
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.into_row_matrix()][crate::vector_impl::Vector] or
/// [.into_col_matrix()][crate::vector_impl::Vector] on the result of this multiplication.
///
/// # Example
/// ```
/// use simp_linalg::prelude::*;
/// 
/// let vector1 = vector![1, 2];
/// let matrix = matrix![[3, 4], 
///                      [5, 6]];
/// 
/// let vector2 = matrix * vector1;
/// 
/// assert_eq!(vector2, vector![11, 17])
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix_impl::Matrix] are not equal to the amount of elements
/// in [Vector][crate::vector_impl::Vector].
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = Vec::with_capacity(self.rows);
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
                // SAFETY: rhs.len() == self.cols and therefore
                //         it will always be in range
                unsafe {
                    param += *row_ptr.add(i) * *vector_ptr.add(i)
                }
            }

            params.push(param)
        }
        
        Vector::from(params)
    }
}


//
//
//          MATRIX * SCALAR IMPLEMENTATIONS
//
//

//
//
//          Borrowed/Scalar Implementation
//
//
/// The [multiplication][std::ops::Mul] implementation for '&Matrix * T'.
/// 
/// In contrast to common mathematical notation,
/// the scalar must be on the right of the matrix.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix = matrix![[1, 2, 3],
///                      [4, 5, 6]];
/// 
/// assert_eq!(&matrix * 3, matrix![[3,  6,  9],
///                                 [12, 15, 18]])
/// ```
impl<T> Mul<T> for &Matrix<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut params_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                params_row.push(rhs * self.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

//
//
//          Owned/Scalar Implementation
//
//
/// The [multiplication][std::ops::Mul] implementation for 'Matrix * T'.
/// 
/// In contrast to common mathematical notation,
/// the scalar must be on the right of the matrix.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix_impl::Matrix;
/// use simp_linalg::matrix;
/// 
/// let matrix = matrix![[1, 2, 3],
///                      [4, 5, 6]];
/// 
/// assert_eq!(matrix * 3, matrix![[3,  6,  9],
///                                [12, 15, 18]])
/// ```
impl<T> Mul<T> for Matrix<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut params_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                params_row.push(rhs * self.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }
        
        Matrix::from(params)
    }
}