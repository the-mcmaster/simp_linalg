mod mul_impl;
mod add_impl;

use crate::vector::Vector;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// The Matrix type.
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
    /// 
    /// assert_eq!(row_matrix.into_vector(), Vector::from(vec![1, 2, 4]));
    /// assert_eq!(col_matrix.into_vector(), Vector::from(vec![1, 2, 3]));
    /// ```
    /// # Panic!
    /// This function will panic if neither rows nor columns are equal to 1.
    /// ## Example
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let row_matrix = Matrix::from(vec![vec![1,2]]);
    /// let col_matrix = Matrix::from(vec![vec![1],
    ///                                    vec![2]]);
    /// let both_matrix = Matrix::from(vec![vec![1,2],
    ///                                     vec![3,4]]);
    /// 
    /// let vector1 = row_matrix.into_vector();
    /// let vector2 = col_matrix.into_vector();
    /// // Panics vvv
    /// // let vector3 = both_matrix.into_vector();
    /// ```
    pub fn into_vector(self) -> Vector<T> {
        if self.rows == 1 {
            let mut params = Vec::with_capacity(self.cols);

            for col in &self.matrix[0] {
                params.push(*col)
            }

            return Vector::from(params)
        }
        
        else if self.cols == 1 {
            let mut params: Vec<T> = Vec::with_capacity(self.rows);

            for row in self.matrix {
                params.push(row[0])
            }

            return Vector::from(params);
        }
        
        else {
            panic!("Cannot convert matrix because neither rows nor columns are 1")
        }
    }
}

impl<T> Matrix<T> {
    /// Applies a function to each individual element
    /// in the matrix.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// 
    /// let matrix1 = Matrix::from(vec![vec![1, 2],
    ///                                 vec![3, 4]]);
    /// 
    /// let matrix2 = matrix1.lambda(|val| val * val);
    /// 
    /// assert_eq!(matrix2, Matrix::from(vec![vec![1, 4],
    ///                                       vec![9, 16]]));
    /// ```
    pub fn lambda<F>(&self, funct: F) -> Matrix<T>
    where
        F: Fn(&T) -> T
    {
        let mut rows = Vec::with_capacity(self.rows);
        
        
        for row in &self.matrix {
            let mut cols = Vec::with_capacity(self.cols);
            
            for col in row {
                cols.push(funct(col))
            }
            
            rows.push(cols)
        }

        Matrix::from(rows)
    }

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

    /// Returns the number of rows of the Matrix<T>.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns of the Matrix<T>.
    pub fn cols(&self) -> usize {
        self.cols
    }
}

/// # Panic!
/// 
/// This function will panic if there exists a differently sized internal [vec][std::vec::Vec].
/// 
/// ## Example
/// ```
/// use simp_linalg::matrix::Matrix;
/// 
/// let matrix1 = Matrix::from(vec![vec![1, 0],
///                                 vec![1, 2]]);
/// //This will panic.
/// //let matrix2 = Matrix::from(vec![vec![1],
/// //                                vec![1, 2]]);
/// ```
impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(params: Vec<Vec<T>>) -> Self {

        let rows = params.len();
        let cols = params[0].len();

        for row in 1..rows {
            if params[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }

        Matrix {
            rows : params.len(),
            cols : params[0].len(),
            matrix : params
        }
    }
}