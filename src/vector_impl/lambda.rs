use crate::vector_impl::{Vector, traits::VectorLambda};

impl<'a, T> VectorLambda<T> for &'a Vector<T> {
    type Output = Vector<T>;

    /// Applies an anonymous function relative to value
    /// to each element of the vector and returns a vector 
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let vector_x = vector![1, 2, 3];
    /// 
    /// // squares each element in vector_x
    /// let vector_y = vector_x.lambda(|val| val * val);
    /// 
    /// assert_eq!(vector_y, vector![1, 4, 9])
    /// ```
    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T {
        let mut params = Vec::with_capacity(self.len());
        
        for item in self.list() {
            params.push(funct(item))
        }

        Vector::from(params)
    }

    /// Applies an anonymous function relative to location
    /// to each location of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let vector_x = vector![0, 0, 0];
    /// 
    /// // counts upwards starting from 1
    /// let vector_y = vector_x.lambda_index(|idx| idx + 1);
    /// 
    /// assert_eq!(vector_y, vector![1, 2, 3])
    /// ```
    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize) -> T {
        let mut params = Vec::with_capacity(self.len());
        
        for item in 0..self.len() {
            params.push(funct(item))
        }

        Vector::from(params)
    }

    /// Applies an anonymous function relative to location and value
    /// to each element of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let vector_x = vector![1, 2, 3];
    /// 
    /// let vector_y = vector_x.lambda_enumerate(|idx, val| idx * val);
    /// 
    /// assert_eq!(vector_y, vector![0, 2, 6])
    /// ```
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, &T) -> T {
        let mut params = Vec::with_capacity(self.len());

        for (idx, item) in self.list.iter().enumerate() {
            params.push(funct(idx, item))
        }

        Vector::from(params)
    }
}