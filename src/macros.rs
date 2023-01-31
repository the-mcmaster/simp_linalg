#[macro_export]
macro_rules! vector {
    //no arguments case
    () => {
        simp_linalg::vector_impl::Vector::from(vec![])
    };

    //repeat some elements some n times
    ($x:expr; $n:expr) => {
        simp_linalg::vector_impl::Vector::from(vec![$x; $n])
    };
    
    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:expr),*) => {
        simp_linalg::vector_impl::Vector::from(vec![$($x),*])
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:expr,)*) => {
        simp_linalg::vector_impl::Vector::from(vec![$($x),*])
    }
}

#[macro_export]
macro_rules! matrix {
    //no arguments case
    () => {
        simp_linalg::matrix_impl::Matrix::from(vec![])
    };

    //repeat some list of elements some n times
    ($x:tt; $n:expr) => {
        simp_linalg::matrix_impl::Matrix::from(vec![vec!$x; $n])
    };

    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:tt),*) => {
        simp_linalg::matrix_impl::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:tt,)*) => {
        simp_linalg::matrix_impl::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    }
}