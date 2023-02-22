use crate::vector_impl::{Vector, traits::VectorLambda};

/// Unlike the &Vector implementation of VectorLambda,
/// this implementation returns the left-hand-side 
/// borrowed mutable Vector with changes applied.
impl<'a, T> VectorLambda<T> for &'a mut Vector<T> {
    type Output = &'a mut Vector<T>;

    /// Applies an anonymous function relative to value
    /// to each element of the vector and returns a vector 
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let mut vector_x = vector![1, 2, 3];
    /// 
    /// // squares each element in vector_x
    /// (&mut vector_x).lambda(|val| val * val);
    /// 
    /// assert_eq!(vector_x, vector![1, 4, 9])
    /// ```
    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T {
        let list_ptr = self.list.as_mut_ptr();
        
        // SAFTEY:
        // - The indexes added to this pointer is bounded 
        // by the length of the vec it points to. Therefore,
        // will stay within the initialized length of the vec.
        //
        // MUTABILITY SAFTEY:
        // - The value at the pointer is read first,
        // does a calculation to determine the next result,
        // then overwrites that location. 
        // - An already overwriten location is not read again.
        for (idx, item) in self.list().iter().enumerate() {
            unsafe {
                *list_ptr.add(idx) = funct(item)
            }
        }

        self
    }

    /// Applies an anonymous function relative to location
    /// to each location of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let mut vector_x = vector![0, 0, 0];
    /// 
    /// // counts upwards starting from 1
    /// (&mut vector_x).lambda_index(|idx| idx + 1);
    /// 
    /// assert_eq!(vector_x, vector![1, 2, 3])
    /// ```
    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize) -> T {
        
        for idx in 0..self.len() {
            self.list[idx] = funct(idx)
        }

        self
    }

    /// Applies an anonymous function relative to location and value
    /// to each element of the vector and returns a vector
    /// with the corresponding results.
    /// 
    /// # Example
    /// ```
    /// use simp_linalg::vector_impl::prelude::*;
    /// 
    /// let mut vector_x = vector![1, 2, 3];
    /// 
    /// (&mut vector_x).lambda_enumerate(|idx, val| idx * val);
    /// 
    /// assert_eq!(vector_x, vector![0, 2, 6])
    /// ```
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, &T) -> T {
        let list_ptr = self.list.as_mut_ptr();
        
        for (idx, item) in self.list.as_mut_slice().iter().enumerate() {
            // SAFTEY:
            // - The indexes added to this pointer is bounded 
            // by the length of the vec it points to. Therefore,
            // will stay within the initialized length of the vec.
            //
            // MUTABILITY SAFTEY:
            // - The value at the pointer is read first,
            // does a calculation to determine the next result,
            // then overwrites that location. 
            // - An already overwriten location is not read again.
            unsafe {
                *list_ptr.add(idx) = funct(idx, item)
            }
        }

        self
    }
}