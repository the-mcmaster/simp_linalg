use std::ops::{Mul, AddAssign};

use crate::vector::Vector;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>
{
    rows : usize,
    cols : usize,
    matrix : Vec<Vec<T>>
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Copy
{
    /// Converts a single dimentional Matrix into a Vector, consuming the Matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector::Vector;
    /// use simp_linalg::matrix::Matrix;
    /// 
    /// let row_matrix = Matrix::from(vec![vec![1, 2, 4]]);
    /// let col_matrix = Matrix::from(vec![vec![1],
    ///                                    vec![2],
    ///                                    vec![3]]);
    /// assert_eq!(row_matrix.to_vector(), Vector::from(vec![1, 2, 4]));
    /// assert_eq!(col_matrix.to_vector(), Vector::from(vec![1, 2, 3]));
    /// ```
    /// # Panic!
    /// This function will panic! if self.rows or self.cols are not equal to 1.
    pub fn to_vector(self) -> Vector<T> {
        if self.rows == 1 {
            let mut params = vec![];
            for param in &self.matrix[0] {
                params.push(*param)
            }
            return Vector::new(params.len(), params)
        }
        else if self.cols == 1 {
            let mut params: Vec<T> = vec![];
            for row in self.matrix {
                params.push(row[0])
            }
            return Vector::new(params.len(), params);
        } else {
            panic!("Cannot convert matrix because neither rows nor columns are 0")
        }
    }

    /// Construct a new Matrix<T> 
    pub fn new(rows: usize, cols: usize, matrix: Vec<Vec<T>>) -> Self {
        Matrix {rows: rows, cols: cols, matrix: matrix}
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(vecs: Vec<Vec<T>>) -> Self {
        let rows = vecs.len();
        let cols = vecs[0].len();
        for row in 1..rows {
            if vecs[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }
        Matrix {
            rows: rows,
            cols: cols,
            matrix : vecs
        }
    }
}

/// [Multiply][std::ops::Mul] implmentation of 'Matrix<T> * Vector<T>'.
///
/// With valid inputs, this function will always produce a [Vector<T>][crate::vector::Vector].
/// If this multiplicaiton needs to be a matrix, it can be converted by
/// using the method [.to_row_matrix()][crate::vector::Vector] or [.to_col_matrix()][crate::vector::Vector] on the result of this multiplication
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
/// let vector2 = matrix * vector1;
/// 
/// assert_eq!(vector2, Vector::from(vec![11, 17]))
/// ```
/// 
/// # Panic!
/// This function will panic if the number of columns
/// in [Matrix][crate::matrix::Matrix] is not the same as the amount of elements
/// in [Vector][crate::vector::Vector].
/// 
/// # Safety
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if rhs.size() != self.cols {
            panic!("Matrix must have same number of columns as the vector has columns")
        };
        let mut params = vec![];
        let vector_ptr = rhs.list().as_ptr();

        
        for row in self.matrix {
            let mut param = T::default();

            let row_ptr = row.as_ptr();
            
            for i in 0..self.cols {
                unsafe {
                    param += *row_ptr.add(i) * *vector_ptr.add(i)
                }
            }

            params.push(param)
        }
        
        Vector::new(params.len(), params)
    }
}

/// [Multiply][std::ops::Mul] implementation of 'Matrix<T> * Matrix<T>'.
/// 
/// With valid inputs, this function will always produce a [Matrix<T>][crate::matrix::Matrix].
/// If this matrix multiplicaiton has dimentions of a vector, it can be converted by
/// using the method [.to_vector()][crate::matrix::Matrix] on the result of this multiplication.
/// 
/// # Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![
///                                 vec![1, 2],
///                                 vec![3, 4],
///                                 vec![5, 6]]);
/// let matrix2 = Matrix::from(vec![
///                                 vec![8,  9,  10, 11],
///                                 vec![12, 13, 14, 15]]);
/// 
/// let matrix3 = matrix1 * matrix2;
/// 
/// assert_eq!(matrix3, Matrix::from(vec![vec![32,  35,  38,  41],
///                                       vec![72,  79,  86,  93],
///                                       vec![112, 123, 134, 145]]));
/// ```
/// 
/// # Panic!
/// This funciton will panic if the number of columns
/// in the left hand side [matrix][crate::matrix::Matrix] is not the same as the number of rows
/// in the right hand side [matrix][crate::matrix::Matrix].
impl<T> Mul for Matrix<T>
where
    T: Copy + AddAssign + Mul<Output = T> + Default
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("Left matrix must have same number of columns as the number of rows in right matrix to multiply")
        }

        let mut params: Vec<Vec<T>> = vec![vec![]; self.rows];

        for out_row_index in 0..self.rows {
            for out_col_index in 0..rhs.cols {
                let mut param = T::default();
                
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
        Matrix{
            rows : self.rows,
            cols : rhs.cols,
            matrix : params
        }
    }
}