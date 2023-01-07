# Simp_LinAlg

A generically defined, light-weight linear algebra library for simple addition and multiplication of vectors and matrices.

Documentation:
- [docs.rs](https://docs.rs/simp_linalg/)

## Usage

Add this to your ``Cargo.toml``
```
[dependencies]
simp_linalg = "0.1.3"
```

## Operator Overloads

Multiplication and addition of vectors and matrices require that their sizes relative to each other are *compatible*.

### Multiplication

- Vectors and Matrices can be multiplied by a scalar. __*Note: the scalar must be on the right side*__
	* ``&Vector<T> * T -> Vector<T>``
	* ``&Matrix<T> * T -> Matrix<T>``
- Matrices can be multiplied by compatible matrices or vectors. 
	* ``&Matrix<T> * &Vector<T> -> Vector<T>``
	* ``&Matrix<T> * &Matrix<T> -> Matrix<T>``
- Vectors can be multiplied by compatible vectors.
	* ``&Vector<T> * &Vector<T> -> T``

### Addition

- Vectors can be added with compatible vectors.
	* ``&Vector<T> + &Vector<T> -> Vector<T>``
- Matrices can be added with compatible matrices.
	* ``&Matrix<T> + &Matrix<T> -> Matrix<T>``

### Fighting the Borrow Checker

<details>
<summary>Technically...</summary>
The aforementioned operator overloaded features utilize borrows frequently. This is only necessary if you intend to continue the lifetime of the variable after its use in the calculation <i>(which is likely often)</i>.

If this is not a requirement, then borrowing is unneeded and the calculation will work as expected.

For example:
```
use simp_linalg::prelude::*;

//Create two vectors
let vector1 = Vector::from(vec![1, 2, 3]);

let vector2 = Vector::from(vec![4, 5, 6]);

// Note: vector2 is dropped after this calculation, but vector1 is not.
let dot_prod: i32 = &vector1 * vector2;
```
</details>

<details>
<summary>Why is it dropped?</summary>
This is due to Rust's <b>move</b> semantics. Rust's standard library type <b>Vec</b> does not implement the <b>Copy</b> trait, thereby moving the value into the multiplication/addition function when called, and consequently dropped when that function goes out of scope. By borrowing the value, the ownership is returned to the original scope and no value is dropped.
</details>

<details>
<summary>Why allow not borrowing?</summary>
This is because it allows for more readable source code.

For instance, suppose you have a vector ``vector_1`` that is transformed by a matrix ``matrix``, whose result will be summed to another vector ``vector_2``.

__In version 0.1.1 *(old)*__:
```
let result: Vector<i32> = &(&matrix * &vector_1) + &vector_2;
```

__In version 0.1.2+__:
```
let result: Vector<i32> = &matrix * &vector_1 + &vector_2;
```

Additionally, with the new feature of multiplying vectors and matrices by scalars, this saves the programmer from another unnecessary borrow. Using the example above, suppose now you want to scale ``vector_2`` before it is summed.

__In version 0.1.1 *(old and hypothetically if scalar multiplication were included)*__:
```
let result: Vector<i32> = &(&matrix * &vector_1) + &(&vector_2 * 4);
```

__In version 0.1.2+__:
```
let result: Vector<i32> = &matrix * &vector_1 + &vector_2 * 4;
```
</details>

## Functionality
Two main functions, **lambda** and **map**, can be used to manipulate the individual values within itself or mesh together two vectors or matrices, in a user-defined way.

### Lambda

Applies a user-defined function to each element of the vector or matrix.

```
use simp_linalg::prelude::*;

// initialization
let vector: Vector<i32> = Vector::from(vec![1, 2, 3]);

let matrix: Matrix<i32> = Matrix::from(vec![vec![1,2],
                                            vec![3,4]]);

// squaring each element inside
let new_vector = vector.lambda(|val| val * val);
let new_matrix = matrix.lambda(|val| val * val);

// tests
assert_eq!(new_vector, Vector::from(vec![1, 4, 9]));
assert_eq!(new_matrix, Matrix::from(vec![vec![1,4],
                                         vec![9,16]]))
```

### Map

Combines two vector or matrices together using a user-defined function.

```
use simp_linalg::prelude::*;

// initialization
// Note: this process is similar to matrix
// Also: the two vectors or matrices must be the same size
let lhs_vector: Vector<f64> = Vector::from(vec![1.5, 2.0]);
let rhs_vector: Vector<f64> = Vector::from(vec![3.0, 4.0]);

// multiplying each corresponding element
let meshed_vector = lhs_vector.map(&rhs_vector, |lhs_val, rhs_val| lhs_val * rhs_val);

// test
assert_eq!(meshed_vector, Vector::from(vec![4.5, 8.0]))
```
