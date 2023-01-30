#[macro_export]
macro_rules! vector {
    //no arguments case
    () => {
        Vector::from(vec![])
    };

    //repeat some elements some n times
    ($x:expr; $n:expr) => {
        Vector::from(vec![$x; $n])
    };
    
    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:expr),*) => {
        Vector::from(vec![$($x),*])
    };

    //match each comma-separated argument
    ($($x:expr,)*) => {
        Vector::from(vec![$($x),*])
    }
}

#[macro_export]
macro_rules! matrix {
    () => {
        Matrix::from(vec![])
    };

    ($x:tt; $n:expr) => {
        Matrix::from(vec![vec!$x; $n])
    };

    ($($x:tt),*) => {
        Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    };

    ($($x:tt,)*) => {
        Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    }
}