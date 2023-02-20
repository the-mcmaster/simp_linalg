use std::ops::{Mul, AddAssign};
use crate::matrix_impl::Matrix;

//
//
//          Borrowed Mutable/Borrowed Mutable Implementation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&mut Matrix<T> * &mut Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
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
/// let mut matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// &mut matrix1 * &mut matrix2;
/// 
/// assert_eq!(matrix1, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<'a, T> Mul for &'a mut Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default + 'a
{
    type Output = &'a mut Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        // ensures the matricies are compatible
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        //ensures there is enough room for resizing
        if self.cols < rhs.cols {
            for vec in &mut self.matrix {
                vec.resize_with(rhs.cols, || T::default())
            }
        }

        let empty_row: Vec<T> = Vec::with_capacity(rhs.cols);
        let mut new_row: Vec<T>;

        for out_row_index in 0..self.rows {
            new_row = empty_row.clone();

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

                new_row.push(param);
            }
            self.matrix[out_row_index] = new_row;
        }
        self.cols = rhs.cols;
        self
    }
}

//
//
//          Borrowed Mutable/Borrowed Implementation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&mut Matrix<T> * &Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
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
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// &mut matrix1 * &matrix2;
/// 
/// assert_eq!(matrix1, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<'a, T> Mul<&Matrix<T>> for &'a mut Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default + 'a
{
    type Output = &'a mut Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        // ensures the matricies are compatible
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        //ensures there is enough room for resizing
        if self.cols < rhs.cols {
            for vec in &mut self.matrix {
                vec.resize_with(rhs.cols, || T::default())
            }
        }

        let empty_row: Vec<T> = Vec::with_capacity(rhs.cols);
        let mut new_row: Vec<T>;

        for out_row_index in 0..self.rows {
            new_row = empty_row.clone();

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

                new_row.push(param);
            }

            self.matrix[out_row_index] = new_row;
        }

        self.cols = rhs.cols;
        self
    }
}

//
//
//          Borrowed Mutable/Owned Implementation
//
//
/// [Multiplication][std::ops::Mul] implementation of '&mut Matrix<T> * Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.into_vector()][crate::matrix_impl::Matrix] on the result of this multiplication.
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
/// let matrix2 = matrix![[8,  9,  10, 11],
///                       [12, 13, 14, 15]];
/// 
/// &mut matrix1 * matrix2;
/// 
/// // matrix2 is dropped
/// 
/// assert_eq!(matrix1, matrix![[32,  35,  38,  41],
///                             [72,  79,  86,  93],
///                             [112, 123, 134, 145]]);
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix_impl::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix_impl::Matrix].
impl<'a, T> Mul<Matrix<T>> for &'a mut Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default + 'a
{
    type Output = &'a mut Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        // ensures the matricies are compatible
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        //ensures there is enough room for resizing
        if self.cols < rhs.cols {
            for vec in &mut self.matrix {
                vec.resize_with(rhs.cols, || T::default())
            }
        }

        let empty_row: Vec<T> = Vec::with_capacity(rhs.cols);
        let mut new_row: Vec<T>;

        for out_row_index in 0..self.rows {
            new_row = empty_row.clone();

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

                new_row.push(param);
            }

            self.matrix[out_row_index] = new_row;
        }
        
        self.cols = rhs.cols;
        self
    }
}

//
//
//          MATRIX * SCALAR IMPLEMENTATION
//
//
//
//
//          Borrowed Mutable/Scalar Implementation
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
/// let mut matrix = matrix![[1, 2, 3],
///                          [4, 5, 6]];
/// 
/// &mut matrix * 3;
/// 
/// assert_eq!(matrix, matrix![[3,  6,  9],
///                            [12, 15, 18]])
/// ```
impl<'a, T> Mul<T> for &'a mut Matrix<T>
where
    T: Copy + Mul<Output = T> + 'a
{
    type Output = &'a mut Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        for row_idx in 0..self.rows {
            
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = rhs * self.matrix[row_idx][col_idx]
            }
        }
        self
    }
}