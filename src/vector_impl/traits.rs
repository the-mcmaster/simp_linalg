pub trait VectorLambda<T> {
    type Output;

    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T;

    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize) -> T;
    
    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, &T) -> T;
}

pub trait VectorMap<'a, T> {
    type Other;
    type Output;

    fn map<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(&T, &T) -> T;
    
    fn map_enumerate<F>(self, other: Self::Other, funct: F) -> Self::Output
    where
        F: Fn(usize, &T, &T) -> T;
    
    /*
        For anyone following the source code, VectorLambda
        has a function called 'lambda_index' while VectorMap
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
}