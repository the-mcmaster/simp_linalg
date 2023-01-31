use simp_linalg;

#[test]
fn macro_example() {
    use simp_linalg::prelude::*;

    let v1 = vector![1, 2, 3];
    let v2 = Vector::from(vec![1, 2, 3]);

    let m1 = matrix![
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let m2 = Matrix::from(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ]);

    assert_eq!(v1, v2);
    assert_eq!(m1, m2);
}

#[test]
#[allow(unused_variables)]
fn mul_val_drop() {
    use simp_linalg::prelude::*;

    //Create two vectors
    let vector1 = vector![1, 2, 3];

    let vector2 = vector![4, 5, 6];

    // Note: vector2 is dropped after this calculation, but vector1 is not.
    let dot_prod: i32 = &vector1 * vector2;
}

#[test]
fn lambda_example() {
    use simp_linalg::prelude::*;

    // initialization
    let vector: Vector<i32> = vector![1, 2, 3];

    let matrix: Matrix<i32> = matrix![[1,2],
                                      [3,4]];

    // squaring each element inside
    let new_vector = vector.lambda(|val| val * val);
    let new_matrix = matrix.lambda(|val| val * val);

    // tests
    assert_eq!(new_vector, Vector::from(vec![1, 4, 9]));
    assert_eq!(new_matrix, matrix![[1,4],
                                   [9,16]])
}

#[test]
fn map_example() {
    use simp_linalg::prelude::*;

    // initialization
    // Note: this process is similar to matrix
    // Also: the two vectors or matrices must be the same size
    let lhs_vector: Vector<f64> = vector![1.5, 2.0];
    let rhs_vector: Vector<f64> = vector![3.0, 4.0];

    let lhs_matrix: Matrix<i32> = matrix![
        [1, 2, 3 ],
        [4, 5, 6 ],
        [8, 9, 10]
    ];

    let rhs_matrix: Matrix<i32> = matrix![
        [2, 2, 2],
        [1, 2, 1],
        [0, 3, 0]
    ];

    // multiplying each corresponding element
    let meshed_vector = lhs_vector.map(&rhs_vector, |lhs_val, rhs_val| lhs_val * rhs_val);
    let meshed_matrix = lhs_matrix.map(&rhs_matrix, |lhs_val, rhs_val| lhs_val * rhs_val);

    // test
    assert_eq!(meshed_vector, vector![4.5, 8.0]);
    assert_eq!(meshed_matrix, matrix![
        [2, 4,  6],
        [4, 10, 6],
        [0, 27, 0]
    ]);
}