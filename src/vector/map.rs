use crate::vector::Vector;

impl<T> Vector<T> {
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
}