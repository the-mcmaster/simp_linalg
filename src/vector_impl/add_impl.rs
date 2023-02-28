use std::ops::Add;
use crate::vector_impl::Vector;
use simp_linalg_proc_macro::vector_add_impl;

vector_add_impl!(&mut Vector<T> &mut Vector<T>);

vector_add_impl!(&mut Vector<T> &Vector<T>);

vector_add_impl!(&mut Vector<T> Vector<T>);

vector_add_impl!(&Vector<T> &mut Vector<T>);

vector_add_impl!(Vector<T> &mut Vector<T>);

vector_add_impl!(&Vector<T> &Vector<T>);

vector_add_impl!(&Vector<T> Vector<T>);

vector_add_impl!(Vector<T> &Vector<T>);

vector_add_impl!(Vector<T> Vector<T>);