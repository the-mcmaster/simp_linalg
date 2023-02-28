use crate::vector_impl::{Vector, traits::VectorMap};


impl<'a, T> VectorMap<'a, T> for Vector<T>
where
    T: 'a
{
    type Other = &'a Vector<T>;

    type Output = Vector<T>;

    /// Applies a function dependent on value
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
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
    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());
        
        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }

    /// Applies a function dependent on value and location 
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
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
    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, &T, &T) -> T {
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

impl<'a, T> VectorMap<'a, T> for &'a Vector<T> {
    type Other = &'a Vector<T>;

    type Output = Vector<T>;

    /// Applies a function dependent on value
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
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
    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());
        
        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }

    /// Applies a function dependent on value and location 
    /// to each corresponding element between the two vectors. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
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
    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, &T, &T) -> T {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());
        
        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }
}



/// Unlike the &Vector implementation of VectorMap,
/// this implementation returns the left-hand-side 
/// borrowed mutable Vector with changes applied.
impl<'a, T> VectorMap<'a, T> for &'a mut Vector<T> {
    type Other = &'a Vector<T>;

    type Output = &'a mut Vector<T>;

    /// Applies a function dependent on value
    /// to each corresponding element between the two vectors
    /// and applies the transformation onto the left-hand-side
    /// vector. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let mut vector_x = vector![1, 2, 3];
    /// let vector_y = vector![4, 5, 6];
    /// 
    /// (&mut vector_x).map(&vector_y, |val1, val2| val1 * val2);
    /// 
    /// assert_eq!(vector_x, vector![4, 10, 18])
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two vectors are not identically
    /// sized.
    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }
        
        for idx in 0..other.len() {
            self.list[idx] = funct(&self.list[idx], &other.list[idx])
        }

        self
    }

    /// Applies a function dependent on value and location 
    /// to each corresponding element between the two vectors
    /// and applies the transformation onto the left-hand-side
    /// vector. 
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let mut vector_x = vector![1, 2, 3];
    /// let vector_y = vector![4, 5, 6];
    /// 
    /// (&mut vector_x).map_enumerate(&vector_y, |idx, val1, val2| val1 * val2 + idx);
    /// 
    /// assert_eq!(vector_x, vector![4, 11, 20])
    /// ```
    /// 
    /// # Panic!
    /// This function will panic if the two vectors are not identically
    /// sized.
    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, &T, &T) -> T {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }
        
        for idx in 0..other.len() {
            self.list[idx] = funct(idx, &self.list[idx], &other.list[idx])
        }

        self
    }
}
