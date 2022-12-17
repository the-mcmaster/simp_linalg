use std::ops::{Mul, AddAssign};
use crate::prelude::*;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// [Multiply][std::ops::Mul] implementation of '&Matrix<T> * &Matrix<T>'.
/// 
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.to_vector()][crate::matrix::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// 
/// let matrix2 = Matrix::from(vec![vec![8,  9,  10, 11],
///                                 vec![12, 13, 14, 15]]);
/// 
/// let matrix3 = &matrix1 * &matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![32,  35,  38,  41],
///                                       vec![72,  79,  86,  93],
///                                       vec![112, 123, 134, 145]]));
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in the left hand side [matrix][crate::matrix::Matrix] is not equal to the number of rows
/// in the right hand side [matrix][crate::matrix::Matrix].
impl<T> Mul for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![vec![]; self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;
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
impl<T> Mul<Matrix<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![vec![]; self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;
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
impl<T> Mul<&Matrix<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![vec![]; self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;
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
impl<T> Mul for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The left matrix row count is not equal to the right matrix column count.")
        }

        let mut params: Vec<Vec<T>> = vec![vec![]; self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
                // SAFTEY: The previous test guarentees that valid memory will be pointed to.
                for index in 0..self.cols {
                    //get the data from the lhs's row
                    let lhs_row_ptr = self.matrix[out_row_index].as_ptr();
                    let lhs_value: T;
                    unsafe {
                        lhs_value = *lhs_row_ptr.add(index)
                    }

                    //get the data from the rhs's col
                    let rhs_row_ptr = rhs.matrix[index].as_ptr();
                    let rhs_value: T;
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
//          Borrowed/Borrowed Vector Implmentation
//
//
/// [Multiply][std::ops::Mul] implmentation of '&Matrix<T> * &Vector<T>'.
///
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.to_row_matrix()][crate::vector::Vector] or [.to_col_matrix()][crate::vector::Vector]
/// on the result of this multiplication.
///
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2]);
/// let matrix = Matrix::from(vec![vec![3, 4], 
///                                vec![5, 6]]);
/// 
/// let vector2 = &matrix * &vector1;
/// 
/// assert_eq!(vector2, Vector::from(vec![11, 17]))
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix::Matrix] are not equal to the amount of elements
/// in [Vector][crate::vector::Vector].
impl<T> Mul<&Vector<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = vec![];
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
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
impl<T> Mul<Vector<T>> for &Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = vec![];
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
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
impl<T> Mul<&Vector<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = vec![];
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
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
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if rhs.len() != self.cols {
            panic!("The matrix column count must be equal to the vector parameter count.")
        };
        let mut params = vec![];
        let vector_ptr = rhs.list().as_ptr();

        // SAFTEY: The previous test guarentees that valid memory will be pointed to.
        for row in &self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
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
//          Borrowed/Scalar Implementation
//
//
impl<T> Mul<T> for &Matrix<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..self.rows {
            let mut params_row = vec![];
            
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
impl<T> Mul<T> for Matrix<T>
where
    T: Copy + Mul<Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..self.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..self.cols {
                params_row.push(rhs * self.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}