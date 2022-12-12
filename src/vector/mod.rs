use crate::matrix::Matrix;

/// The Vector type
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T>
{
    size : usize,
    list : Vec<T>
}

#[allow(dead_code)]
impl<T> Vector<T>
where
    T: Copy
{
    /// Converts a Vector<T> to a column Matrix<T>, consuming the Vector.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.to_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![
    ///                                          vec![1],
    ///                                          vec![2],
    ///                                          vec![3]]));
    /// ```
    pub fn to_col_matrix(self) -> Matrix<T> {
        let mut matrix = vec![];
        for param in self.list {
            matrix.push(vec![param])
        }
        Matrix::new(matrix.len(), 1, matrix)
    }

    /// Converts a Vector<T> to a row Matrix<T>, consuming the Vector.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::matrix::Matrix;
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.to_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1, 2, 3]]));
    /// ```
    pub fn to_row_matrix(self) -> Matrix<T> {
        Matrix::new(1, self.list.len(), vec![self.list])
    }

    /// Returns the size of the Vector
    pub fn size(&self) -> usize {
        self.size
    }

    /// Borrows the list of elements in the Vector<T>
    pub fn list(&self) -> &Vec<T> {
        &self.list
    }

    /// Construct a new Vector<T>
    pub fn new(size: usize, list: Vec<T>) -> Self {
        Vector { size: size, list: list }
    }
}

/// Converts a Vec<T> to a Vector<T>
/// 
/// This function will never fail.
impl<T> From<Vec<T>> for Vector<T>
{
    fn from(vec: Vec<T>) -> Self {
        Vector {
            size : vec.len(),
            list : vec
        }
    }
}