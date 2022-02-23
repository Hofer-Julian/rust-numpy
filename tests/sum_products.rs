use numpy::{array, dot, einsum, inner, pyarray, Ix2, PyArray1};
use pyo3::Python;

#[test]
fn test_dot() {
    Python::with_gil(|py| {
        let a = pyarray![py, [1, 0], [0, 1]];
        let b = pyarray![py, [4, 1], [2, 2]];
        let c = dot(a, b).unwrap();
        assert_eq!(c.readonly().as_array(), array![[4, 1], [2, 2]]);

        let a = pyarray![py, 1, 2, 3];
        let err = dot::<_, _, _, Ix2>(a, b).unwrap_err();
        assert!(err.to_string().contains("not aligned"), "{}", err);
    });
}

#[test]
fn test_inner() {
    Python::with_gil(|py| {
        let a = pyarray![py, 1, 2, 3];
        let b = pyarray![py, 0, 1, 0];
        let c = inner(a, b).unwrap();
        assert_eq!(c.readonly().as_array(), ndarray::arr0(2));

        let a = pyarray![py, [1, 0], [0, 1]];
        let b = pyarray![py, [4, 1], [2, 2]];
        let c = inner(a, b).unwrap();
        assert_eq!(c.readonly().as_array(), array![[4, 2], [1, 2]]);

        let a = pyarray![py, 1, 2, 3];
        let err = inner::<_, _, _, Ix2>(a, b).unwrap_err();
        assert!(err.to_string().contains("not aligned"), "{}", err);
    });
}

#[test]
fn test_einsum() {
    Python::with_gil(|py| {
        let a = PyArray1::<i32>::arange(py, 0, 25, 1)
            .reshape([5, 5])
            .unwrap();
        let b = pyarray![py, 0, 1, 2, 3, 4];
        let c = pyarray![py, [0, 1, 2], [3, 4, 5]];

        assert_eq!(
            einsum!("ii", a).unwrap().readonly().as_array(),
            ndarray::arr0(60)
        );
        assert_eq!(
            einsum!("ii->i", a).unwrap().readonly().as_array(),
            array![0, 6, 12, 18, 24],
        );
        assert_eq!(
            einsum!("ij->i", a).unwrap().readonly().as_array(),
            array![10, 35, 60, 85, 110],
        );
        assert_eq!(
            einsum!("ji", c).unwrap().readonly().as_array(),
            array![[0, 3], [1, 4], [2, 5]],
        );
        assert_eq!(
            einsum!("ij,j", a, b).unwrap().readonly().as_array(),
            array![30, 80, 130, 180, 230],
        );
    });
}
