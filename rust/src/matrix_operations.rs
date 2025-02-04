pub fn shape(m: &[&Vec<f32>]) -> (i32, i32) {
    let i = m.len().try_into().unwrap();
    let j = if i > 0 {
        m[0].len().try_into().unwrap()
    } else {
        0
    };
    (i, j)
}

pub fn get_row(m: &[&Vec<f32>], r: usize) -> Vec<f32> {
    m[r].clone()
}

pub fn get_column(m: &[&Vec<f32>], c: usize) -> Vec<f32> {
    m.iter().map(|v| v[c]).collect()
}

pub fn make_matrix(num_rows: i32, num_cols: i32, entry_fn: fn(i32, i32) -> f32) -> Vec<Vec<f32>> {
    (0..num_rows)
        .map(|i| (0..num_cols).map(|j| entry_fn(i, j)).collect())
        .collect()
}

pub fn identity_matrix(n: i32) -> Vec<Vec<f32>> {
    make_matrix(n, n, |x, y| if x == y { 1.0 } else { 0.0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let c1 = vec![1.0, 2.0, 3.0];
        let c2 = vec![1.0, 2.0, 3.0];
        let m1 = &vec![&c1, &c2];
        assert_eq!(shape(m1), (2, 3));

        let c1 = vec![1.0, 2.0];
        let m2 = &vec![&c1];
        assert_eq!(shape(m2), (1, 2));

        let m3 = &vec![];
        assert_eq!(shape(m3), (0, 0));
    }

    #[test]
    fn test_get_row() {
        let c1 = vec![1.0, 2.0, 3.0];
        let c2 = vec![1.0, 2.0, 3.0];
        let m = &vec![&c1, &c2];
        let result = vec![1.0, 2.0, 3.0];
        assert_eq!(get_row(m, 1), result);
    }

    #[test]
    fn test_get_column() {
        let c1 = vec![1.0, 2.0, 3.0];
        let c2 = vec![1.0, 2.0, 3.0];
        let m = &vec![&c1, &c2];
        let result1 = vec![1.0, 1.0];
        let result2 = vec![2.0, 2.0];
        let result3 = vec![3.0, 3.0];
        assert_eq!(get_column(m, 0), result1);
        assert_eq!(get_column(m, 1), result2);
        assert_eq!(get_column(m, 2), result3);
    }

    #[test]
    fn test_make_matrix() {
        let result = vec![vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 3.0]];
        assert_eq!(make_matrix(2, 3, |x, y| (x + y) as f32), result);
    }

    #[test]
    fn test_identity_matrix() {
        let result = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_eq!(identity_matrix(3), result);
    }
}
