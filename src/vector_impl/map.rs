use crate::vector_impl::Vector;

impl<T> Vector<T> {
    /// Applies a function dependent on value
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::Vector;
    /// use simp_linalg::vector;
    /// 
    /// let vector_x = vector![1, 2, 3];
    /// let vector_y = vector![4, 5, 6];
    /// 
    /// let vector_z = vector_x.map(&vector_y, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(vector_z, vector![4, 10, 18])
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
    
    /*
        For anyone following the source code, lambda.rs
        has a function called 'lambda_index' while map.rs
        does not. This is on purpose, as it is pointless.

        Consider what the generic function definition
        would be for a map_index.
        
        This is what it would be:
            F: Fn(usize) -> T
        
        This is already used for lambda_index.

        Additionally, the vectors are independent from
        the function definition, and no actual mapping
        will be done.
        
        Therefore, it is pointless to add the method 'map_index',
        as it does not make sense.
    */
    
    /// Applies a function dependent on value and location 
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::Vector;
    /// use simp_linalg::vector;
    /// 
    /// let vector_x = vector![1, 2, 3];
    /// let vector_y = vector![4, 5, 6];
    /// 
    /// let vector_z = vector_x.map_enumerate(&vector_y, |idx, val1, val2| val1 * val2 + idx);
    /// 
    /// assert_eq!(vector_z, vector![4, 11, 20])
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two vectors are not identically
    /// sized.
    pub fn map_enumerate<F>(&self, other: &Vector<T>, funct: F) -> Vector<T>
    where
        F: Fn(usize, &T, &T) -> T
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        let lhs = self.list();
        let rhs = other.list();
        
        for idx in 0..self.len() {
            params.push(funct(idx, &lhs[idx], &rhs[idx]))
        }

        Vector::from(params)
    }
}