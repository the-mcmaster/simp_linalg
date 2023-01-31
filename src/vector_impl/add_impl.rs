use std::ops::Add;
use crate::vector_impl::Vector;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// The [addition][std::ops::Add] implementation for '&Vector + &Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// let vector3 = &vector1 + &vector2;
/// 
/// assert_eq!(vector3, vector![5, 7, 9])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<T> Add for &Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }
        
        let length = self.len();

        let mut params = Vec::with_capacity(length);
        for idx in 0..length {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}


//
//
//          Borrowed/Owned Implementation
//
//
/// The [addition][std::ops::Add] implementation for '&Vector + Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // Notice that 'vector2' is moved here
/// let vector3 = &vector1 + vector2;
/// 
/// assert_eq!(vector3, vector![5, 7, 9]);
/// ```
/// This is useful for addition of vectors that are scaled.
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // The result of '&vector2 * 2' is an owned Vector,
/// // which is then added to '&vector1'.
/// let vector3 = &vector1 + &vector2 * 2;
/// 
/// assert_eq!(vector3, vector![9, 12, 15])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<T> Add<Vector<T>> for &Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }
        
        let length = self.len();

        let mut params = Vec::with_capacity(length);
        for idx in 0..length {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}


//
//
//          Owned/Borrowed Implementation
//
//
/// The [addition][std::ops::Add] implementation for 'Vector + &Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // Notice that 'vector1' is moved here
/// let vector3 = vector1 + &vector2;
/// 
/// assert_eq!(vector3, vector![5, 7, 9]);
/// ```
/// This is useful for addition of vectors that are scaled.
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // The result of '&vector1 * 2' is an owned Vector,
/// // which is then added to '&vector2'.
/// let vector3 = &vector1 * 2 + &vector2;
/// 
/// assert_eq!(vector3, vector![6, 9, 12])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<T> Add<&Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }
        
        let length = self.len();

        let mut params = Vec::with_capacity(length);
        for idx in 0..length {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}


//
//
//          Owned/Owned Implementation
//
//
/// The [addition][std::ops::Add] implementation for 'Vector + Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // Notice that both vectors are moved here
/// let vector3 = vector1 + vector2;
/// 
/// assert_eq!(vector3, vector![5, 7, 9]);
/// ```
/// This is useful for addition of vectors that are scaled.
/// ```
/// use simp_linalg::vector_impl::Vector;
/// use simp_linalg::vector;
/// 
/// let vector1 = vector![1, 2, 3];
/// let vector2 = vector![4, 5, 6];
/// 
/// // The result of '&vector1 * 2' is an owned Vector,
/// // which is then added to another owned vector '&vector2 * 3'.
/// let vector3 = &vector1 * 2 + &vector2 * 3;
/// 
/// assert_eq!(vector3, vector![14, 19, 24])
/// ```
/// 
/// # Panic!
/// 
/// This function will panic if the vectors are not the same size.
impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }
        
        let length = self.len();

        let mut params = Vec::with_capacity(length);
        for idx in 0..length {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}