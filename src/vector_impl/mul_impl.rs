use std::ops::{Mul, AddAssign};
use crate::vector_impl::Vector;
use simp_linalg_proc_macro::vector_dot_prod_impl;
use simp_linalg_proc_macro::vector_scalar_mul_impl;


// Dot Product Implementations

vector_dot_prod_impl!(&Vector<T> &Vector<T>);

vector_dot_prod_impl!(&Vector<T> Vector<T>);

vector_dot_prod_impl!(Vector<T> &Vector<T>);

vector_dot_prod_impl!(Vector<T> Vector<T>);


// Vector * Scalar Implementations

vector_scalar_mul_impl!(&mut Vector<T> T);

vector_scalar_mul_impl!(&Vector<T> T);

vector_scalar_mul_impl!(Vector<T> T);