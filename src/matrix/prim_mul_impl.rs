use std::ops::Mul;
use crate::matrix::Matrix;

//
//          Floating Point Implementations
//
impl Mul<&Matrix<f32>> for f32 {
    type Output = Matrix<f32>;

    fn mul(self, rhs: &Matrix<f32>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<f64>> for f64 {
    type Output = Matrix<f64>;

    fn mul(self, rhs: &Matrix<f64>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

//
//         Signed Integer Implementations
//
impl Mul<&Matrix<i8>> for i8 {
    type Output = Matrix<i8>;

    fn mul(self, rhs: &Matrix<i8>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<i16>> for i16 {
    type Output = Matrix<i16>;

    fn mul(self, rhs: &Matrix<i16>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<i32>> for i32 {
    type Output = Matrix<i32>;

    fn mul(self, rhs: &Matrix<i32>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<i64>> for i64 {
    type Output = Matrix<i64>;

    fn mul(self, rhs: &Matrix<i64>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<i128>> for i128 {
    type Output = Matrix<i128>;

    fn mul(self, rhs: &Matrix<i128>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<isize>> for isize {
    type Output = Matrix<isize>;

    fn mul(self, rhs: &Matrix<isize>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

//
//          Unsigned Integer Implmentations
//
impl Mul<&Matrix<u8>> for u8 {
    type Output = Matrix<u8>;

    fn mul(self, rhs: &Matrix<u8>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<u16>> for u16 {
    type Output = Matrix<u16>;

    fn mul(self, rhs: &Matrix<u16>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<u32>> for u32 {
    type Output = Matrix<u32>;

    fn mul(self, rhs: &Matrix<u32>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<u64>> for u64 {
    type Output = Matrix<u64>;

    fn mul(self, rhs: &Matrix<u64>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<u128>> for u128 {
    type Output = Matrix<u128>;

    fn mul(self, rhs: &Matrix<u128>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}

impl Mul<&Matrix<usize>> for usize {
    type Output = Matrix<usize>;

    fn mul(self, rhs: &Matrix<usize>) -> Self::Output {
        let mut params = vec![];

        for row_idx in 0..rhs.rows {
            let mut params_row = vec![];
            
            for col_idx in 0..rhs.cols {
                params_row.push(self * rhs.matrix[row_idx][col_idx])
            }
            
            params.push(params_row)
        }

        Matrix::from(params)
    }
}