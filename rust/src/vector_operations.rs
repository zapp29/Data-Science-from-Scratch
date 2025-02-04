pub fn add_vectors(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

pub fn subtract_vectors(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

pub fn vector_sum(vectors: &Vec<&Vec<f32>>) -> Vec<f32> {
    let mut result: Vec<f32> = (*(*vectors)[0]).clone();
    for vector in &vectors[1..] {
        result = add_vectors(&result, vector);
    }
    result
}

pub fn scalar_multiply(c: f32, vector: &[f32]) -> Vec<f32> {
    vector.iter().map(|v| v * c).collect()
}

pub fn vector_mean(vectors: &Vec<&Vec<f32>>) -> Vec<f32> {
    scalar_multiply(1.0 / (vectors.len()) as f32, &vector_sum(vectors))
}

pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .collect::<Vec<f32>>()
        .iter()
        .sum()
}

pub fn sum_of_squares(v: &[f32]) -> f32 {
    dot(v, v)
}

pub fn magnitude(v: &[f32]) -> f32 {
    sum_of_squares(v).sqrt()
}

pub fn squared_distance(a: &[f32], b: &[f32]) -> f32 {
    sum_of_squares(&subtract_vectors(a, b))
}

pub fn distance(a: &[f32], b: &[f32]) -> f32 {
    squared_distance(a, b).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vectors() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![1.0, 2.0, 3.0];
        assert_eq!(add_vectors(&v1, &v2), vec![2.0, 4.0, 6.0]);
        assert_eq!(add_vectors(&v2, &v1), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_subtract_vectors() {
        let v1 = vec![2.0, 3.0, 4.0];
        let v2 = vec![1.0, 2.0, 3.0];
        assert_eq!(subtract_vectors(&v1, &v2), vec![1.0, 1.0, 1.0]);
        assert_eq!(subtract_vectors(&v2, &v1), vec![-1.0, -1.0, -1.0]);
    }

    #[test]
    fn test_scalar_multiply() {
        let v1 = vec![2.0, 3.0, 4.0];
        assert_eq!(scalar_multiply(2.0, &v1), vec![4.0, 6.0, 8.0]);
        assert_eq!(scalar_multiply(3.0, &v1), vec![6.0, 9.0, 12.0]);
    }

    #[test]
    fn test_vector_mean() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![3.0, 4.0, 5.0];
        let vecs1 = vec![&v1, &v2];
        let vecs2 = vec![&v2, &v1];
        assert_eq!(vector_mean(&vecs1), vec![2.0, 3.0, 4.0]);
        assert_eq!(vector_mean(&vecs2), vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_dot() {
        let v1 = vec![2.0, 3.0, 4.0];
        let v2 = vec![6.0, 9.0, 12.0];
        assert_eq!(dot(&v1, &v2), 87.0);
        assert_eq!(dot(&v2, &v1), 87.0);
    }

    #[test]
    fn test_sum_of_squares() {
        let v = vec![2.0, 3.0, 4.0];
        assert_eq!(sum_of_squares(&v), 29.0);
    }

    #[test]
    fn test_magnitude() {
        let v = vec![3.0, 4.0];
        assert_eq!(magnitude(&v), 5.0);

        let v = vec![1.0, 2.0, 2.0];
        assert_eq!(magnitude(&v), 3.0);
    }

    #[test]
    fn test_squared_distance() {
        let v1 = vec![3.0, 4.0, 5.0];
        let v2 = vec![1.0, 2.0, 3.0];
        assert_eq!(squared_distance(&v1, &v2), 12.0);
    }

    #[test]
    fn test_distance() {
        let v1 = vec![5.0, 6.0];
        let v2 = vec![2.0, 2.0];
        assert_eq!(distance(&v1, &v2), 5.0);
        assert_eq!(distance(&v2, &v1), 5.0);

        let v1 = vec![5.0, 6.0, 7.0];
        let v2 = vec![4.0, 4.0, 5.0];
        assert_eq!(distance(&v1, &v2), 3.0);
        assert_eq!(distance(&v2, &v1), 3.0);
    }
}
