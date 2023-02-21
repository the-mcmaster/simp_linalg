use crate::matrix_impl::{Matrix, MatrixLambda};

impl<'a, T> MatrixLambda<T> for &'a mut Matrix<T> {
    type Output = &'a mut Matrix<T>;

    fn lambda<F>(self, funct: F) -> Self::Output
    where
        F: Fn(&T) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(&self.matrix[row][col])
            }
        }
        self
    }

    fn lambda_index<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(row, col)
            }
        }
        self
    }

    fn lambda_enumerate<F>(self, funct : F) -> Self::Output
    where
        F: Fn(usize, usize, &T) -> T {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.matrix[row][col] = funct(row, col, &self.matrix[row][col])
            }
        }
        self
    }
}