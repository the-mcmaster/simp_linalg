use crate::vector::Vector;

impl<T> Vector<T> {
    /// Applies an anonymous function relative to value
    /// to each element of the vector and returns a vector 
    /// with the corresponding results.
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
    
    /// Applies an anonymous function relative to location
    /// to each location of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector_x = Vector::from(vec![0, 0, 0]);
    /// 
    /// // counts upwards starting from 1
    /// let vector_y = vector_x.lambda_index(|idx| idx + 1);
    /// 
    /// assert_eq!(vector_y, Vector::from(vec![1, 2, 3]))
    /// ```
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

    /// Applies an anonymous function relative to location and value
    /// to each element of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector::Vector;
    /// 
    /// let vector_x = Vector::from(vec![1, 2, 3]);
    /// 
    /// let vector_y = vector_x.lambda_enumerate(|idx, val| idx * val);
    /// 
    /// assert_eq!(vector_y, Vector::from(vec![0, 2, 6]))
    /// ```
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
}