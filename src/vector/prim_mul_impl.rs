use std::ops::Mul;
use crate::vector::Vector;

//
//          Floating Point Implementations
//
impl Mul<&Vector<f32>> for f32 {
    type Output = Vector<f32>;

    fn mul(self, rhs: &Vector<f32>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<f64>> for f64 {
    type Output = Vector<f64>;

    fn mul(self, rhs: &Vector<f64>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

//
//         Signed Integer Implementations
//
impl Mul<&Vector<i8>> for i8 {
    type Output = Vector<i8>;

    fn mul(self, rhs: &Vector<i8>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<i16>> for i16 {
    type Output = Vector<i16>;

    fn mul(self, rhs: &Vector<i16>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<i32>> for i32 {
    type Output = Vector<i32>;

    fn mul(self, rhs: &Vector<i32>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<i64>> for i64 {
    type Output = Vector<i64>;

    fn mul(self, rhs: &Vector<i64>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<i128>> for i128 {
    type Output = Vector<i128>;

    fn mul(self, rhs: &Vector<i128>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<isize>> for isize {
    type Output = Vector<isize>;

    fn mul(self, rhs: &Vector<isize>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

//
//          Unsigned Integer Implmentations
//
impl Mul<&Vector<u8>> for u8 {
    type Output = Vector<u8>;

    fn mul(self, rhs: &Vector<u8>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<u16>> for u16 {
    type Output = Vector<u16>;

    fn mul(self, rhs: &Vector<u16>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<u32>> for u32 {
    type Output = Vector<u32>;

    fn mul(self, rhs: &Vector<u32>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<u64>> for u64 {
    type Output = Vector<u64>;

    fn mul(self, rhs: &Vector<u64>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<u128>> for u128 {
    type Output = Vector<u128>;

    fn mul(self, rhs: &Vector<u128>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}

impl Mul<&Vector<usize>> for usize {
    type Output = Vector<usize>;

    fn mul(self, rhs: &Vector<usize>) -> Self::Output {
        let mut params = vec![];
        for item in rhs.list() {
            params.push(self * item)
        }
        Vector::from(params)
    }
}