use std::ops::Add;
use crate::vector::Vector;

//
//
//          Borrowed/Borrowed Implementation
//
//
/// The [Addition][std::ops::Add] implementation for '&Vector + &Vector'.
/// 
/// # Example
/// ```
/// use simp_linalg::vector::Vector;
/// 
/// let vector1 = Vector::from(vec![1, 2, 3]);
/// let vector2 = Vector::from(vec![4, 5, 6]);
/// 
/// let vector3 = &vector1 + &vector2;
/// 
/// assert_eq!(vector3, Vector::from(vec![5, 7, 9]))
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

        let mut params = vec![];
        for idx in 0..self.len() {
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
impl<T> Add<Vector<T>> for &Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        let mut params = vec![];
        for idx in 0..self.len() {
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
impl<T> Add<&Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: &Vector<T>) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        let mut params = vec![];
        for idx in 0..self.len() {
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
impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            panic!("Vectors with different sizes cannot be added together.")
        }

        let mut params = vec![];
        for idx in 0..self.len() {
            params.push(self.list[idx] + rhs.list[idx])
        }

        Vector::from(params)
    }
}