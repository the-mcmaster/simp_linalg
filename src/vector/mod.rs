mod mul_impl;
mod add_impl;

use crate::matrix::Matrix;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// The Vector type.
pub struct Vector<T>
{
    list : Vec<T>
}

#[allow(dead_code)]
impl<T> Vector<T>
{
    /// Converts a Vector into a column Matrix, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1],
    ///                                          vec![2],
    ///                                          vec![3]]));
    /// 
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![vec!["woah"], vec!["what"]]);
    /// 
    /// let col_matrix = vector.into_col_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![vec!["woah"]],
    ///                                          vec![vec!["what"]]]));
    /// ```
    pub fn into_col_matrix(self) -> Matrix<T> {
        let mut matrix = Vec::with_capacity(self.len());

        for param in self.list {
            matrix.push(vec![param])
        }

        Matrix::from(matrix)
    }

    /// Converts a Vector<T> into a row Matrix<T>, consuming the Vector.
    /// 
    /// # Examples
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![1, 2, 3]);
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![1, 2, 3]]));
    /// ```
    /// The internal type of the vector does not need to implement traits
    /// like [Copy] or [Sized] to convert.
    /// ```
    /// use simp_linalg::prelude::*;
    /// 
    /// let vector = Vector::from(vec![vec!["woah"], vec!["what"]]);
    /// 
    /// let col_matrix = vector.into_row_matrix();
    /// 
    /// assert_eq!(col_matrix, Matrix::from(vec![vec![vec!["woah"], vec!["what"]]]));
    /// ```
    pub fn into_row_matrix(self) -> Matrix<T> {
        Matrix::from(vec![self.list])
    }

    /// Applies an anonymous function to each element of the vector
    /// and return a vector with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector_x = Vector::from(vec![1, 2, 3]);
    /// 
    /// // squares each element in vector_x
    /// let vector_y = vector_x.lambda(|val| val * val);
    /// 
    /// assert_eq!(vector_y, Vector::from(vec![1, 4, 9]))
    /// ```
    pub fn lambda<F>(&self, funct: F) -> Vector<T>
    where
        F: Fn(&T) -> T
    {
        let mut params = Vec::with_capacity(self.len());
        
        for item in self.list() {
            params.push(funct(item))
        }

        Vector::from(params)
    }

    /// Applies a function to each corresponding 
    /// elements between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector_x = Vector::from(vec![1, 2, 3]);
    /// let vector_y = Vector::from(vec![4, 5, 6]);
    /// 
    /// let vector_z = vector_x.map(&vector_y, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(vector_z, Vector::from(vec![4, 10, 18]))
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two vectors are not identically
    /// sized.
    pub fn map<F>(&self, other: &Vector<T>, funct: F) -> Vector<T>
    where
        F: Fn(&T, &T) -> T
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        let lhs = self.list();
        let rhs = other.list();
        
        for idx in 0..self.len() {
            params.push(funct(&lhs[idx], &rhs[idx]))
        }

        Vector::from(params)
    }

    pub fn lambda_index<F>(&self, funct : F) -> Vector<T>
    where
        F: Fn(usize) -> T
    {
        let mut params = Vec::with_capacity(self.len());
        
        for item in 0..self.len() {
            params.push(funct(item))
        }

        Vector::from(params)
    }

    pub fn lambda_enumerate<F>(&self, funct : F) -> Vector<T>
    where
        F: Fn(usize, &T) -> T
    {
        let mut params = Vec::with_capacity(self.len());

        for (idx, item) in self.list.iter().enumerate() {
            params.push(funct(idx, item))
        }

        Vector::from(params)
    }

    /// Returns the length of the Vector<T>.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Borrows the list of elements in the Vector<T>.
    pub fn list(&self) -> &Vec<T> {
        &self.list
    }
}

/// Converts a [Vec][std::vec::Vec] to a Vector.
/// 
/// This function will never fail since a Vector can
/// be constructed with any valid [std::vec::Vec]. However,
/// the methods available are based upon the Vector's internal
/// type's traits, such as multiplcation and addition.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3, 4]);
/// let vector2 = Vector::from(vec![1.7, -0.03]);
/// 
/// // Vectors can be constructed with any std::vec::Vec.
/// let vector3 = Vector::from(vec!["Hello", "I'm in the vector!"]);
/// ```
impl<T> From<Vec<T>> for Vector<T>
{
    fn from(vec: Vec<T>) -> Self {
        Vector {
            list : vec
        }
    }
}