/// [MatrixLambda][crate::matrix_impl::traits::MatrixLambda] trait definition.
/// This allows for different behavior when the left-hand-side Matrix
/// is mutable versus immutable.
pub trait MatrixLambda<T> {
    type Output;

    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T;

    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize) -> T;
    
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize, &T) -> T;
}

/// [MatrixMap][crate::matrix_impl::traits::MatrixMap] trait definition.
/// This allows for different behavior when the left-hand-side Matrix
/// is mutable versus immutable.
pub trait MatrixMap<T> {
    type Other;
    type Output;

    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, usize, &T, &T) -> T;

    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T;

    /*
        For anyone following the source code, MatrixLambda
        has a function called 'lambda_index' while MatrixMap
        does not. This is on purpose, as it is pointless.

        Consider what the generic function definition
        would be for a map_index.
        
        This is what it would be:
            F: Fn(usize, usize) -> T
        
        This is already used for lambda_index.

        Additionally, the matrices are independent from
        the function definition, and no actual mapping
        will be done.
        
        Therefore, it is pointless to add the method 'map_index',
        as it does not make sense.
    */
}