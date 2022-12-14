# Simp_LinAlg

A simply, light-weight linear algebra library for simple addition and multiplication of mathematical vectors and matrices.

Below is a working demonstration of initializing vectors and matrices, and some overloaded operations.
```
use simp_linalg::prelude::*;

// Initializing matrices
let matrix1 = Matrix::from(vec![vec![1, 2],
								vec![3, 4]]);
                                
let matrix2 = Matrix::from(vec![vec![5, 6],
								vec![7, 8]]);

// Initializing vectors
let vector1 = Vector::from(vec![9, 10]);
let vector2 = Vector::from(vec![11, 12]);

// Use of overloaded '*' operator
let matrix_mul 	= &matrix1 * &matrix2;
let vec_trans 	= &matrix1 * &vector2;
let dot_prod   	= &vector1 * &vector2;

assert_eq!(matrix_mul, Matrix::from(vec![vec![19, 22],
										 vec![43, 50]]));
assert_eq!(vec_trans, Vector::from(vec![35, 81]));
assert_eq!(dot_prod, 219);

// Use of overloaded '+' operator
let matrix_add	= &matrix1 + &matrix2;
let vector_add	= &vector1 + &vector2;

assert_eq!(matrix_add, Matrix::from(vec![vec![6,  8],
										 vec![10, 12]]));

assert_eq!(vector_add, Vector::from(vec![20, 22]));
```