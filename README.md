# Simp_LinAlg

A generically defined, light-weight linear algebra library for simple addition and multiplication of vectors and matrices.

Documentation:
- [docs.rs](https://docs.rs/simp_linalg/)

## Usage

Add this to your ``Cargo.toml``
```
[dependencies]
simp_linalg = "0.1.2"
```

## Operator Overloads

### Multiplication

- Vectors and Matrices can be multiplied by a scalar. __*Note: the scalar must be on the right side*__
	* ``&Vector<T> * T -> Vector<T>``
	* ``&Matrix<T> * T -> Matrix<T>``
- Matrices can be multiplied by compatible matrices or vectors. 
	* ``&Matrix<T> * &Vector<T> -> Vector<T>``
	* ``&Matrix<T> * &Matrix<T> -> Matrix<T>``
- Vectors can be multiplied by compatible vectors.
	* ``&Vector<T> * &Vector<T> -> Vector<T>``

### Addition

- Vectors can be added with compatible vectors.
	* ``&Vector<T> + &Vector<T> -> Vector<T>``
- Matrices can be added with compatible matrices.
	* ``&Matrix<T> + &Matrix<T> -> Matrix<T>``

### Fighting the Borrow Checker

#### Technically...

The aforementioned operator overloaded features utilize borrows frequently. This is only necessary if you intend to continue the lifetime of the variable after its use in the calculation *(which is likely often)*.

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

#### Why is it dropped?

This is due to Rust's **move** semantics. Rust's standard library type **Vec** does not implement the **Copy** trait, thereby moving the value into the multiplication/addition function when called, and dropped when that function goes out of scope. By borrowing the value, the ownership is returned to the original scope and no value is dropped.

#### Why allow not borrowing?

This is because it allows for more readable source code.

For instance, suppose you have a vector ``vector_1`` that is transformed by a matrix ``matrix``, whose result will be summed to another vector ``vector_2``.

__In version 0.1.1 *(old)*__:
```
let result: i32 = &(&matrix * &vector_1) + &vector_2;
```

__In version 0.1.2__:
```
let result i32 = &matrix * &vector_1 + &vector_2;
```

Additionally, with the new feature of multiplying vector and matrices by scalars, it save the programmer from another unnecessary borrow. Using the example above, suppose now you want to scale ``vector_2`` before it is summed.

__In version 0.1.1 *(old and hypothetically if scalar multiplication were included)*__:
```
let result: Vector<i32> = &(&matrix * &vector_1) + &(&vector_2 * 4);
```

__In version 0.1.2__:
```
let result: Vector<i32> = &matrix * &vector_1 + &vector_2 * 4;
```